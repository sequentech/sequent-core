// SPDX-FileCopyrightText: 2022 Felix Robles <felix@sequentech.io>
//
// SPDX-License-Identifier: AGPL-3.0-only
#[macro_use]
extern crate quick_error;
extern crate cfg_if;

mod ballot;
pub use ballot::*;
mod encrypt;
mod util;
