// SPDX-FileCopyrightText: 2022 Felix Robles <felix@sequentech.io>
//
// SPDX-License-Identifier: AGPL-3.0-only
use crate::util::*;
use crate::ballot::{Ballot};
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen]
    fn postMessage(s: &str);
}

#[wasm_bindgen]
pub fn message(s: &str) {
    log(s);
    postMessage(s);
}

#[wasm_bindgen]
pub fn test() {
    let res = sum(2,2);
    let formatted_msg = format!("--- wasm::test.rs - {}", res.to_string());
    message(&formatted_msg.to_string())
}

#[wasm_bindgen]
pub fn my_init_function() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn parse_ballot(val: &JsValue) -> String {
    let ballot: Ballot = val.into_serde().unwrap();
    ballot.cad
}