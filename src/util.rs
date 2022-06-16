// SPDX-FileCopyrightText: 2022 Felix Robles <felix@sequentech.io>
//
// SPDX-License-Identifier: AGPL-3.0-only

pub fn sum(a: u16, b: u16) -> u16 {
    a + b
}

#[test]
fn it_works() {
    assert_eq!(sum(2, 2), 4);
}