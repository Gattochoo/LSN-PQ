// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0

use std::{env, fs, path::PathBuf};

use polar_validation::conj_m2_minweight_dp::conj_m2_n3_minweight_probe_report_json;

fn main() {
    let mut max_m = 4usize;
    let mut output = PathBuf::from("experiments/output/928-conj-m2-n3-minweight-sparse-dp.json");
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--max-m" => {
                max_m = args
.next()
.expect("--max-m requires a value")
.parse()
.expect("--max-m must be a usize");
            }
            "--output" => {
                output = PathBuf::from(args.next().expect("--output requires a path"));
            }
            other => panic!("unknown argument {other}; expected --max-m or --output"),
        }
    }

    let json = conj_m2_n3_minweight_probe_report_json(max_m);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).expect("failed to create output directory");
    }
    fs::write(&output, json).expect("failed to write conj:m2 minweight report");
    eprintln!("wrote {}", output.display());
}
