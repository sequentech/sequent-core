// SPDX-FileCopyrightText: 2022 Felix Robles <felix@sequentech.io>
//
// SPDX-License-Identifier: AGPL-3.0-only
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct AuditableBallot {
    pub schema_version: SchemaVersion,
    pub cyphertext: Cyphertext,
    pub replication: Replication,
    pub ballot_hash: String,
    pub config: ElectionConfig,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub enum SchemaVersion {
    ALPHA,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct Cyphertext {
    pub issue_date: String,
    pub choices: Vec<CyphertextChoice>,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct CyphertextChoice {
    pub gr: String,  // alpha
    pub mhr: String, // beta
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct Replication {
    pub issue_date: String,
    pub choices: Vec<ReplicationChoice>,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct ReplicationChoice {
    pub plaintext: String,
    pub randomness: String,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct ElectionConfig {
    pub date: String,
    pub payload: Payload,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct Payload {
    pub id: i64,
    pub configuration: Configuration,
    pub state: String,
    pub start_date: String,
    pub end_date: String,
    pub pks: Vec<Pk>,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct Presentation {
    pub theme: String,
    pub share_text: String,
    pub urls: Vec<String>,
    pub theme_css: String,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct Answer {
    pub category: String,
    pub text: String,
    pub sort_order: i64,
    pub details: String,
    pub urls: Vec<String>,
    pub id: i64,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct Pk {
    pub key_type: KeyType,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub enum KeyType {
    P2048,
}
