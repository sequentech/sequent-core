// SPDX-FileCopyrightText: 2022 Felix Robles <felix@sequentech.io>
//
// SPDX-License-Identifier: AGPL-3.0-only
use crate::ballot::*;
use crate::encrypt::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hash_ballot(val: &JsValue) -> String {
    let ballot: AuditableBallot = val.into_serde().unwrap();
    hash_to(&ballot).unwrap()
}
