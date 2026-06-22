// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0

use std::{env, fs, path::PathBuf};

use polar_validation::conj_pencil_sigma3::conj_pencil_sigma3_report_json;

fn main() {
    let mut output = PathBuf::from("experiments/output/931-conj-pencil-sigma3-search-n6.json");
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--output" {
            output = PathBuf::from(
                args.next()
.expect("--output must be followed by a destination path"),
);
        }
    }

    let json = conj_pencil_sigma3_report_json();
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).expect("failed to create output directory");
    }
    fs::write(&output, json).expect("failed to write conj:pencil sigma3 JSON");
    println!("wrote {}", output.display());
}
