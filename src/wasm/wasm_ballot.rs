// SPDX-FileCopyrightText: 2022 Felix Robles <felix@sequentech.io>
//
// SPDX-License-Identifier: AGPL-3.0-only
use crate::ballot::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const IBALLOT: &'static str = r#"
interface IBallot {
    schema_version: ISchemaVersion;
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

#[wasm_bindgen(typescript_custom_section)]
const SCHEMA_VERSION: &'static str = r#"
enum ISchemaVersion {
    ALPHA = "ALPHA",
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ISchemaVersion")]
    pub type ISchemaVersion;
}

#[wasm_bindgen(typescript_custom_section)]
const ICYPHERTEXT: &'static str = r#"
interface ICyphertext {
    issue_date: string;
    choices: Array<ICyphertextChoice>;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ICyphertext")]
    pub type ICyphertext;
}

#[wasm_bindgen(typescript_custom_section)]
const ICYPHERTEXT_CHOICE: &'static str = r#"
interface ICyphertextChoice {
    gr: string;
    mhr: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ICyphertextChoice")]
    pub type ICyphertextChoice;
}

#[wasm_bindgen(typescript_custom_section)]
const IREPLICATION: &'static str = r#"
interface IReplication {
    issue_date: string;
    choices: Array<IReplicationChoice>;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IReplication")]
    pub type IReplication;
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

#[wasm_bindgen(typescript_custom_section)]
const IPK: &'static str = r#"
interface IPk {
    key_type: IKeyType;
    public_key: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IPk")]
    pub type IPk;
}

#[wasm_bindgen(typescript_custom_section)]
const KEY_TYPE: &'static str = r#"
enum IKeyType {
    P2048 = "P2048",
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IKeyType")]
    pub type IKeyType;
}
