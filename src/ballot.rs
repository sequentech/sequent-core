//use ed25519_dalek::{Digest, Sha512};
//use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Ballot {
    issue_date: String,
}
/*
#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Ballot {
    cyphertext: Cyphertext,
    replication: Replication,
    ballot_hash: String,
    config: ElectionConfig,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Cyphertext {
    issue_date: String,
    choices: Vec<CyphertextChoice>,
    proofs: Vec<Proof>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CyphertextChoice {
    alpha: String,
    beta: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Replication {
    choices: Vec<ReplicationChoice>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ReplicationChoice {
    plaintext: String,
    randomness: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Proof {
    challenge: String,
    commitment: String,
    response: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ElectionConfig {
    date: String,
    payload: Payload,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Payload {
    id: i64,
    configuration: Configuration,
    state: String,
    #[serde(rename = "startDate")]
    start_date: String,
    #[serde(rename = "endDate")]
    end_date: String,
    pks: Vec<Pk>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Configuration {
    layout: String,
    description: String,
    end_date: String,
    title: String,
    start_date: String,
    director: String,
    questions: Vec<Question>,
    authorities: Vec<String>,
    presentation: Presentation,
    id: i64,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Presentation {
    theme: String,
    share_text: String,
    urls: Vec<Option<serde_json::Value>>,
    theme_css: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Question {
    layout: String,
    description: String,
    min: i64,
    max: i64,
    tally_type: String,
    answers: Vec<Answer>,
    num_winners: i64,
    title: String,
    randomize_answer_order: bool,
    answer_total_votes_percentage: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Answer {
    category: Category,
    text: String,
    sort_order: i64,
    details: String,
    urls: Vec<Option<serde_json::Value>>,
    id: i64,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Pk {
    q: String,
    p: String,
    y: String,
    g: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, JsonSchema)]
pub enum Category {
    #[serde(rename = "Candidaturas no agrupadas")]
    CandidaturasNoAgrupadas,
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
*/