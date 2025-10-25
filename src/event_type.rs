use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum EventType {
    Greetings(Greetings),
    SnapshotConfirmed(SnapshotConfirmed),
    TxValid(TxValid),
    TxInvalid(TxInvalid),
    DepositExpired(DepositExpired),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Greetings {
    pub head_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotConfirmed {
    pub head_id: String,
    pub seq: u64,
    pub signatures: Signatures,
    pub snapshot: Snapshot,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxValid {
    pub head_id: String,
    pub seq: u64,
    pub timestamp: String,
    pub transaction_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxInvalid {
    pub head_id: String,
    pub seq: u64,
    pub timestamp: String,
    pub validation_error: ValidationError,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositExpired {
    pub deposit_tx_id: String,
    pub deadline: String,
}
