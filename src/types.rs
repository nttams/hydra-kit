use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub confirmed: Vec<ConfirmedTx>,
    pub head_id: String,
    pub number: u64,
    pub utxo: UtxoSnapshot,
    pub utxo_to_commit: Option<serde_json::Value>,
    pub utxo_to_decommit: Option<serde_json::Value>,
    pub version: u64,
}

pub type UtxoSnapshot = HashMap<String, Utxo>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signatures {
    pub multi_signature: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmedTx {
    pub cbor_hex: String,
    pub description: String,
    pub tx_id: String,
    #[serde(rename = "type")]
    pub tx_type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Utxo {
    #[serde(flatten, default, skip_serializing)]
    pub input: UtxoInput,

    #[serde(flatten)]
    pub output: UtxoOutput,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoInput {
    #[serde(default, skip_serializing)]
    pub output_index: u32,
    #[serde(default, skip_serializing)]
    pub tx_hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoOutput {
    pub address: String,
    pub datum: Option<serde_json::Value>,
    pub inline_datum: Option<serde_json::Value>,
    pub inline_datum_raw: Option<String>,
    pub inline_datumhash: Option<String>,
    pub reference_script: Option<String>,
    pub value: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Value {
    #[serde(flatten)]
    pub assets: HashMap<String, ValueKind>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueKind {
    Lovelace(u64),
    Tokens(HashMap<String, u64>),
}
