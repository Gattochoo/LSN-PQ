// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0

use std::{env, fs, path::PathBuf};

use polar_validation::conj_m2_minweight_dp::conj_m2_lemm2_usable_lpn_sd_report_json;

fn main() {
    let mut max_n = 6usize;
    let mut n4_lagrangian_limit = 128usize;
    let mut n5_lagrangian_limit = 8usize;
    let mut output = PathBuf::from("experiments/output/957-lemm2-diffuse-SD-scale.json");
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
            "--n4-lagrangian-limit" => {
                n4_lagrangian_limit = args
.next()
.expect("--n4-lagrangian-limit must be followed by an integer")
.parse()
.expect("--n4-lagrangian-limit must be an integer");
            }
            "--n5-lagrangian-limit" => {
                n5_lagrangian_limit = args
.next()
.expect("--n5-lagrangian-limit must be followed by an integer")
.parse()
.expect("--n5-lagrangian-limit must be an integer");
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

    let json =
        conj_m2_lemm2_usable_lpn_sd_report_json(max_n, n4_lagrangian_limit, n5_lagrangian_limit);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).expect("failed to create output directory");
    }
    fs::write(&output, json).expect("failed to write lem:m2 usable-LPN SD JSON");
    println!("wrote {}", output.display());
}
