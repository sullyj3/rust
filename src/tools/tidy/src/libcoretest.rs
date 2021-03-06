// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Tidy check to ensure `#[test]` is not used directly inside `libcore`.
//!
//! `#![no_core]` libraries cannot be tested directly due to duplicating lang
//! item. All tests must be written externally in `libcore/tests`.

use std::path::Path;
use std::fs::read_to_string;

pub fn check(path: &Path, bad: &mut bool) {
    let libcore_path = path.join("libcore");
    super::walk(
        &libcore_path,
        &mut |subpath| t!(subpath.strip_prefix(&libcore_path)).starts_with("tests"),
        &mut |subpath| {
            if t!(read_to_string(subpath)).contains("#[test]") {
                tidy_error!(
                    bad,
                    "{} contains #[test]; libcore tests must be placed inside `src/libcore/tests/`",
                    subpath.display()
                );
            }
        },
    );
}
