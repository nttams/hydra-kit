use crate::event_type::*;
use crate::types::*;
use anyhow::{Result, anyhow};
use futures_util::sink::SinkExt;
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::stream::StreamExt;
use hex::decode;
use pallas::crypto::hash::Hash;
use pallas::ledger::addresses::Address;
use pallas::txbuilder::{Input, Output};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::str::FromStr;
use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

pub struct HydraProvider {
    http_client: Client,
    ws_writer: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    cfg: HydraProviderConfig,
    pub listener: Listener,
}

#[derive(Default, Clone)]
pub struct Listener {
    pub on_greeting: Option<fn(Greetings)>,
    pub on_snapshot: Option<fn(SnapshotConfirmed)>,
    pub on_tx_valid: Option<fn(TxValid)>,
    pub on_tx_invalid: Option<fn(TxInvalid)>,
    // TODO: add more events
}

pub struct HydraProviderConfig {
    pub head_addr: String,
}

impl HydraProvider {
    pub fn new(cfg: HydraProviderConfig) -> Self {
        Self {
            cfg: cfg,
            http_client: Client::new(),
            listener: Listener::default(),
            ws_writer: None,
        }
    }

    pub async fn connect_websocket(&mut self) -> Result<()> {
        let ws_stream = match connect_async(format!("ws://{}", self.cfg.head_addr)).await {
            Ok(conn) => conn.0,
            Err(e) => {
                return Err(e.into());
            }
        };

        let (writer, reader) = ws_stream.split();
        self.ws_writer = Some(writer);

        let listener = self.listener.clone();

        tokio::spawn(async move {
            Self::start_listening(listener, reader).await;
        });
        Ok(())
    }

    async fn start_listening(
        listener: Listener,
        mut reader: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) {
        while let Some(msg_result) = reader.next().await {
            let msg = match msg_result {
                Ok(msg) => msg,
                Err(err) => {
                    eprintln!("failed to read web socket message, error: {}", err);
                    break;
                }
            };

            match msg {
                Message::Text(text) => match serde_json::from_str::<EventType>(&text) {
                    Ok(snapshot) => match snapshot {
                        EventType::SnapshotConfirmed(v) => {
                            if let Some(func) = listener.on_snapshot {
                                func(v);
                            };
                        }
                        EventType::TxValid(v) => {
                            if let Some(func) = listener.on_tx_valid {
                                func(v);
                            };
                        }
                        EventType::Greetings(v) => {
                            if let Some(func) = listener.on_greeting {
                                func(v);
                            };
                        }
                        EventType::TxInvalid(v) => {
                            if let Some(func) = listener.on_tx_invalid {
                                func(v);
                            };
                        }
                        _ => {
                            // TODO: handle other events
                        }
                    },
                    Err(err) => {
                        eprintln!("failed to parse, msg: {}, error: {}", text, err)
                    }
                },
                Message::Binary(_)
                | Message::Ping(_)
                | Message::Pong(_)
                | Message::Close(_)
                | Message::Frame(_) => {
                    println!("got non-text message: {:?}", msg);
                }
            }
        }
    }

    pub async fn submit_via_http(&self, tx_hex: &str) -> Result<()> {
        let msg = Self::build_http_msg(&tx_hex);
        let resp = self
            .http_client
            .post(format!("http://{}{}", self.cfg.head_addr, "/transaction"))
            .body(msg.to_string())
            .send()
            .await?;

        if resp.status().as_u16() == 200 || resp.status().as_u16() == 202 {
            return Ok(());
        }
        let code = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_else(|e| e.to_string());

        Err(anyhow!("non-ok code: {}, body: {}", code, body))
    }

    pub async fn submit_via_ws(&mut self, tx_hex: &str) -> Result<()> {
        let writer = self
            .ws_writer
            .as_mut()
            .ok_or(anyhow!("writer is not initialized"))?;

        let msg = tungstenite::Message::text(Self::build_websocket_msg(tx_hex));
        writer.send(msg).await?;
        Ok(())
    }

    async fn send_command(&mut self, tag: &str) -> Result<()> {
        let msg = Message::text(format!(r#"{{"tag":"{}"}}"#, tag));

        let writer = self
            .ws_writer
            .as_mut()
            .ok_or_else(|| anyhow!("WebSocket writer not initialized"))?;

        writer.send(msg).await?;
        Ok(())
    }

    pub async fn init_head(&mut self) -> Result<()> {
        self.send_command("Init").await
    }

    pub async fn close_head(&mut self) -> Result<()> {
        self.send_command("Close").await
    }

    pub async fn get_snapshot(
        &self,
        include_addresses: Option<&HashSet<String>>,
    ) -> Result<UtxoSnapshot> {
        let resp = self
            .http_client
            .get(format!("http://{}{}", self.cfg.head_addr, "/snapshot/utxo"))
            .send()
            .await?;
        let bytes = resp.bytes().await?;
        let map: UtxoSnapshot = serde_json::from_slice(&bytes)?;

        let filtered = map
            .into_iter()
            .filter_map(|(key, mut utxo)| {
                if include_addresses
                    .as_ref()
                    .map_or(false, |a| !a.contains(&utxo.output.address))
                {
                    return None;
                }

                let (tx, idx) = key.split_once('#')?;
                utxo.input.tx_hash = tx.to_owned();
                utxo.input.output_index = idx.parse().ok()?;
                Some((key, utxo))
            })
            .collect();

        Ok(filtered)
    }

    pub async fn get_snapshot_pallas(
        &self,
        include_addresses: Option<&HashSet<String>>,
    ) -> Result<Vec<PallasUTxO>> {
        let map = self.get_snapshot(include_addresses).await?;
        let utxos: Vec<PallasUTxO> = map
            .into_iter()
            .filter_map(|(_key, utxo)| match PallasUTxO::from_hydra_utxo(utxo) {
                Ok(v) => Some(v),
                Err(err) => {
                    eprintln!("failed to convert to pallas utxo, err: {}", err);
                    return None;
                }
            })
            .collect();

        Ok(utxos)
    }

    fn build_http_msg(cbor_hex: &str) -> String {
        let obj: Value = json!({
            "type": "Tx ConwayEra",
            "description": "string",
            "cborHex": cbor_hex,
        });
        obj.to_string()
    }

    fn build_websocket_msg(cbor_hex: &str) -> String {
        let obj: Value = json!({
            "tag": "NewTx",
            "transaction": {
                "type": "Tx ConwayEra",
                "description": "Ledger Cddl Format",
                "cborHex": cbor_hex
            }
        });
        obj.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PallasUTxO {
    pub input: Input,
    pub output: Output,
}

impl PallasUTxO {
    // TODO: need to parse script/datum too
    pub fn from_hydra_utxo(utxo: Utxo) -> Result<Self> {
        let input = Input::new(
            Hash::from_str(&utxo.input.tx_hash)?,
            utxo.input.output_index as u64,
        );

        let mut output = Output::new(Address::from_bech32(&utxo.output.address)?, 0);
        if let Some(datum) = utxo.output.inline_datum_raw {
            let bytes = decode(datum)?;
            output = output.set_inline_datum(bytes);
        }

        for (k, v) in &utxo.output.value.assets {
            match v {
                ValueKind::Lovelace(v) => output.lovelace = *v,
                ValueKind::Tokens(v) => {
                    for (k2, v2) in v {
                        let policy: Hash<28> = Hash::from_str(k)?;
                        let name: Vec<u8> = hex::decode(k2)?;
                        output = output.add_asset(policy, name, *v2)?;
                    }
                }
            };
        }
        Ok(Self { input, output })
    }
}
