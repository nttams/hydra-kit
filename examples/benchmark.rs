use anyhow::Result;
use hydra_kit::helper::*;
use hydra_kit::hydra::*;
use pallas::crypto::key::ed25519::SecretKey;
use pallas::ledger::addresses::Address;
use pallas::txbuilder::{BuildConway, Output, StagingTransaction};
use pallas::wallet::keystore::PrivateKey;
use std::collections::HashSet;
use tokio::time::{Duration, sleep};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let mut provider = HydraProvider::new(HydraProviderConfig {
        head_addr: "localhost:4001".to_string(),
    });
    provider.connect_websocket().await?;
    benchmark_http(&mut provider).await;
    Ok(())
}

// this sends alice's utxo to herself
async fn benchmark_http(provider: &mut HydraProvider) {
    let mut my_set: HashSet<_> = HashSet::new();

    for i in 0..10000 {
        let utxos = provider.get_snapshot_pallas(None).await.expect("error man");
        if utxos.len() <= 0 {
            continue;
        }

        for utxo in utxos {
            let receiver = ALICE_ADDR.to_string();
            let sign_key = ALICE_SK.to_string();
            if utxo.output.address.to_bech32().unwrap() != ALICE_ADDR {
                continue;
            }

            let mut tx = StagingTransaction::new();
            tx = tx.input(utxo.input.clone());
            let mut output = Output::new(
                Address::from_bech32(&receiver).unwrap(),
                utxo.output.lovelace,
            );

            let data_string = "a random immutable data: ".to_owned() + &Uuid::new_v4().to_string();
            let datum = string_to_datum(&data_string).unwrap();

            output = output.set_inline_datum(datum);

            output.assets = utxo.output.assets;
            tx = tx.output(output);

            let str = hex_to_array32(&sign_key).unwrap();
            let private_key = PrivateKey::from(SecretKey::from(str));
            let built_tx = tx.build_conway_raw().unwrap().sign(private_key).unwrap();

            if my_set.contains(&built_tx.tx_hash) {
                continue;
            }
            my_set.insert(built_tx.tx_hash);

            let hex_msg = hex::encode(built_tx.tx_bytes);

            if i % 2 == 0 {
                println!("sending over http");
                match provider.submit_via_http(&hex_msg).await {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("failed to submit tx, err: {}", err)
                    }
                };
            } else {
                println!("sending over websocket");
                match provider.submit_via_ws(&hex_msg).await {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("failed to submit tx, err: {}", err)
                    }
                };
            }

            sleep(Duration::from_millis(2000)).await;
        }
    }
}

const ALICE_ADDR: &str = "addr_test1vqxvmfj9ky733sqhwv38n95hammmlrdwmzjnv7huezh8cwgxrcmtd";
const ALICE_SK: &str = "9fc392611d00553b1e034fbe1a017b894727a055fae498cd8d4b65c315550dbf";
