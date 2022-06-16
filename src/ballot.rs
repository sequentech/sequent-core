use serde::{Deserialize, Serialize};

#[cfg(test)]
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Ballot {
    election_url: String,
    issue_date: String,
    choices: Vec<Choice>,
    proofs: Vec<Proof>,
    ballot_hash: String,
    config: ElectionConfig,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    alpha: String,
    beta: String,
    plaintext: String,
    randomness: String,
}

#[derive(Serialize, Deserialize)]
pub struct Proof {
    challenge: String,
    commitment: String,
    response: String,
}

#[derive(Serialize, Deserialize)]
pub struct ElectionConfig {
    date: String,
    payload: Payload,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Presentation {
    theme: String,
    share_text: String,
    urls: Vec<Option<serde_json::Value>>,
    theme_css: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Answer {
    category: Category,
    text: String,
    sort_order: i64,
    details: String,
    urls: Vec<Option<serde_json::Value>>,
    id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Pk {
    q: String,
    p: String,
    y: String,
    g: String,
}

#[derive(Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "Candidaturas no agrupadas")]
    CandidaturasNoAgrupadas,
}

#[test]
fn parse_ballot() {
    let contents =
        fs::read_to_string("fixtures/ballot.json").expect("Something went wrong reading the file");
    let v: Ballot = serde_json::from_str(&contents).unwrap();
}
