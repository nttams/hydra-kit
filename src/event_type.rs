use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum EventType {
    Greetings(Greetings),
    SnapshotConfirmed(SnapshotConfirmed),
    TxValid(TxValid),
    TxInvalid(TxInvalid),
    DepositExpired(DepositExpired),

    // Other events as generic mapped values
    NetworkConnected { #[serde(flatten)] data: serde_json::Value },
    NetworkDisconnected { #[serde(flatten)] data: serde_json::Value },
    NetworkVersionMismatch { #[serde(flatten)] data: serde_json::Value },
    NetworkClusterIDMismatch { #[serde(flatten)] data: serde_json::Value },
    PeerConnected { #[serde(flatten)] data: serde_json::Value },
    PeerDisconnected { #[serde(flatten)] data: serde_json::Value },
    HeadIsInitializing { #[serde(flatten)] data: serde_json::Value },
    Committed { #[serde(flatten)] data: serde_json::Value },
    HeadIsOpen { #[serde(flatten)] data: serde_json::Value },
    HeadIsClosed { #[serde(flatten)] data: serde_json::Value },
    HeadIsContested { #[serde(flatten)] data: serde_json::Value },
    ReadyToFanout { #[serde(flatten)] data: serde_json::Value },
    HeadIsAborted { #[serde(flatten)] data: serde_json::Value },
    HeadIsFinalized { #[serde(flatten)] data: serde_json::Value },
    InvalidInput { #[serde(flatten)] data: serde_json::Value },
    IgnoredHeadInitializing { #[serde(flatten)] data: serde_json::Value },
    DecommitInvalid { #[serde(flatten)] data: serde_json::Value },
    DecommitRequested { #[serde(flatten)] data: serde_json::Value },
    DecommitApproved { #[serde(flatten)] data: serde_json::Value },
    DecommitFinalized { #[serde(flatten)] data: serde_json::Value },
    CommitRecorded { #[serde(flatten)] data: serde_json::Value },
    DepositActivated { #[serde(flatten)] data: serde_json::Value },
    CommitApproved { #[serde(flatten)] data: serde_json::Value },
    CommitFinalized { #[serde(flatten)] data: serde_json::Value },
    CommitRecovered { #[serde(flatten)] data: serde_json::Value },
    SnapshotSideLoaded { #[serde(flatten)] data: serde_json::Value },
    EventLogRotated { #[serde(flatten)] data: serde_json::Value },
    NodeUnsynced { #[serde(flatten)] data: serde_json::Value },
    NodeSynced { #[serde(flatten)] data: serde_json::Value },
    CommandFailed { #[serde(flatten)] data: serde_json::Value },
    PostTxOnChainFailed { #[serde(flatten)] data: serde_json::Value },
    RejectedInputBecauseUnsynced { #[serde(flatten)] data: serde_json::Value },
    SideLoadSnapshotRejected { #[serde(flatten)] data: serde_json::Value },
    SyncedStatusReport { #[serde(flatten)] data: serde_json::Value },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Greetings {
    pub head_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotConfirmed {
    pub head_id: String,
    pub seq: u64,
    pub signatures: Signatures,
    pub snapshot: Snapshot,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxValid {
    pub head_id: String,
    pub seq: u64,
    pub timestamp: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxInvalid {
    pub head_id: String,
    pub seq: u64,
    pub timestamp: String,
    pub validation_error: ValidationError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositExpired {
    pub deposit_tx_id: String,
    pub deadline: String,
}
