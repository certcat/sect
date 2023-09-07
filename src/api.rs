use base64::{engine::general_purpose::STANDARD as b64, Engine};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

fn b64arrayencode<S: Serializer>(data: &Vec<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error> {
    let encoded: Vec<String> = data.iter().map(|v| b64.encode(v)).collect();
    encoded.serialize(serializer)
}

fn b64arraydecode<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error> {
    let encoded: Vec<String> = Deserialize::deserialize(deserializer)?;
    let decoded: Result<Vec<Vec<u8>>, base64::DecodeError> =
        encoded.iter().map(|s| b64.decode(s)).collect();
    decoded.map_err(|e| serde::de::Error::custom(e))
}

fn b64encode<S: Serializer>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error> {
    b64.encode(data).serialize(serializer)
}

fn b64decode<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    let data: String = Deserialize::deserialize(deserializer)?;
    b64.decode(data).map_err(|e| serde::de::Error::custom(e))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddChainInput {
    #[serde(serialize_with = "b64arrayencode", deserialize_with = "b64arraydecode")]
    pub chain: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddChainOutput {
    pub sct_version: u64,
    #[serde(serialize_with = "b64encode", deserialize_with = "b64decode")]
    pub id: Vec<u8>,
    pub timestamp: u64,
    #[serde(serialize_with = "b64encode", deserialize_with = "b64decode")]
    pub extensions: Vec<u8>,
    #[serde(serialize_with = "b64encode", deserialize_with = "b64decode")]
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddPreChainInput {
    #[serde(serialize_with = "b64arrayencode", deserialize_with = "b64arraydecode")]
    pub chain: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddPreChainOutput {
    pub sct_version: u64,
    #[serde(serialize_with = "b64encode", deserialize_with = "b64decode")]
    pub id: Vec<u8>,
    pub timestamp: u64,
    #[serde(serialize_with = "b64encode", deserialize_with = "b64decode")]
    pub extensions: Vec<u8>,
    #[serde(serialize_with = "b64encode", deserialize_with = "b64decode")]
    pub signature: Vec<u8>,
}
