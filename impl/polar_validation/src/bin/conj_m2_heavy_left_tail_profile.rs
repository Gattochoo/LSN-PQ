// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0

use std::{env, fs, path::PathBuf};

use polar_validation::conj_m2_minweight_dp::conj_m2_heavy_left_tail_profile_c4_report_json;

fn main() {
    let mut max_n = 4usize;
    let mut output = PathBuf::from("experiments/output/939-conj-m2-heavy-n4-thirdpoint.json");
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--max-n" => {
                max_n = args
.next()
.expect("--max-n must be followed by an integer")
.parse()
.expect("--max-n must be an integer");
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

    let json = conj_m2_heavy_left_tail_profile_c4_report_json(max_n);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).expect("failed to create output directory");
    }
    fs::write(&output, json).expect("failed to write conj:m2 heavy left-tail profile JSON");
    println!("wrote {}", output.display());
}
