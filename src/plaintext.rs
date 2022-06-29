use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PlaintextVoteQuestion {
    layout: String,
    description: String,
    min: i64,
    max: i64,
    tally_type: String,
    answers: Vec<PlaintextVoteAnswer>,
    num_winners: i64,
    title: String,
    randomize_answer_order: bool,
    answer_total_votes_percentage: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PlaintextVoteAnswer {
    category: String,
    text: String,
    sort_order: i64,
    details: String,
    urls: Vec<AnswerUrl>,
    selected: i64,
    id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerUrl {
    title: String,
    url: String,
}
