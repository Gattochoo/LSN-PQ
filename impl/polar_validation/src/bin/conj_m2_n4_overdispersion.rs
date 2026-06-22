// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0

use std::{env, fs, path::PathBuf};

use polar_validation::conj_m2_minweight_dp::conj_m2_n4_overdispersion_report_json;

fn main() {
    let mut max_m = 2usize;
    let mut output = PathBuf::from("experiments/output/935-conj-m2-n4-overdispersion-char.json");
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--max-m" => {
                max_m = args
.next()
.expect("--max-m must be followed by an integer")
.parse()
.expect("--max-m must be an integer");
            }
            "--output" => {
                output = PathBuf::from(
                    args.next()
.expect("--output must be followed by a destination path"),
);
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    let json = conj_m2_n4_overdispersion_report_json(max_m);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).expect("failed to create output directory");
    }
    fs::write(&output, json).expect("failed to write conj:m2 n=4 overdispersion JSON");
    println!("wrote {}", output.display());
}
