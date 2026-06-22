// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0

use std::{env, fs, path::PathBuf};

use polar_validation::conj_m2_minweight_dp::{
    conj_m2_samec_sd_scaling_report_json, conj_m2_samec_sd_scaling_single_c_report_json,
};

fn main() {
    let mut max_c = 3usize;
    let mut only_c = None;
    let mut output = PathBuf::from("experiments/output/936-conj-m2-samec-sd-scaling.json");
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--only-c" => {
                only_c = Some(
                    args.next()
.expect("--only-c must be followed by an integer")
.parse()
.expect("--only-c must be an integer"),
);
            }
            "--max-c" => {
                max_c = args
.next()
.expect("--max-c must be followed by an integer")
.parse()
.expect("--max-c must be an integer");
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

    let json = if let Some(c) = only_c {
        conj_m2_samec_sd_scaling_single_c_report_json(c)
    } else {
        conj_m2_samec_sd_scaling_report_json(max_c)
    };
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).expect("failed to create output directory");
    }
    fs::write(&output, json).expect("failed to write conj:m2 same-c SD scaling JSON");
    println!("wrote {}", output.display());
}
