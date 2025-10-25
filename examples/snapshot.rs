use anyhow::Result;
use hydra_kit::hydra::*;

#[tokio::main]
async fn main() -> Result<()> {
    test_get_snapshot().await;
    test_get_pallas_snapshot().await;
    Ok(())
}

async fn test_get_snapshot() {
    let provider = HydraProvider::new(HydraProviderConfig {
        head_addr: "localhost:4001".to_string(),
    });
    let utxo = provider.get_snapshot(None).await.unwrap();

    println!("utxo: {}", serde_json::to_string(&utxo).unwrap());
}

async fn test_get_pallas_snapshot() {
    let provider = HydraProvider::new(HydraProviderConfig {
        head_addr: "localhost:4001".to_string(),
    });
    let utxo = provider.get_snapshot_pallas(None).await.unwrap();

    println!("utxo: {}", serde_json::to_string(&utxo).unwrap());
}
