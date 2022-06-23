use ed25519_dalek::{Digest, Sha512};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Ballot {
    pub cyphertext: Cyphertext,
    pub replication: Replication,
    pub ballot_hash: String,
    pub config: ElectionConfig,
}

#[wasm_bindgen(typescript_custom_section)]
const IBALLOT: &'static str = r#"
interface IBallot {
    cyphertext: ICyphertext;
    replication: IReplication;
    ballot_hash: string;
    config: IElectionConfig;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IBallot")]
    pub type IBallot;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Cyphertext {
    pub issue_date: String,
    pub choices: Vec<CyphertextChoice>,
    pub proofs: Vec<Proof>,
}

#[wasm_bindgen(typescript_custom_section)]
const ICYPHERTEXT: &'static str = r#"
interface ICyphertext {
    issue_date: string;
    choices: Array<ICyphertextChoice>;
    proofs: Array<IProof>;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ICyphertext")]
    pub type ICyphertext;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CyphertextChoice {
    pub alpha: String,
    pub beta: String,
}

#[wasm_bindgen(typescript_custom_section)]
const ICYPHERTEXT_CHOICE: &'static str = r#"
interface ICyphertextChoice {
    alpha: string;
    beta: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ICyphertextChoice")]
    pub type ICyphertextChoice;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Replication {
    pub choices: Vec<ReplicationChoice>,
}

#[wasm_bindgen(typescript_custom_section)]
const IREPLICATION: &'static str = r#"
interface IReplication {
    choices: Array<IReplicationChoice>;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IReplication")]
    pub type IReplication;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ReplicationChoice {
    pub plaintext: String,
    pub randomness: String,
}

#[wasm_bindgen(typescript_custom_section)]
const IREPLICATION_CHOICE: &'static str = r#"
interface IReplicationChoice {
    plaintext: string;
    randomness: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IReplicationChoice")]
    pub type IReplicationChoice;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Proof {
    pub challenge: String,
    pub commitment: String,
    pub response: String,
}

#[wasm_bindgen(typescript_custom_section)]
const IPROOF: &'static str = r#"
interface IProof {
    challenge: string;
    commitment: string;
    response: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IProof")]
    pub type IProof;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ElectionConfig {
    pub date: String,
    pub payload: Payload,
}

#[wasm_bindgen(typescript_custom_section)]
const IELECTION_CONFIG: &'static str = r#"
interface IElectionConfig {
    date: string;
    payload: IPayload;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IElectionConfig")]
    pub type IElectionConfig;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Payload {
    pub id: i64,
    pub configuration: Configuration,
    pub state: String,
    pub start_date: String,
    pub end_date: String,
    pub pks: Vec<Pk>,
}

#[wasm_bindgen(typescript_custom_section)]
const IPAYLOAD: &'static str = r#"
interface IPayload {
    id: number;
    configuration: IConfiguration;
    state: string;
    start_date: string;
    end_date: string;
    pks: Array<IPk>;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IPayload")]
    pub type IPayload;
}

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

#[wasm_bindgen(typescript_custom_section)]
const ICONFIGURATION: &'static str = r#"
interface IConfiguration {
    layout: string;
    description: string;
    end_date: string;
    title: string;
    start_date: string;
    director: string;
    questions: Array<IQuestion>;
    authorities: Array<string>;
    presentation: IPresentation;
    id: number;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IConfiguration")]
    pub type IConfiguration;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Presentation {
    pub theme: String,
    pub share_text: String,
    pub urls: Vec<String>,
    pub theme_css: String,
}

#[wasm_bindgen(typescript_custom_section)]
const IPRESENTATION: &'static str = r#"
interface IPresentation {
    theme: string;
    share_text: string;
    urls: Array<string>;
    theme_css: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IPresentation")]
    pub type IPresentation;
}

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

#[wasm_bindgen(typescript_custom_section)]
const IQUESTION: &'static str = r#"
interface IQuestion {
    layout: string;
    description: string;
    min: number;
    max: number;
    tally_type: string;
    answers: Array<IAnswer>;
    num_winners: number;
    title: string; 
    randomize_answer_order: boolean;
    answer_total_votes_percentage: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IQuestion")]
    pub type IQuestion;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Answer {
    pub category: String,
    pub text: String,
    pub sort_order: i64,
    pub details: String,
    pub urls: Vec<String>,
    pub id: i64,
}

#[wasm_bindgen(typescript_custom_section)]
const IANSWER: &'static str = r#"
interface IAnswer {
    category: string;
    text: string;
    sort_order: number;
    details: string;
    urls: Array<string>;
    id: number;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IAnswer")]
    pub type IAnswer;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Pk {
    pub q: String,
    pub p: String,
    pub y: String,
    pub g: String,
}

#[wasm_bindgen(typescript_custom_section)]
const IPK: &'static str = r#"
interface IPk {
    p: string;
    q: string;
    g: string;
    y: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IPk")]
    pub type IPk;
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
