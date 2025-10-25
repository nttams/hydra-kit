use anyhow::Result;
use hydra_kit::event_type::*;
use hydra_kit::hydra::{HydraProvider, HydraProviderConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let mut provider = HydraProvider::new(HydraProviderConfig {
        head_addr: "localhost:4001".to_string(),
    });
    let cb_snapshot_confirmed = |msg: SnapshotConfirmed| {
        println!("listener got new snapshot, seq: {}", msg.seq);

        for (_k, v) in msg.snapshot.utxo {
            if let Some(inline_datum) = v.output.inline_datum.clone() {
                let bytes_str = inline_datum
                    .get("fields")
                    .and_then(|f| f.get(0))
                    .and_then(|item| item.get("bytes"))
                    .and_then(|b| b.as_str());

                match bytes_str {
                    Some(s) => {
                        let str = hex::decode(s).unwrap();

                        match String::from_utf8(str) {
                            Ok(s) => println!("String: {}", s),
                            Err(e) => println!("Invalid UTF-8: {}", e),
                        }
                    }
                    None => println!("Error: 'bytes' field not found or not a string"),
                }
            }
        }
    };

    let cb_tx_valid = |msg: TxValid| {
        println!("listener got new valid tx, seq: {}", msg.transaction_id);
    };

    provider.listener.on_snapshot = Some(cb_snapshot_confirmed);
    provider.listener.on_tx_valid = Some(cb_tx_valid);

    provider.connect_websocket().await?;

    tokio::signal::ctrl_c().await.unwrap();
    println!("Shutting down...");

    Ok(())
}
