// SPDX-FileCopyrightText: 2022 Felix Robles <felix@sequentech.io>
//
// SPDX-License-Identifier: AGPL-3.0-only
#[macro_use]
extern crate cfg_if;

#[cfg(feature = "wasm")]
pub mod wasm;
