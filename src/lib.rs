// SPDX-FileCopyrightText: 2022 Felix Robles <felix@sequentech.io>
//
// SPDX-License-Identifier: AGPL-3.0-only
#[macro_use]
extern crate quick_error;
extern crate cfg_if;

pub mod ballot;
pub use ballot::*;
pub mod encrypt;
pub mod util;

/// Webassembly API.
#[cfg(feature = "wasm")]
pub mod wasm;
