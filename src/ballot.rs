use ed25519_dalek::{Digest, Sha512};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Ballot {
    pub cyphertext: Cyphertext,
    pub replication: Replication,
    pub ballot_hash: String,
    pub config: ElectionConfig,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Cyphertext {
    pub issue_date: String,
    pub choices: Vec<CyphertextChoice>,
    pub proofs: Vec<Proof>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CyphertextChoice {
    pub alpha: String,
    pub beta: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Replication {
    pub choices: Vec<ReplicationChoice>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ReplicationChoice {
    pub plaintext: String,
    pub randomness: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Proof {
    pub challenge: String,
    pub commitment: String,
    pub response: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ElectionConfig {
    pub date: String,
    pub payload: Payload,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Payload {
    pub id: i64,
    pub configuration: Configuration,
    pub state: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub pks: Vec<Pk>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Configuration {
    pub layout: String,
    pub description: String,
    pub end_date: String,
    pub title: String,
    pub start_date: String,
    pub director: String,
    pub questions: Vec<Question>,
    pub authorities: Vec<String>,
    pub presentation: Presentation,
    pub id: i64,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Presentation {
    pub theme: String,
    pub share_text: String,
    pub urls: Vec<Option<serde_json::Value>>,
    pub theme_css: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Question {
    pub layout: String,
    pub description: String,
    pub min: i64,
    pub max: i64,
    pub tally_type: String,
    pub answers: Vec<Answer>,
    pub num_winners: i64,
    pub title: String,
    pub randomize_answer_order: bool,
    pub answer_total_votes_percentage: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Answer {
    pub category: Category,
    pub text: String,
    pub sort_order: i64,
    pub details: String,
    pub urls: Vec<Option<serde_json::Value>>,
    pub id: i64,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Pk {
    pub q: String,
    pub p: String,
    pub y: String,
    pub g: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub enum Category {
    #[serde(rename = "Candidaturas no agrupadas")]
    pub CandidaturasNoAgrupadas,
}

pub fn hash_to(ballot: &Ballot) -> String {
    let ballot_str = serde_json::to_string(&ballot.cyphertext).unwrap();
    let mut hasher = Sha512::new();
    hasher.update(ballot_str.as_bytes());
    let hashed = hasher.finalize();
    hex::encode(&hashed)
}

#[cfg(test)]
mod tests {
    use crate::ballot::*;
    use std::fs;

    #[test]
    fn parse_ballot() {
        let contents = fs::read_to_string("fixtures/ballot.json")
            .expect("Something went wrong reading the file");
        let ballot: Ballot = serde_json::from_str(&contents).unwrap();
        let sha512_ballot = hash_to(&ballot);
        assert_eq!(&sha512_ballot, &ballot.ballot_hash);
        assert_eq!(
            &sha512_ballot,
            "f2f11a3ad2c03e7dea6d47316e607e4807a18ae064ddf6195fb27a0a180e9168dd9f0f169a352f409e32ee3f89b41a9a21a618a4857788efba13382066aa8df6"
        )
    }
}
