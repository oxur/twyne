// Note that the stucts here are based upon the JSON created from
// this tool:
// * https://github.com/jtschoonhoven/twine-to-json

use serde::{Deserialize, Serialize};
use semver;

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
    tags: String,
    pub text: String,
    pub clean_text: String,
    pub links: Vec<Link>,
    pub hooks: Vec<Hook>,
}

impl Passage {
    pub fn tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self.tags.split(" ").map(|s| s.trim().to_string()).collect();
        tags.sort();
        tags
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub uuid: String,
    pub name: String,
    pub creator: String,
    creator_version: String,
    pub schema_name: String,
    schema_version: String,
    pub created_at_ms: u64,
    pub passages: Vec<Passage>,
}

impl Game {
    pub fn creator_version(&self) -> semver::Version {
        semver::Version::parse(&self.creator_version).unwrap()
    }

    pub fn schema_version(&self) -> semver::Version {
        semver::Version::parse(&self.schema_version).unwrap()
    }
}
