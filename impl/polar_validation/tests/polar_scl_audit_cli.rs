// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use polar_validation::{
    ofa_evidence_audit_posture_json_fields, ofa_evidence_audit_posture_json_lines,
    ofa_evidence_entry_audit_posture_json_fields, ofa_evidence_entry_audit_posture_json_lines,
    ofa_evidence_entry_json_fields, ofa_evidence_entry_paper_claim_dependency_json_field,
    ofa_evidence_entry_runtime_dependency_json_field, ofa_evidence_entry_scope_json_field,
    ofa_evidence_entry_use_in_impl_json_field, ofa_evidence_identity_json_fields,
    ofa_evidence_paper_claim_dependency_json_field, ofa_evidence_runtime_dependency_json_field,
    ofa_evidence_scope_json_field, OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY,
    OFA_EVIDENCE_RUNTIME_DEPENDENCY, OFA_EVIDENCE_SCOPE, OFA_EVIDENCE_USE_IN_IMPL,
    SCL_WORK_SHAPE_OFA_EVIDENCE, SCL_WORK_SHAPE_OFA_EVIDENCE_IDS,
};
use std::{env, fs, process::Command};

#[test]
fn polar_scl_audit_cli_writes_and_checks_exact_json() {
    let bin = env::var("CARGO_BIN_EXE_polar_scl_audit")
.expect("Cargo should expose the polar_scl_audit test binary path");
    let path = env::temp_dir().join(format!("polar_scl_audit_{}.json", std::process::id()));

    let write = Command::new(&bin)
.args(["--output", path.to_str().expect("temp path must be UTF-8")])
.status()
.expect("failed to run polar_scl_audit writer");
    assert!(write.success());

    let json = fs::read_to_string(&path).expect("failed to read generated SCL audit");

    assert_eq!(
        OFA_EVIDENCE_SCOPE,
        "historical OFA Rust research, audit metadata only"
);
    assert!(!OFA_EVIDENCE_RUNTIME_DEPENDENCY);
    assert!(!OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY);
    assert_eq!(
        SCL_WORK_SHAPE_OFA_EVIDENCE_IDS,
        ["OFA-359", "OFA-360", "OFA-388", "OFA-391", "OFA-398", "OFA-399"]
);
    assert_eq!(
        json.matches("\"ofa\": \"OFA-").count(),
        SCL_WORK_SHAPE_OFA_EVIDENCE_IDS.len()
);
    for ofa in SCL_WORK_SHAPE_OFA_EVIDENCE_IDS {
        assert!(json.contains(&format!("\"ofa\": \"{ofa}\"")));
    }
    assert_eq!(
        SCL_WORK_SHAPE_OFA_EVIDENCE.len(),
        SCL_WORK_SHAPE_OFA_EVIDENCE_IDS.len()
);
    for entry in SCL_WORK_SHAPE_OFA_EVIDENCE {
        assert!(SCL_WORK_SHAPE_OFA_EVIDENCE_IDS.contains(&entry.ofa));
        assert!(json.contains(&format!("\"ofa\": \"{}\"", entry.ofa)));
        assert!(json.contains(&format!("\"finding\": \"{}\"", entry.finding)));
        assert!(!entry.finding.is_empty());
        assert!(json.contains(&format!("\"claim_posture\": \"{}\"", entry.claim_posture)));
        assert!(!entry.claim_posture.is_empty());
        for field in ofa_evidence_identity_json_fields(entry) {
            assert!(json.contains(&field));
        }
        for field in ofa_evidence_entry_json_fields(entry) {
            assert!(json.contains(&field));
        }
        assert_eq!(entry.evidence_scope, OFA_EVIDENCE_SCOPE);
        assert_eq!(entry.runtime_dependency, OFA_EVIDENCE_RUNTIME_DEPENDENCY);
        assert_eq!(entry.use_in_impl, OFA_EVIDENCE_USE_IN_IMPL);
        assert_eq!(
            entry.paper_claim_dependency,
            OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY
);
    }
    for field in ofa_evidence_audit_posture_json_fields() {
        assert!(json.contains(field));
    }
    assert!(json.contains(ofa_evidence_audit_posture_json_lines()));
    for field in ofa_evidence_entry_audit_posture_json_fields() {
        assert!(json.contains(field));
        assert_eq!(json.matches(field).count(), 6);
    }
    assert_eq!(
        json.matches(ofa_evidence_entry_audit_posture_json_lines())
.count(),
        6
);
    assert!(json.contains("\"experiment\": \"-polar-scl-workshape-audit\""));
    assert!(json.contains("\"current_verdict\": \"not_constant_time\""));
    assert!(json.contains("\"active_decoder_entrypoints\""));
    assert!(json.contains("\"name\": \"decode_scl_fixed_i64\""));
    assert!(
        json.contains("\"status\": \"active_fixed_i64_reference_entrypoint_not_production_ct\"")
);
    assert!(json.contains("\"production_constant_time_claim\": false"));
    assert!(json.contains("\"research_status\": \"OPEN\""));
    assert!(json.contains("\"lem_m2_residual\": \"correlated Be noise detection gap\""));
    assert!(json.contains("\"dev_research_runtime_dependency\": false"));
    assert!(json.contains("\"dev_research_paper_claim_dependency\": false"));
    assert!(json.contains("audit metadata only; does not change decoder behavior"));
    assert!(json.contains(polar_validation::dev_research_audit_contract_json_lines()));
    for field in polar_validation::dev_research_audit_contract_json_fields() {
        assert!(json.contains(field));
    }
    for flag in polar_validation::dev_research_claim_flag_json_fields() {
        assert!(json.contains(flag));
    }
    let contract_offset = json
.find(polar_validation::dev_research_audit_contract_json_lines())
.expect("generated audit JSON must contain dev research contract block");
    let audited_functions_offset = json
.find("\"audited_functions\"")
.expect("generated audit JSON must contain audited_functions after contract block");
    assert!(contract_offset < audited_functions_offset);
    assert!(json.contains(
        "\"related_public_screens\": [\n    {\n      \"experiment\": \"-lem-m2-marginal-invariant-smoke\",\n      \"source_track\": \"impl/polar_validation Rust\",\n      \"python_research_harness_dependency\": false,\n      \"artifact_scope\": \"public audit artifact only\",\n      \"status\": \"OPEN\",\n      \"production_constant_time_claim\": false,\n      \"reduction_claim\": false,\n      \"security_claim\": false"
));
    assert!(json.contains(&ofa_evidence_scope_json_field()));
    assert!(json.contains(&ofa_evidence_runtime_dependency_json_field()));
    assert!(json.contains(&ofa_evidence_paper_claim_dependency_json_field()));
    assert!(json.contains("\"related_ofa_evidence\""));
    assert!(json.contains("\"ofa\": \"OFA-359\""));
    assert!(json.contains("\"finding\": \"low_degree_cube_sparsity_wall\""));
    assert!(json.contains("\"ofa\": \"OFA-360\""));
    assert!(json.contains("\"finding\": \"full_noise_law_not_preserved_by_nonlocal_sp\""));
    assert!(json.contains("\"ofa\": \"OFA-388\""));
    assert!(json.contains("\"finding\": \"k3_count_guardrail\""));
    assert!(json.contains("\"ofa\": \"OFA-391\""));
    assert!(json.contains("\"finding\": \"corrected_inverse_correlation_bits_about_2n_minus_1\""));
    assert!(json.contains("\"ofa\": \"OFA-398\""));
    assert!(json.contains("\"finding\": \"omega_stress_margin_live_signal_wall_bound\""));
    assert!(json.contains("\"ofa\": \"OFA-399\""));
    assert!(json.contains("\"finding\": \"constant_rate_wall_n7\""));
    assert!(json.contains("\"use_in_impl\": \"audit ledger only; no runtime dependency\""));
    assert!(json.contains(&ofa_evidence_entry_runtime_dependency_json_field()));
    assert!(json.contains(&ofa_evidence_entry_use_in_impl_json_field()));
    assert!(json.contains(&ofa_evidence_entry_scope_json_field()));
    assert!(json.contains(&ofa_evidence_entry_paper_claim_dependency_json_field()));
    assert_eq!(
        json.matches(&ofa_evidence_entry_runtime_dependency_json_field())
.count(),
        6
);
    assert_eq!(
        json.matches(&ofa_evidence_entry_scope_json_field()).count(),
        6
);
    assert_eq!(
        json.matches(&ofa_evidence_entry_paper_claim_dependency_json_field())
.count(),
        6
);
    assert!(json.contains("no reduction claim"));
    assert!(json.contains("no security claim"));
    assert!(json.contains("OPEN = LSN"));

    let check = Command::new(&bin)
.args(["--check", path.to_str().expect("temp path must be UTF-8")])
.status()
.expect("failed to run polar_scl_audit checker");
    assert!(check.success());
}

#[test]
fn scl_audit_cli_ofa_evidence_count_is_fixed_shape() {
    assert_eq!(polar_validation::SCL_WORK_SHAPE_OFA_EVIDENCE_ENTRY_COUNT, 6);
    assert_eq!(
        polar_validation::scl_work_shape_ofa_evidence_entry_count(),
        polar_validation::SCL_WORK_SHAPE_OFA_EVIDENCE_ENTRY_COUNT
);
    assert_eq!(
        polar_validation::scl_work_shape_ofa_evidence_entry_count(),
        polar_validation::SCL_WORK_SHAPE_OFA_EVIDENCE.len()
);
    assert_eq!(
        polar_validation::scl_work_shape_ofa_evidence_entry_count(),
        polar_validation::SCL_WORK_SHAPE_OFA_EVIDENCE_IDS.len()
);
}

#[test]
fn scl_audit_cli_json_records_ofa_evidence_entry_count() {
    let json = polar_validation::scl_work_shape_audit_json();
    assert!(json.contains(polar_validation::scl_work_shape_ofa_evidence_entry_count_json_field()));
    assert_eq!(
        json.matches("\"related_ofa_evidence_entry_count\"").count(),
        1
);
}

#[test]
fn scl_audit_cli_json_records_ofa_evidence_ids() {
    let json = polar_validation::scl_work_shape_audit_json();
    let field = polar_validation::scl_work_shape_ofa_evidence_ids_json_field();
    assert!(json.contains(field));
    assert_eq!(json.matches("\"related_ofa_evidence_ids\"").count(), 1);
    for ofa in polar_validation::SCL_WORK_SHAPE_OFA_EVIDENCE_IDS {
        assert!(field.contains(ofa));
    }
}

#[test]
fn scl_audit_cli_entry_posture_line_block_includes_use_in_impl() {
    let block = polar_validation::ofa_evidence_entry_audit_posture_json_lines();
    assert!(block.contains(polar_validation::ofa_evidence_entry_use_in_impl_json_field()));
    let json = polar_validation::scl_work_shape_audit_json();
    assert_eq!(
        json.matches(block).count(),
        polar_validation::scl_work_shape_ofa_evidence_entry_count()
);
}

#[test]
fn scl_audit_cli_ofa_summary_line_block_records_fixed_fields() {
    let block = polar_validation::scl_work_shape_ofa_evidence_summary_json_lines();
    assert!(block.contains(polar_validation::ofa_evidence_scope_json_field()));
    assert!(block.contains(polar_validation::ofa_evidence_runtime_dependency_json_field()));
    assert!(block.contains(polar_validation::ofa_evidence_paper_claim_dependency_json_field()));
    assert!(block.contains(polar_validation::scl_work_shape_ofa_evidence_entry_count_json_field()));
    assert!(block.contains(polar_validation::scl_work_shape_ofa_evidence_ids_json_field()));
    let json = polar_validation::scl_work_shape_audit_json();
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_ofa_summary_field_list_matches_line_block() {
    let block = polar_validation::scl_work_shape_ofa_evidence_summary_json_lines();
    let fields = polar_validation::scl_work_shape_ofa_evidence_summary_json_fields();
    assert_eq!(fields.len(), 5);
    let json = polar_validation::scl_work_shape_audit_json();
    for field in fields {
        assert!(block.contains(field));
        assert!(json.contains(field));
    }
}

#[test]
fn scl_audit_cli_claim_posture_field_list_matches_json() {
    let fields = polar_validation::scl_work_shape_claim_posture_json_fields();
    assert_eq!(fields.len(), 5);
    let json = polar_validation::scl_work_shape_audit_json();
    for field in fields {
        assert!(json.contains(field));
    }
}

#[test]
fn scl_audit_cli_related_public_screen_field_list_matches_json() {
    let fields = polar_validation::scl_related_public_screen_json_fields();
    assert_eq!(fields.len(), 10);
    let json = polar_validation::scl_work_shape_audit_json();
    assert_eq!(json.matches("\"related_public_screens\"").count(), 1);
    for field in fields {
        assert!(json.contains(field));
    }
}

#[test]
fn scl_audit_cli_related_public_screen_line_block_matches_field_list() {
    let block = polar_validation::scl_related_public_screen_json_lines();
    let fields = polar_validation::scl_related_public_screen_json_fields();
    let json = polar_validation::scl_work_shape_audit_json();
    assert_eq!(fields.len(), 10);
    assert_eq!(json.matches(block).count(), 1);
    for field in fields {
        assert!(block.contains(field));
        assert!(json.contains(field));
    }
}

#[test]
fn scl_audit_cli_claim_posture_line_block_matches_field_list() {
    let block = polar_validation::scl_work_shape_claim_posture_json_lines();
    let fields = polar_validation::scl_work_shape_claim_posture_json_fields();
    let json = polar_validation::scl_work_shape_audit_json();
    assert_eq!(fields.len(), 5);
    assert_eq!(json.matches(block).count(), 1);
    for field in fields {
        assert!(block.contains(field));
        assert!(json.contains(field));
    }
}

#[test]
fn scl_audit_cli_public_artifact_fields_record_rust_only_boundary() {
    let fields = polar_validation::scl_work_shape_public_artifact_json_fields();
    let json = polar_validation::scl_work_shape_audit_json();
    assert_eq!(fields.len(), 6);
    assert!(fields.contains(&"\"source_track\": \"impl/polar_validation Rust\""));
    assert!(fields.contains(&"\"python_research_harness_dependency\": false"));
    assert!(fields.contains(&"\"artifact_scope\": \"public audit artifact only\""));
    for field in fields {
        assert!(json.contains(field));
    }
}

#[test]
fn scl_audit_cli_public_artifact_line_block_matches_field_list() {
    let block = polar_validation::scl_work_shape_public_artifact_json_lines();
    let fields = polar_validation::scl_work_shape_public_artifact_json_fields();
    let json = polar_validation::scl_work_shape_audit_json();
    assert_eq!(fields.len(), 6);
    assert_eq!(json.matches(block).count(), 1);
    for field in fields {
        assert!(block.contains(field));
        assert!(json.contains(field));
    }
}

#[test]
fn scl_audit_cli_related_public_screens_array_line_block_matches_entry() {
    let block = polar_validation::scl_related_public_screens_json_lines();
    let entry = polar_validation::scl_related_public_screen_json_lines();
    let fields = polar_validation::scl_related_public_screen_json_fields();
    let json = polar_validation::scl_work_shape_audit_json();
    assert!(block.contains("\"related_public_screens\": ["));
    assert!(block.contains(entry));
    assert_eq!(json.matches(block).count(), 1);
    for field in fields {
        assert!(block.contains(field));
        assert!(json.contains(field));
    }
}

#[test]
fn scl_audit_cli_audit_boundary_line_block_matches_components() {
    let block = polar_validation::scl_work_shape_audit_boundary_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();
    assert!(block.contains(polar_validation::scl_work_shape_public_artifact_json_lines()));
    assert!(block.contains(polar_validation::scl_work_shape_claim_posture_json_lines()));
    assert!(block.contains(polar_validation::scl_related_public_screens_json_lines()));
    assert!(block.contains(polar_validation::scl_work_shape_ofa_evidence_summary_json_lines()));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_adjudication_field_records_non_claim_boundary() {
    let field = polar_validation::scl_work_shape_adjudication_json_field();
    let json = polar_validation::scl_work_shape_audit_json();
    assert!(field.contains("\"adjudication\":"));
    assert!(field.contains("no reduction claim"));
    assert!(field.contains("no production CT claim"));
    assert!(field.contains("no security claim"));
    assert!(field.contains("OPEN = LSN"));
    assert_eq!(json.matches(field).count(), 1);
}

#[test]
fn scl_audit_cli_required_action_field_records_replacement_boundary() {
    let field = polar_validation::scl_work_shape_required_action_json_field();
    let json = polar_validation::scl_work_shape_audit_json();
    assert!(field.contains("\"required_action\":"));
    assert!(field.contains("fixed-schedule integer decoder plan required"));
    assert!(field.contains("before replacing ct-003"));
    assert_eq!(json.matches(field).count(), 1);
}

#[test]
fn scl_audit_cli_closure_boundary_line_block_matches_required_action_and_adjudication() {
    let block = polar_validation::scl_work_shape_closure_boundary_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.contains(polar_validation::scl_work_shape_required_action_json_field()));
    assert!(block.contains(polar_validation::scl_work_shape_adjudication_json_field()));
    assert!(block.contains("no production CT claim"));
    assert!(block.contains("before replacing ct-003"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_screen_linkage_line_block_matches_related_public_screens() {
    let block = polar_validation::scl_work_shape_screen_linkage_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"related_public_screens\": [\n"));
    assert!(block.contains(polar_validation::scl_related_public_screens_json_lines()));
    assert!(block.contains("\"source_track\": \"impl/polar_validation Rust\""));
    assert!(block.contains("\"python_research_harness_dependency\": false"));
    assert!(block.ends_with("\n  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_related_ofa_evidence_line_block_matches_audit_entries() {
    let block = polar_validation::scl_work_shape_related_ofa_evidence_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"related_ofa_evidence\": [\n"));
    for ofa in [
        "OFA-359", "OFA-360", "OFA-388", "OFA-391", "OFA-398", "OFA-399",
    ] {
        assert!(block.contains(ofa));
    }
    assert!(block.contains("NOT A REDUCTION"));
    assert!(block.contains("NOT A SECURITY CLAIM"));
    assert!(block.contains("\"runtime_dependency\": false"));
    assert!(block.contains("\"paper_claim_dependency\": false"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_audited_functions_line_block_matches_reference_surfaces() {
    let block = polar_validation::scl_work_shape_audited_functions_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"audited_functions\": [\n"));
    for function in [
        "decode_scl",
        "decode_scl_fixed_i64",
        "decode_scl_fast",
        "scl_decode_node",
        "prune_paths",
    ] {
        assert!(block.contains(function));
    }
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_active_decoder_entrypoints_line_block_records_reference_boundary() {
    let block = polar_validation::scl_work_shape_active_decoder_entrypoints_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"active_decoder_entrypoints\": [\n"));
    assert!(block.contains("\"name\": \"decode_scl_fixed_i64\""));
    assert!(block.contains("\"cli_decoder\": \"fixed-i64\""));
    assert!(block.contains("active_fixed_i64_reference_entrypoint_not_production_ct"));
    assert!(block.contains("KAT/BLER-tested validation rail only"));
    assert!(block.contains("before any production constant-time claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_variable_shape_surfaces_line_block_records_audit_only_reasons() {
    let block = polar_validation::scl_work_shape_variable_shape_surfaces_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"variable_shape_surfaces\": [\n"));
    assert!(block.contains("path metric sort in prune_paths"));
    assert!(block.contains("Vec growth and truncate in path pruning"));
    assert!(block.contains("branching on frozen_mask and candidate bit expansion"));
    assert!(block.contains("floating-point path metrics and total_cmp ordering"));
    assert!(block.contains("recursive SCL node composition with data-dependent path contents"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_fixed_schedule_requirements_line_block_records_ct_preconditions() {
    let block = polar_validation::scl_work_shape_fixed_schedule_requirements_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"fixed_schedule_requirements\": [\n"));
    assert!(block.contains("fixed-list array layout for all paths"));
    assert!(block.contains("integer or masked metric updates"));
    assert!(block.contains("data-oblivious top-L selection network"));
    assert!(
        block.contains("no secret-dependent allocation, sorting, truncation, or branch pruning")
);
    assert!(block.contains("generated-code and timing/leakage audit before any production claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_metric_domain_assumptions_line_block_records_integer_boundary() {
    let block = polar_validation::scl_work_shape_metric_domain_assumptions_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"metric_domain_assumptions\": [\n"));
    assert!(block.contains("negative metric deltas are diagnostic-only in source-level rail tests"));
    assert!(block.contains("future active integer SCL rail requires fixed-width non-negative penalties before decoder wiring"));
    assert!(block.contains("forbidden sentinel must remain terminal under parent-metric addition"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_top_l_selection_domain_failure_codes_line_block_records_public_statuses() {
    let block = polar_validation::scl_work_shape_top_l_selection_domain_failure_codes_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"top_l_selection_domain_failure_codes\": [\n"));
    assert!(block.contains("\"code\": 0"));
    assert!(block.contains("\"name\": \"ok\""));
    assert!(block.contains("valid public top-L selection shape"));
    assert!(block.contains("\"code\": 1"));
    assert!(block.contains("\"name\": \"width\""));
    assert!(block.contains("list size must be no larger than selection width"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_path_domain_failure_codes_line_block_records_public_statuses() {
    let block = polar_validation::scl_work_shape_public_path_domain_failure_codes_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_path_domain_failure_codes\": [\n"));
    for name in [
        "ok",
        "empty_schedule",
        "first_child_capacity",
        "repeated_child_capacity",
        "top_l_width",
        "bit_index",
    ] {
        assert!(block.contains(name));
    }
    assert!(block.contains("valid public path-buffer schedule shape"));
    assert!(block.contains("round schedule must contain at least one public round"));
    assert!(block.contains("every public bit index must be inside the path bit width"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_child_write_failure_codes_line_block_records_public_statuses() {
    let block = polar_validation::scl_work_shape_public_child_write_failure_codes_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_child_write_failure_codes\": [\n"));
    for name in ["ok", "parent_slot", "dst_capacity", "bit_index"] {
        assert!(block.contains(name));
    }
    assert!(block.contains("valid public fixed child-write domain"));
    assert!(block.contains("parent slot must be inside the fixed parent buffer"));
    assert!(block.contains("destination child buffer must have room for both children"));
    assert!(block.contains("public bit index must be inside the path bit width"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_integer_schedule_domain_failure_codes_line_block_records_public_statuses() {
    let block = polar_validation::scl_work_shape_integer_schedule_domain_failure_codes_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"integer_schedule_domain_failure_codes\": [\n"));
    for name in ["ok", "hard_bit", "magnitude"] {
        assert!(block.contains(name));
    }
    assert!(block.contains("valid public integer schedule inputs"));
    assert!(block.contains("hard decisions must be public bits"));
    assert!(block.contains("integer metric magnitudes must be non-negative"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_integer_metric_domain_failure_codes_line_block_records_public_statuses() {
    let block = polar_validation::scl_work_shape_integer_metric_domain_failure_codes_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"integer_metric_domain_failure_codes\": [\n"));
    for name in ["ok", "hard_bit", "magnitude"] {
        assert!(block.contains(name));
    }
    assert!(block.contains("valid public integer metric inputs"));
    assert!(block.contains("hard decisions must be public bits"));
    assert!(block.contains("integer metric magnitudes must be non-negative"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_integer_schedule_shape_failure_families_line_block_records_public_statuses() {
    let block =
        polar_validation::scl_work_shape_integer_schedule_shape_failure_families_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"integer_schedule_shape_failure_families\": [\n"));
    for name in ["ok", "integer_domain", "path_domain", "work_shape"] {
        assert!(block.contains(name));
    }
    assert!(block.contains("valid integer schedule-shape preflight"));
    assert!(block.contains("integer hard-bit or metric-magnitude domain failed first"));
    assert!(block.contains("public path-buffer schedule domain failed first"));
    assert!(block.contains("public top-L work-shape envelope failed after domain checks"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_round_schedule_shape_failure_families_line_block_records_public_statuses() {
    let block =
        polar_validation::scl_work_shape_public_round_schedule_shape_failure_families_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_round_schedule_shape_failure_families\": [\n"));
    for name in ["ok", "path_domain", "work_shape"] {
        assert!(block.contains(name));
    }
    assert!(block.contains("valid public schedule-shape preflight"));
    assert!(block.contains("public path-buffer schedule domain failed first"));
    assert!(block.contains("public top-L work-shape envelope failed after path-domain checks"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_non_panicking_wrapper_failure_code_map_line_block_records_public_statuses() {
    let block =
        polar_validation::scl_work_shape_non_panicking_wrapper_failure_code_map_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"non_panicking_wrapper_failure_code_map\": [\n"));
    for wrapper in [
        "try_write_binary_children_from",
        "try_expand_then_compact_one_bit",
        "try_expand_then_compact_two_public_bits",
        "try_expand_then_compact_public_rounds",
        "try_fixed_scl_integer_metric_deltas",
        "try_fixed_scl_integer_round_schedule",
        "try_expand_then_compact_integer_round_schedule",
    ] {
        assert!(block.contains(wrapper));
    }
    assert!(block.contains("\"failure_family\": \"public_child_write_failure_codes\""));
    assert!(block.contains("\"failure_family\": \"public_path_domain_failure_codes\""));
    assert!(block.contains("\"failure_family\": \"integer_metric_domain_failure_codes\""));
    assert!(block.contains("\"failure_family\": \"integer_schedule_domain_failure_codes\""));
    assert!(block.contains("\"integer_status_family\": \"integer_schedule_domain_failure_codes\""));
    assert!(block.contains("FixedSclBinaryChildWriteDomainCheck.child_slots_written"));
    assert!(block.contains("FixedSclPublicRoundScheduleRun.work_counts"));
    assert!(block.contains("FixedSclIntegerRoundScheduleBuild.round_slots_written"));
    assert!(block.contains("FixedSclPathBufferIntegerScheduleRun.work_counts"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_prototype_building_blocks_line_block_records_audit_only_statuses() {
    let block = polar_validation::scl_work_shape_prototype_building_blocks_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"prototype_building_blocks\": [\n"));
    for building_block in [
        "decode_scl_fixed_i64",
        "fixed_schedule_top_l_i64",
        "FixedSclPathBuffer",
        "try_write_binary_children_from",
        "try_expand_then_compact_public_rounds",
        "fixed_scl_public_round_shape_parity_check",
    ] {
        assert!(block.contains(building_block));
    }
    assert!(block.contains("not production constant-time"));
    assert!(block.contains("not wired into decode_scl"));
    assert!(block.contains("generated-code and timing audit pending"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_fixed_work_count_guardrails_line_block_records_public_statuses() {
    let block = polar_validation::scl_work_shape_fixed_work_count_guardrails_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"fixed_work_count_guardrails\": [\n"));
    for surface in [
        "top_l_selection",
        "path_buffer_public_rounds",
        "integer_round_schedule",
    ] {
        assert!(block.contains(surface));
    }
    assert!(block.contains("public list width and list size"));
    assert!(block.contains("public bit indices and path-buffer capacities"));
    assert!(block.contains("public hard-bit and integer metric domains"));
    assert!(block.contains("compare-exchange count only"));
    assert!(block.contains("FixedSclPublicRoundScheduleRun.work_counts"));
    assert!(block.contains("FixedSclIntegerRoundScheduleBuild.round_slots_written"));
    assert!(block.contains("audit-only fixed-work accounting"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_status_selection_guardrails_line_block_records_audit_only_statuses() {
    let block = polar_validation::scl_work_shape_status_selection_guardrails_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"status_selection_guardrails\": [\n"));
    for surface in [
        "child_write_status_selection",
        "path_buffer_status_selection",
        "integer_schedule_status_selection",
    ] {
        assert!(block.contains(surface));
    }
    assert!(block.contains("FixedSclBinaryChildWriteDomainCheck.failure_code"));
    assert!(block.contains("FixedSclPublicRoundScheduleRun.path_domain_check.failure_code"));
    assert!(block.contains("FixedSclIntegerRoundScheduleBuild.domain_check.failure_code"));
    assert!(block.contains("source-level status-selected update only"));
    assert!(block.contains("source-level public status accounting only"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a security claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_domain_guardrails_line_block_records_audit_only_statuses() {
    let block = polar_validation::scl_work_shape_public_domain_guardrails_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_domain_guardrails\": [\n"));
    for surface in [
        "top_l_public_shape",
        "path_buffer_public_shape",
        "integer_metric_public_domain",
    ] {
        assert!(block.contains(surface));
    }
    assert!(block.contains("public list width, list size, and candidate count"));
    assert!(
        block.contains("public bit indices, parent slots, child capacities, and path bit width")
);
    assert!(block.contains("public hard decisions and integer metric magnitudes"));
    assert!(block.contains("public invariant screen only"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_field_provenance_line_block_records_audit_only_statuses() {
    let block = polar_validation::scl_work_shape_public_status_field_provenance_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_field_provenance\": [\n"));
    for field in [
        "FixedSclBinaryChildWriteDomainCheck.failure_code",
        "FixedSclPublicRoundScheduleRun.path_domain_check.failure_code",
        "FixedSclIntegerRoundScheduleBuild.domain_check.failure_code",
        "FixedSclPathBufferIntegerScheduleRun.domain_check.failure_code",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status field"));
    assert!(block.contains("derived from public preflight/domain checks"));
    assert!(block.contains("not secret-dependent branching evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_selector_obligations_line_block_records_mask_select_boundary() {
    let block = polar_validation::scl_work_shape_public_status_selector_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_selector_obligations\": [\n"));
    for selector in [
        "FixedSclBinaryChildWriteDomainCheck.failure_code",
        "FixedSclPublicRoundScheduleRun.path_domain_check.failure_code",
        "FixedSclIntegerRoundScheduleBuild.domain_check.failure_code",
    ] {
        assert!(block.contains(selector));
    }
    assert!(block.contains("public status selector"));
    assert!(block.contains("mask/select obligation"));
    assert!(block.contains("preserve slot shape"));
    assert!(block.contains("not secret-dependent control flow evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_shape_preservation_obligations_line_block_records_fixed_shape_boundary() {
    let block = polar_validation::scl_work_shape_public_shape_preservation_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_shape_preservation_obligations\": [\n"));
    for surface in [
        "top_l_selection_output_shape",
        "path_buffer_round_output_shape",
        "integer_schedule_work_shape",
    ] {
        assert!(block.contains(surface));
    }
    assert!(block.contains("public shape/work-count obligation"));
    assert!(block.contains("preserve output shape"));
    assert!(block.contains("fixed public work shape"));
    assert!(block.contains("audit-only source-level obligation"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_failure_code_wrapper_obligations_line_block_records_non_panicking_boundary()
{
    let block =
        polar_validation::scl_work_shape_public_failure_code_wrapper_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_failure_code_wrapper_obligations\": [\n"));
    for surface in [
        "top_l_selection_status_wrapper",
        "path_buffer_schedule_status_wrapper",
        "integer_schedule_status_wrapper",
    ] {
        assert!(block.contains(surface));
    }
    assert!(block.contains("public failure-code wrapper"));
    assert!(block.contains("return public failure codes"));
    assert!(block.contains("instead of panic-as-control-flow"));
    assert!(block.contains("audit-only wrapper obligation"));
    assert!(block.contains("not a panic-freedom proof"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_preflight_coverage_obligations_line_block_records_public_screen_boundary() {
    let block = polar_validation::scl_work_shape_public_preflight_coverage_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_preflight_coverage_obligations\": [\n"));
    for preflight in [
        "top_l_shape_preflight",
        "path_buffer_capacity_preflight",
        "integer_metric_domain_preflight",
    ] {
        assert!(block.contains(preflight));
    }
    assert!(block.contains("public preflight coverage"));
    assert!(block.contains("before schedule-dependent writes"));
    assert!(block.contains("public invariant screen only"));
    assert!(block.contains("audit-only coverage obligation"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_invalid_input_status_reporting_obligations_line_block_records_audit_status_boundary(
) {
    let block = polar_validation::scl_work_shape_public_invalid_input_status_reporting_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_invalid_input_status_reporting_obligations\": [\n"));
    for report in [
        "top_l_invalid_shape_status",
        "path_buffer_invalid_capacity_status",
        "integer_metric_invalid_domain_status",
    ] {
        assert!(block.contains(report));
    }
    assert!(block.contains("public invalid-input status"));
    assert!(block.contains("record public audit status"));
    assert!(block.contains("without secret-dependent status claims"));
    assert!(block.contains("audit-only reporting obligation"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_rejection_reason_taxonomy_line_block_records_public_reason_boundary() {
    let block = polar_validation::scl_work_shape_public_rejection_reason_taxonomy_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_rejection_reason_taxonomy\": [\n"));
    for reason in [
        "invalid_top_l_shape",
        "invalid_path_buffer_capacity",
        "invalid_integer_metric_domain",
    ] {
        assert!(block.contains(reason));
    }
    assert!(block.contains("public rejection reason"));
    assert!(block.contains("derived from public domain checks"));
    assert!(block.contains("audit-only reason taxonomy"));
    assert!(block.contains("not secret-derived evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_lifecycle_obligations_line_block_records_audit_lifecycle_boundary() {
    let block = polar_validation::scl_work_shape_public_status_lifecycle_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_lifecycle_obligations\": [\n"));
    for phase in [
        "public_preflight_status",
        "public_failure_code_status",
        "public_audit_report_status",
    ] {
        assert!(block.contains(phase));
    }
    assert!(block.contains("public status lifecycle"));
    assert!(block.contains("derived from public checks only"));
    assert!(block.contains("audit-only lifecycle obligation"));
    assert!(block.contains("not secret-state evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_transition_ordering_obligations_line_block_records_public_order_boundary(
) {
    let block =
        polar_validation::scl_work_shape_public_status_transition_ordering_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_transition_ordering_obligations\": [\n"));
    for transition in [
        "preflight_before_failure_code",
        "failure_code_before_mask_select",
        "mask_select_before_audit_report",
    ] {
        assert!(block.contains(transition));
    }
    assert!(block.contains("public status transition ordering"));
    assert!(block.contains("public-before-public transition"));
    assert!(block.contains("audit-only ordering obligation"));
    assert!(block.contains("not secret-order evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_stability_obligations_line_block_records_public_schema_boundary(
) {
    let block =
        polar_validation::scl_work_shape_public_status_schema_stability_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_stability_obligations\": [\n"));
    for field in [
        "public_preflight_status_fields",
        "public_failure_code_fields",
        "public_audit_report_fields",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema stability"));
    assert!(block.contains("stable audit JSON field names"));
    assert!(block.contains("public schema obligation"));
    assert!(block.contains("not secret-schema evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_compatibility_obligations_line_block_records_public_compat_boundary(
) {
    let block =
        polar_validation::scl_work_shape_public_status_schema_compatibility_obligations_json_lines(
);
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_compatibility_obligations\": [\n"));
    for field in [
        "public_status_schema_version_field",
        "append_only_public_status_fields",
        "unknown_public_status_field_handling",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema compatibility"));
    assert!(block.contains("schema version field"));
    assert!(block.contains("append-only compatibility"));
    assert!(block.contains("public compatibility obligation"));
    assert!(block.contains("not secret-compatibility evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_consumer_obligations_line_block_records_public_consumer_boundary(
) {
    let block =
        polar_validation::scl_work_shape_public_status_schema_consumer_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_consumer_obligations\": [\n"));
    for field in [
        "public_only_status_consumers",
        "unknown_public_status_field_policy",
        "compatibility_report_consumers",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema consumer"));
    assert!(block.contains("consume public audit JSON fields only"));
    assert!(block.contains("unknown public fields stay public compatibility data"));
    assert!(block.contains("public consumer obligation"));
    assert!(block.contains("not secret-consumer evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_producer_obligations_line_block_records_public_producer_boundary(
) {
    let block =
        polar_validation::scl_work_shape_public_status_schema_producer_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_producer_obligations\": [\n"));
    for field in [
        "public_only_status_producers",
        "public_status_field_source_tags",
        "producer_side_secret_evidence_exclusion",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema producer"));
    assert!(block.contains("emit public audit JSON fields only"));
    assert!(block.contains("tag public field sources as public metadata"));
    assert!(block.contains("public producer obligation"));
    assert!(block.contains("not secret-producer evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_round_trip_obligations_line_block_records_public_round_trip_boundary(
) {
    let block =
        polar_validation::scl_work_shape_public_status_schema_round_trip_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_round_trip_obligations\": [\n"));
    for field in [
        "public_status_json_generation",
        "public_status_cli_write_check",
        "public_status_consumer_replay",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema round-trip"));
    assert!(block.contains("generated public audit JSON stays public metadata"));
    assert!(block.contains("CLI write/check round-trip remains audit-only"));
    assert!(block.contains("public round-trip obligation"));
    assert!(block.contains("not secret-round-trip evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_drift_obligations_line_block_records_public_drift_boundary() {
    let block =
        polar_validation::scl_work_shape_public_status_schema_drift_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_drift_obligations\": [\n"));
    for field in [
        "public_status_field_addition_drift",
        "public_status_field_rename_drift",
        "public_status_field_order_drift",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema drift"));
    assert!(block.contains("field additions are public audit metadata"));
    assert!(block.contains("field renames require public compatibility notes"));
    assert!(block.contains("public drift obligation"));
    assert!(block.contains("not secret-drift evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_migration_obligations_line_block_records_public_migration_boundary(
) {
    let block =
        polar_validation::scl_work_shape_public_status_schema_migration_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_migration_obligations\": [\n"));
    for field in [
        "public_status_schema_version_migration",
        "public_status_field_compatibility_map",
        "public_status_deprecated_field_notice",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema migration"));
    assert!(block.contains("migration stays public compatibility metadata"));
    assert!(block.contains("compatibility maps are public audit metadata"));
    assert!(block.contains("public migration obligation"));
    assert!(block.contains("not secret-migration evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_rollback_obligations_line_block_records_public_rollback_boundary(
) {
    let block =
        polar_validation::scl_work_shape_public_status_schema_rollback_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_rollback_obligations\": [\n"));
    for field in [
        "public_status_schema_version_rollback",
        "public_status_field_compatibility_rollback",
        "public_status_deprecated_field_restore",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema rollback"));
    assert!(block.contains("rollback stays public compatibility metadata"));
    assert!(block.contains("rollback maps are public audit metadata"));
    assert!(block.contains("public rollback obligation"));
    assert!(block.contains("not secret-rollback evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_deprecation_obligations_line_block_records_public_deprecation_boundary(
) {
    let block =
        polar_validation::scl_work_shape_public_status_schema_deprecation_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_deprecation_obligations\": [\n"));
    for field in [
        "public_status_field_deprecation_notice",
        "public_status_field_replacement_hint",
        "public_status_deprecated_field_retention",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema deprecation"));
    assert!(block.contains("deprecation notices stay public compatibility metadata"));
    assert!(block.contains("replacement hints are public audit metadata"));
    assert!(block.contains("public deprecation obligation"));
    assert!(block.contains("not secret-deprecation evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_removal_obligations_line_block_records_public_removal_boundary(
) {
    let block =
        polar_validation::scl_work_shape_public_status_schema_removal_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_removal_obligations\": [\n"));
    for field in [
        "public_status_field_removal_notice",
        "public_status_field_removal_window",
        "public_status_removed_field_tombstone",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema removal"));
    assert!(block.contains("field removal notices stay public compatibility metadata"));
    assert!(block.contains("removed field tombstones are public audit metadata"));
    assert!(block.contains("public removal obligation"));
    assert!(block.contains("not secret-removal evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_archival_obligations_line_block_records_public_archival_boundary(
) {
    let block =
        polar_validation::scl_work_shape_public_status_schema_archival_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_archival_obligations\": [\n"));
    for field in [
        "public_status_archived_field_catalog",
        "public_status_archived_transition_log",
        "public_status_archive_replay_notice",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema archival"));
    assert!(block.contains("archived field catalogs stay public audit metadata"));
    assert!(block.contains("archive replay notices are public reproducibility metadata"));
    assert!(block.contains("public archival obligation"));
    assert!(block.contains("not secret-archive evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_sunset_obligations_line_block_records_public_sunset_boundary()
{
    let block =
        polar_validation::scl_work_shape_public_status_schema_sunset_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_sunset_obligations\": [\n"));
    for field in [
        "public_status_sunset_notice",
        "public_status_sunset_cutover_epoch",
        "public_status_sunset_archive_reference",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema sunset"));
    assert!(block.contains("sunset notices stay public lifecycle metadata"));
    assert!(block.contains("archive references stay public reproducibility metadata"));
    assert!(block.contains("public sunset obligation"));
    assert!(block.contains("not secret-sunset evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_retention_obligations_line_block_records_public_retention_boundary(
) {
    let block =
        polar_validation::scl_work_shape_public_status_schema_retention_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_retention_obligations\": [\n"));
    for field in [
        "public_status_retention_period",
        "public_status_retention_manifest",
        "public_status_retention_review_marker",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema retention"));
    assert!(block.contains("retention periods stay public lifecycle metadata"));
    assert!(block.contains("retention manifests stay public audit metadata"));
    assert!(block.contains("public retention obligation"));
    assert!(block.contains("not secret-retention evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_lifecycle_order_obligations_line_block_records_public_order_boundary(
) {
    let block = polar_validation::scl_work_shape_public_status_schema_lifecycle_order_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_lifecycle_order_obligations\": [\n"));
    for stage in [
        "public_status_schema_deprecation",
        "public_status_schema_removal",
        "public_status_schema_archival",
        "public_status_schema_sunset",
        "public_status_schema_retention",
    ] {
        assert!(block.contains(stage));
    }
    assert!(block.contains("public status schema lifecycle order"));
    assert!(block.contains("deprecation precedes removal as public lifecycle metadata"));
    assert!(block.contains("retention closes the public lifecycle order"));
    assert!(block.contains("public lifecycle-order obligation"));
    assert!(block.contains("not secret-order evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_lifecycle_shape_obligations_line_block_records_public_shape_boundary(
) {
    let block = polar_validation::scl_work_shape_public_status_schema_lifecycle_shape_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_lifecycle_shape_obligations\": [\n"));
    for field in [
        "public_status_lifecycle_stage_count",
        "public_status_lifecycle_stage_order",
        "public_status_lifecycle_stage_labels",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema lifecycle shape"));
    assert!(block.contains("stage counts stay public fixed-shape metadata"));
    assert!(block.contains("stage order stays public fixed-work metadata"));
    assert!(block.contains("stage labels stay public audit metadata"));
    assert!(block.contains("public lifecycle-shape obligation"));
    assert!(block.contains("not secret-shape evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_lifecycle_registry_obligations_line_block_records_public_registry_boundary(
) {
    let block = polar_validation::scl_work_shape_public_status_schema_lifecycle_registry_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_lifecycle_registry_obligations\": [\n"));
    for field in [
        "public_status_lifecycle_registry_name",
        "public_status_lifecycle_registry_exposure",
        "public_status_lifecycle_registry_consumer_anchor",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema lifecycle registry"));
    assert!(block.contains("registry names stay public audit metadata"));
    assert!(block.contains("registry exposure stays public lifecycle metadata"));
    assert!(block.contains("consumer anchors stay public compatibility metadata"));
    assert!(block.contains("public lifecycle-registry obligation"));
    assert!(block.contains("not secret-registry evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_lifecycle_manifest_obligations_line_block_records_public_manifest_boundary(
) {
    let block = polar_validation::scl_work_shape_public_status_schema_lifecycle_manifest_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_lifecycle_manifest_obligations\": [\n"));
    for field in [
        "public_status_lifecycle_manifest_id",
        "public_status_lifecycle_manifest_stage_list",
        "public_status_lifecycle_manifest_consumer_notice",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema lifecycle manifest"));
    assert!(block.contains("manifest ids stay public audit metadata"));
    assert!(block.contains("manifest stage lists stay public fixed-shape metadata"));
    assert!(block.contains("consumer notices stay public compatibility metadata"));
    assert!(block.contains("public lifecycle-manifest obligation"));
    assert!(block.contains("not secret-manifest evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_lifecycle_replay_obligations_line_block_records_public_replay_boundary(
) {
    let block = polar_validation::scl_work_shape_public_status_schema_lifecycle_replay_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_lifecycle_replay_obligations\": [\n"));
    for field in [
        "public_status_lifecycle_replay_label",
        "public_status_lifecycle_replay_stage_trace",
        "public_status_lifecycle_replay_consumer_check",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema lifecycle replay"));
    assert!(block.contains("replay labels stay public reproducibility metadata"));
    assert!(block.contains("stage traces stay public fixed-shape metadata"));
    assert!(block.contains("consumer checks stay public compatibility metadata"));
    assert!(block.contains("public lifecycle-replay obligation"));
    assert!(block.contains("not secret-replay evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_lifecycle_digest_obligations_line_block_records_public_digest_boundary(
) {
    let block = polar_validation::scl_work_shape_public_status_schema_lifecycle_digest_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_lifecycle_digest_obligations\": [\n"));
    for field in [
        "public_status_lifecycle_digest_label",
        "public_status_lifecycle_digest_stage_checksum",
        "public_status_lifecycle_digest_consumer_comparison",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema lifecycle digest"));
    assert!(block.contains("digest labels stay public reproducibility metadata"));
    assert!(block.contains("stage checksums stay public fixed-shape metadata"));
    assert!(block.contains("consumer comparisons stay public compatibility metadata"));
    assert!(block.contains("public lifecycle-digest obligation"));
    assert!(block.contains("not secret-digest evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_lifecycle_receipt_obligations_line_block_records_public_receipt_boundary(
) {
    let block = polar_validation::scl_work_shape_public_status_schema_lifecycle_receipt_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_lifecycle_receipt_obligations\": [\n"));
    for field in [
        "public_status_lifecycle_receipt_label",
        "public_status_lifecycle_receipt_stage_ack",
        "public_status_lifecycle_receipt_consumer_boundary",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema lifecycle receipt"));
    assert!(block.contains("receipt labels stay public reproducibility metadata"));
    assert!(block.contains("stage acknowledgements stay public fixed-shape metadata"));
    assert!(block.contains("consumer boundaries stay public compatibility metadata"));
    assert!(block.contains("public lifecycle-receipt obligation"));
    assert!(block.contains("not secret-receipt evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_lifecycle_transcript_obligations_line_block_records_public_transcript_boundary(
) {
    let block = polar_validation::scl_work_shape_public_status_schema_lifecycle_transcript_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_lifecycle_transcript_obligations\": [\n"));
    for field in [
        "public_status_lifecycle_transcript_label",
        "public_status_lifecycle_transcript_stage_entry",
        "public_status_lifecycle_transcript_consumer_join",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema lifecycle transcript"));
    assert!(block.contains("transcript labels stay public reproducibility metadata"));
    assert!(block.contains("stage entries stay public fixed-shape metadata"));
    assert!(block.contains("consumer joins stay public compatibility metadata"));
    assert!(block.contains("public lifecycle-transcript obligation"));
    assert!(block.contains("not secret-transcript evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_lifecycle_witness_obligations_line_block_records_public_witness_boundary(
) {
    let block = polar_validation::scl_work_shape_public_status_schema_lifecycle_witness_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_lifecycle_witness_obligations\": [\n"));
    for field in [
        "public_status_lifecycle_witness_label",
        "public_status_lifecycle_witness_stage_marker",
        "public_status_lifecycle_witness_consumer_view",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema lifecycle witness"));
    assert!(block.contains("witness labels stay public reproducibility metadata"));
    assert!(block.contains("stage markers stay public fixed-shape metadata"));
    assert!(block.contains("consumer views stay public compatibility metadata"));
    assert!(block.contains("public lifecycle-witness obligation"));
    assert!(block.contains("not secret-witness evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_lifecycle_audit_trail_obligations_line_block_records_public_trail_boundary(
) {
    let block = polar_validation::scl_work_shape_public_status_schema_lifecycle_audit_trail_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_lifecycle_audit_trail_obligations\": [\n"));
    for field in [
        "public_status_lifecycle_audit_trail_label",
        "public_status_lifecycle_audit_trail_stage_note",
        "public_status_lifecycle_audit_trail_consumer_readback",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema lifecycle audit trail"));
    assert!(block.contains("audit trail labels stay public reproducibility metadata"));
    assert!(block.contains("stage notes stay public fixed-shape metadata"));
    assert!(block.contains("consumer readbacks stay public compatibility metadata"));
    assert!(block.contains("public lifecycle-audit-trail obligation"));
    assert!(block.contains("not secret-audit-trail evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn scl_audit_cli_public_status_schema_lifecycle_checkpoint_obligations_line_block_records_public_checkpoint_boundary(
) {
    let block = polar_validation::scl_work_shape_public_status_schema_lifecycle_checkpoint_obligations_json_lines();
    let json = polar_validation::scl_work_shape_audit_json();

    assert!(block.starts_with("  \"public_status_schema_lifecycle_checkpoint_obligations\": [\n"));
    for field in [
        "public_status_lifecycle_checkpoint_label",
        "public_status_lifecycle_checkpoint_stage_slot",
        "public_status_lifecycle_checkpoint_consumer_resume",
    ] {
        assert!(block.contains(field));
    }
    assert!(block.contains("public status schema lifecycle checkpoint"));
    assert!(block.contains("checkpoint labels stay public reproducibility metadata"));
    assert!(block.contains("stage slots stay public fixed-shape metadata"));
    assert!(block.contains("consumer resumes stay public compatibility metadata"));
    assert!(block.contains("public lifecycle-checkpoint obligation"));
    assert!(block.contains("not secret-checkpoint evidence"));
    assert!(block.contains("not a production constant-time claim"));
    assert!(block.contains("not a reduction"));
    assert!(block.contains("not a security claim"));
    assert!(block.ends_with("  ],\n"));
    assert_eq!(json.matches(block).count(), 1);
}
