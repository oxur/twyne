use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Hook {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub link_text: String,
    pub passage_name: String,
    pub original: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Passage {
    pub id: String,
    pub name: String,
    pub tags: String,
    pub text: String,
    pub clean_text: String,
    pub links: Vec<Link>,
    pub hooks: Vec<Hook>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub uuid: String,
    pub name: String,
    pub creator: String,
    pub creator_version: String,
    pub schema_name: String,
    pub schema_version: String,
    pub created_at_ms: u64,
    pub passages: Vec<Passage>,
}
