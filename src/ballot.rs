use ed25519_dalek::{Digest, Sha512};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Ballot {
    election_url: String,
    issue_date: String,
    choices: Vec<Choice>,
    proofs: Vec<Proof>,
    ballot_hash: String,
    config: ElectionConfig,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Choice {
    alpha: String,
    beta: String,
    plaintext: String,
    randomness: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Proof {
    challenge: String,
    commitment: String,
    response: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ElectionConfig {
    date: String,
    payload: Payload,
}

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

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Presentation {
    theme: String,
    share_text: String,
    urls: Vec<Option<serde_json::Value>>,
    theme_css: String,
}

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

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Answer {
    category: Category,
    text: String,
    sort_order: i64,
    details: String,
    urls: Vec<Option<serde_json::Value>>,
    id: i64,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Pk {
    q: String,
    p: String,
    y: String,
    g: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum Category {
    #[serde(rename = "Candidaturas no agrupadas")]
    CandidaturasNoAgrupadas,
}

pub fn hash_to(ballot: &Ballot) -> String {
    let ballot_str = serde_json::to_string(&ballot).unwrap();
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

        assert_eq!(
            &sha512_ballot,
            "5febcf8df5b2f8ba818e026467af4d851c15379286b72194ea1537c3fe6085239fc13a2631a973ce0bfe48b4fe9569547e8fe5a71cb82c6a30f179e66ac23142"
        )
    }
}
