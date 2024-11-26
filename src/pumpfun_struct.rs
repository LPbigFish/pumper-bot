use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub address: String,
    pub metadata: Metadata,
    pub instructions: Vec<Instruction>,
    pub accounts: Vec<Account2>,
    pub events: Vec<Event>,
    pub types: Vec<Type>,
    pub errors: Vec<Error>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub spec: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    pub name: String,
    pub discriminator: Vec<i64>,
    pub docs: Vec<String>,
    pub accounts: Vec<Account>,
    pub args: Vec<Arg>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub writable: Option<bool>,
    pub pda: Option<Pda>,
    pub signer: Option<bool>,
    pub address: Option<String>,
    #[serde(rename = "isMut")]
    pub is_mut: Option<bool>,
    #[serde(rename = "isSigner")]
    pub is_signer: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pda {
    pub seeds: Vec<Seed>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Seed {
    pub kind: String,
    #[serde(default)]
    pub value: Vec<i64>,
    pub path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arg {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account2 {
    pub name: String,
    pub discriminator: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub discriminator: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: Type2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type2 {
    pub kind: String,
    pub fields: Vec<Field>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub index: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: i64,
    pub name: String,
    pub msg: String,
}