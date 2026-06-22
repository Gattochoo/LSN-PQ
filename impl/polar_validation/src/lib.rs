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

use std::collections::HashSet;

pub mod conj_m2_fresh;
pub mod conj_m2_minweight_dp;
pub mod conj_pencil_sigma3;

macro_rules! ofa_evidence_scope_value {
    () => {
        "historical OFA Rust research, audit metadata only"
    };
}

macro_rules! ofa_evidence_runtime_dependency_value {
    () => {
        "false"
    };
}

macro_rules! ofa_evidence_paper_claim_dependency_value {
    () => {
        "false"
    };
}

macro_rules! ofa_evidence_use_in_impl_value {
    () => {
        "audit ledger only; no runtime dependency"
    };
}

macro_rules! marginal_invariant_ofa_evidence_entry_count_value {
    () => {
        "4"
    };
}

macro_rules! scl_work_shape_ofa_evidence_entry_count_value {
    () => {
        "6"
    };
}

macro_rules! ofa_evidence_scope_json_field {
    () => {
        concat!(
            "\"related_ofa_evidence_scope\": \"",
            ofa_evidence_scope_value!(),
            "\""
)
    };
}

macro_rules! ofa_evidence_runtime_dependency_json_field {
    () => {
        concat!(
            "\"related_ofa_evidence_runtime_dependency\": ",
            ofa_evidence_runtime_dependency_value!()
)
    };
}

macro_rules! ofa_evidence_paper_claim_dependency_json_field {
    () => {
        concat!(
            "\"related_ofa_evidence_paper_claim_dependency\": ",
            ofa_evidence_paper_claim_dependency_value!()
)
    };
}

macro_rules! marginal_invariant_ofa_evidence_entry_count_json_field {
    () => {
        concat!(
            "\"related_ofa_evidence_entry_count\": ",
            marginal_invariant_ofa_evidence_entry_count_value!()
)
    };
}

macro_rules! scl_work_shape_ofa_evidence_entry_count_json_field {
    () => {
        concat!(
            "\"related_ofa_evidence_entry_count\": ",
            scl_work_shape_ofa_evidence_entry_count_value!()
)
    };
}

macro_rules! marginal_invariant_ofa_evidence_ids_json_field {
    () => {
        "\"related_ofa_evidence_ids\": [\"OFA-359\", \"OFA-360\", \"OFA-398\", \"OFA-399\"]"
    };
}

macro_rules! scl_work_shape_ofa_evidence_ids_json_field {
    () => {
        concat!(
            "\"related_ofa_evidence_ids\": ",
            "[\"OFA-359\", \"OFA-360\", \"OFA-388\", \"OFA-391\", \"OFA-398\", \"OFA-399\"]"
)
    };
}

macro_rules! marginal_invariant_ofa_evidence_summary_json_lines {
    () => {
        concat!(
            ofa_evidence_audit_posture_json_lines!(),
            "  ",
            marginal_invariant_ofa_evidence_entry_count_json_field!(),
            ",\n",
            "  ",
            marginal_invariant_ofa_evidence_ids_json_field!(),
            ",\n",
)
    };
}

macro_rules! marginal_invariant_public_artifact_json_lines {
    () => {
        concat!(
            "  \"experiment\": \"-lem-m2-marginal-invariant-smoke\",\n",
            "  \"artifact_scope\": \"public audit artifact only\",\n",
            "  \"source_track\": \"impl/polar_validation Rust\",\n",
            "  \"python_research_harness_dependency\": false,\n",
            "  \"implementation_posture\": \"Rust-side smoke guard; not production constant-time evidence\",\n",
            "  \"checked_invariant\": \"B A equals sampled C\",\n",
            "  \"out_of_scope\": \"statistical indistinguishability, reduction, and security claims\",\n",
            "  \"screen\": \"public marginal-invariant algebra guard for B=section_A(C)+K with K A=0\",\n",
)
    };
}

macro_rules! marginal_invariant_claim_posture_json_lines {
    () => {
        concat!(
            "  \"status\": \"OPEN\",\n",
            "  \"production_constant_time_claim\": false,\n",
            "  \"reduction_claim\": false,\n",
            "  \"security_claim\": false,\n",
            "  \"reduction\": \"NOT A REDUCTION\",\n",
            "  \"security\": \"NOT A SECURITY CLAIM\",\n",
)
    };
}

macro_rules! scl_work_shape_ofa_evidence_summary_json_lines {
    () => {
        concat!(
            ofa_evidence_audit_posture_json_lines!(),
            "  ",
            scl_work_shape_ofa_evidence_entry_count_json_field!(),
            ",\n",
            "  ",
            scl_work_shape_ofa_evidence_ids_json_field!(),
            ",\n",
)
    };
}

macro_rules! scl_work_shape_public_artifact_json_lines {
    () => {
        concat!(
            "  \"experiment\": \"-polar-scl-workshape-audit\",\n",
            "  \"ct_surface\": \"ct-003\",\n",
            "  \"component\": \"impl/polar_validation SCL decoder\",\n",
            "  \"source_track\": \"impl/polar_validation Rust\",\n",
            "  \"python_research_harness_dependency\": false,\n",
            "  \"artifact_scope\": \"public audit artifact only\",\n",
)
    };
}

macro_rules! scl_work_shape_claim_posture_json_lines {
    () => {
        concat!(
            "  \"status\": \"audit boundary only; current decoder is variable-shape reference code\",\n",
            "  \"current_verdict\": \"not_constant_time\",\n",
            "  \"production_constant_time_claim\": false,\n",
            "  \"reduction_claim\": false,\n",
            "  \"security_claim\": false,\n",
)
    };
}

macro_rules! scl_related_public_screen_json_lines {
    () => {
        concat!(
            "    {\n",
            "      \"experiment\": \"-lem-m2-marginal-invariant-smoke\",\n",
            "      \"source_track\": \"impl/polar_validation Rust\",\n",
            "      \"python_research_harness_dependency\": false,\n",
            "      \"artifact_scope\": \"public audit artifact only\",\n",
            "      \"status\": \"OPEN\",\n",
            "      \"production_constant_time_claim\": false,\n",
            "      \"reduction_claim\": false,\n",
            "      \"security_claim\": false,\n",
            "      \"checked_invariant\": \"B A equals sampled C\",\n",
            "      \"out_of_scope\": \"statistical indistinguishability, reduction, and security claims\"\n",
            "    }\n",
)
    };
}

macro_rules! scl_related_public_screens_json_lines {
    () => {
        concat!(
            "  \"related_public_screens\": [\n",
            scl_related_public_screen_json_lines!(),
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_screen_linkage_json_lines {
    () => {
        scl_related_public_screens_json_lines!()
    };
}

macro_rules! marginal_invariant_audit_boundary_json_lines {
    () => {
        concat!(
            marginal_invariant_public_artifact_json_lines!(),
            marginal_invariant_claim_posture_json_lines!(),
            marginal_invariant_ofa_evidence_summary_json_lines!(),
)
    };
}

macro_rules! scl_work_shape_audit_boundary_json_lines {
    () => {
        concat!(
            scl_work_shape_public_artifact_json_lines!(),
            scl_work_shape_claim_posture_json_lines!(),
            scl_work_shape_screen_linkage_json_lines!(),
            scl_work_shape_ofa_evidence_summary_json_lines!(),
)
    };
}

macro_rules! marginal_invariant_adjudication_json_field {
    () => {
        "\"adjudication\": \"engineering smoke artifact only; no reduction claim, no security claim, OPEN = LSN\""
    };
}

macro_rules! scl_work_shape_adjudication_json_field {
    () => {
        "\"adjudication\": \"engineering audit artifact only; no reduction claim, no production CT claim, no security claim, OPEN = LSN\""
    };
}

macro_rules! scl_work_shape_required_action_json_field {
    () => {
        "\"required_action\": \"fixed-schedule integer decoder plan required before replacing ct-003\""
    };
}

macro_rules! scl_work_shape_closure_boundary_json_lines {
    () => {
        concat!(
            "  ",
            scl_work_shape_required_action_json_field!(),
            ",\n",
            "  ",
            scl_work_shape_adjudication_json_field!(),
            "\n",
)
    };
}

macro_rules! scl_work_shape_related_ofa_evidence_json_lines {
    () => {
        concat!(
            "  \"related_ofa_evidence\": [\n",
            "    {\n",
            "      \"ofa\": \"OFA-359\",\n",
            "      \"finding\": \"low_degree_cube_sparsity_wall\",\n",
            "      \"claim_posture\": \"NOT A REDUCTION; NOT A SECURITY CLAIM\",\n",
            ofa_evidence_entry_audit_posture_json_lines!(),
            "    },\n",
            "    {\n",
            "      \"ofa\": \"OFA-360\",\n",
            "      \"finding\": \"full_noise_law_not_preserved_by_nonlocal_sp\",\n",
            "      \"claim_posture\": \"NOT A REDUCTION; NOT A SECURITY CLAIM\",\n",
            ofa_evidence_entry_audit_posture_json_lines!(),
            "    },\n",
            "    {\n",
            "      \"ofa\": \"OFA-388\",\n",
            "      \"finding\": \"k3_count_guardrail\",\n",
            "      \"claim_posture\": \"OPEN; proof-text guardrail, not implementation security\",\n",
            ofa_evidence_entry_audit_posture_json_lines!(),
            "    },\n",
            "    {\n",
            "      \"ofa\": \"OFA-391\",\n",
            "      \"finding\": \"corrected_inverse_correlation_bits_about_2n_minus_1\",\n",
            "      \"claim_posture\": \"OPEN; parameter-table guardrail, not implementation security\",\n",
            ofa_evidence_entry_audit_posture_json_lines!(),
            "    },\n",
            "    {\n",
            "      \"ofa\": \"OFA-398\",\n",
            "      \"finding\": \"omega_stress_margin_live_signal_wall_bound\",\n",
            "      \"claim_posture\": \"live refinement signal; NOT A REDUCTION\",\n",
            ofa_evidence_entry_audit_posture_json_lines!(),
            "    },\n",
            "    {\n",
            "      \"ofa\": \"OFA-399\",\n",
            "      \"finding\": \"constant_rate_wall_n7\",\n",
            "      \"claim_posture\": \"wall-bound evidence; NOT A REDUCTION; NOT A SECURITY CLAIM\",\n",
            ofa_evidence_entry_audit_posture_json_lines!(),
            "    }\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_audited_functions_json_lines {
    () => {
        concat!(
            "  \"audited_functions\": [\n",
            "    \"decode_scl\",\n",
            "    \"decode_scl_fixed_i64\",\n",
            "    \"decode_scl_fast\",\n",
            "    \"scl_decode_node\",\n",
            "    \"prune_paths\"\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_active_decoder_entrypoints_json_lines {
    () => {
        concat!(
            "  \"active_decoder_entrypoints\": [\n",
            "    {\n",
            "      \"name\": \"decode_scl_fixed_i64\",\n",
            "      \"cli_decoder\": \"fixed-i64\",\n",
            "      \"status\": \"active_fixed_i64_reference_entrypoint_not_production_ct\",\n",
            "      \"metric_domain\": \"i64 path metrics with public validation scale\",\n",
            "      \"claim\": \"KAT/BLER-tested validation rail only; generated-code and timing/leakage audit pending before any production constant-time claim\"\n",
            "    }\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_variable_shape_surfaces_json_lines {
    () => {
        concat!(
            "  \"variable_shape_surfaces\": [\n",
            "    \"path metric sort in prune_paths\",\n",
            "    \"Vec growth and truncate in path pruning\",\n",
            "    \"branching on frozen_mask and candidate bit expansion\",\n",
            "    \"floating-point path metrics and total_cmp ordering\",\n",
            "    \"recursive SCL node composition with data-dependent path contents\"\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_fixed_schedule_requirements_json_lines {
    () => {
        concat!(
            "  \"fixed_schedule_requirements\": [\n",
            "    \"fixed-list array layout for all paths\",\n",
            "    \"integer or masked metric updates\",\n",
            "    \"data-oblivious top-L selection network\",\n",
            "    \"no secret-dependent allocation, sorting, truncation, or branch pruning\",\n",
            "    \"generated-code and timing/leakage audit before any production claim\"\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_metric_domain_assumptions_json_lines {
    () => {
        concat!(
            "  \"metric_domain_assumptions\": [\n",
            "    \"negative metric deltas are diagnostic-only in source-level rail tests\",\n",
            "    \"future active integer SCL rail requires fixed-width non-negative penalties before decoder wiring\",\n",
            "    \"forbidden sentinel must remain terminal under parent-metric addition\"\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_top_l_selection_domain_failure_codes_json_lines {
    () => {
        concat!(
            "  \"top_l_selection_domain_failure_codes\": [\n",
            "    {\"code\": 0, \"name\": \"ok\", \"meaning\": \"valid public top-L selection shape\"},\n",
            "    {\"code\": 1, \"name\": \"width\", \"meaning\": \"list size must be no larger than selection width\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_path_domain_failure_codes_json_lines {
    () => {
        concat!(
            "  \"public_path_domain_failure_codes\": [\n",
            "    {\"code\": 0, \"name\": \"ok\", \"meaning\": \"valid public path-buffer schedule shape\"},\n",
            "    {\"code\": 1, \"name\": \"empty_schedule\", \"meaning\": \"round schedule must contain at least one public round\"},\n",
            "    {\"code\": 2, \"name\": \"first_child_capacity\", \"meaning\": \"first child buffer must hold two children per parent slot\"},\n",
            "    {\"code\": 3, \"name\": \"repeated_child_capacity\", \"meaning\": \"repeated child buffer must hold two children per compacted path\"},\n",
            "    {\"code\": 4, \"name\": \"top_l_width\", \"meaning\": \"list size must fit the parent and child selection widths\"},\n",
            "    {\"code\": 5, \"name\": \"bit_index\", \"meaning\": \"every public bit index must be inside the path bit width\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_child_write_failure_codes_json_lines {
    () => {
        concat!(
            "  \"public_child_write_failure_codes\": [\n",
            "    {\"code\": 0, \"name\": \"ok\", \"meaning\": \"valid public fixed child-write domain\"},\n",
            "    {\"code\": 1, \"name\": \"parent_slot\", \"meaning\": \"parent slot must be inside the fixed parent buffer\"},\n",
            "    {\"code\": 2, \"name\": \"dst_capacity\", \"meaning\": \"destination child buffer must have room for both children\"},\n",
            "    {\"code\": 3, \"name\": \"bit_index\", \"meaning\": \"public bit index must be inside the path bit width\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_integer_schedule_domain_failure_codes_json_lines {
    () => {
        concat!(
            "  \"integer_schedule_domain_failure_codes\": [\n",
            "    {\"code\": 0, \"name\": \"ok\", \"meaning\": \"valid public integer schedule inputs\"},\n",
            "    {\"code\": 1, \"name\": \"hard_bit\", \"meaning\": \"hard decisions must be public bits\"},\n",
            "    {\"code\": 2, \"name\": \"magnitude\", \"meaning\": \"integer metric magnitudes must be non-negative\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_integer_metric_domain_failure_codes_json_lines {
    () => {
        concat!(
            "  \"integer_metric_domain_failure_codes\": [\n",
            "    {\"code\": 0, \"name\": \"ok\", \"meaning\": \"valid public integer metric inputs\"},\n",
            "    {\"code\": 1, \"name\": \"hard_bit\", \"meaning\": \"hard decisions must be public bits\"},\n",
            "    {\"code\": 2, \"name\": \"magnitude\", \"meaning\": \"integer metric magnitudes must be non-negative\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_integer_schedule_shape_failure_families_json_lines {
    () => {
        concat!(
            "  \"integer_schedule_shape_failure_families\": [\n",
            "    {\"code\": 0, \"name\": \"ok\", \"meaning\": \"valid integer schedule-shape preflight\"},\n",
            "    {\"code\": 1, \"name\": \"integer_domain\", \"meaning\": \"integer hard-bit or metric-magnitude domain failed first\"},\n",
            "    {\"code\": 2, \"name\": \"path_domain\", \"meaning\": \"public path-buffer schedule domain failed first\"},\n",
            "    {\"code\": 3, \"name\": \"work_shape\", \"meaning\": \"public top-L work-shape envelope failed after domain checks\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_round_schedule_shape_failure_families_json_lines {
    () => {
        concat!(
            "  \"public_round_schedule_shape_failure_families\": [\n",
            "    {\"code\": 0, \"name\": \"ok\", \"meaning\": \"valid public schedule-shape preflight\"},\n",
            "    {\"code\": 1, \"name\": \"path_domain\", \"meaning\": \"public path-buffer schedule domain failed first\"},\n",
            "    {\"code\": 2, \"name\": \"work_shape\", \"meaning\": \"public top-L work-shape envelope failed after path-domain checks\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_non_panicking_wrapper_failure_code_map_json_lines {
    () => {
        concat!(
            "  \"non_panicking_wrapper_failure_code_map\": [\n",
            "    {\"wrapper\": \"try_write_binary_children_from\", \"failure_family\": \"public_child_write_failure_codes\", \"status_field\": \"FixedSclBinaryChildWriteDomainCheck.failure_code\", \"work_count_field\": \"FixedSclBinaryChildWriteDomainCheck.child_slots_written\"},\n",
            "    {\"wrapper\": \"try_expand_then_compact_one_bit\", \"failure_family\": \"public_path_domain_failure_codes\", \"status_field\": \"FixedSclOneBitExpansionRun.path_domain_check.failure_code\", \"work_count_field\": \"FixedSclOneBitExpansionRun.work_counts\"},\n",
            "    {\"wrapper\": \"try_expand_then_compact_two_public_bits\", \"failure_family\": \"public_path_domain_failure_codes\", \"status_field\": \"FixedSclPublicRoundScheduleRun.path_domain_check.failure_code\", \"work_count_field\": \"FixedSclPublicRoundScheduleRun.work_counts\"},\n",
            "    {\"wrapper\": \"try_expand_then_compact_public_rounds\", \"failure_family\": \"public_path_domain_failure_codes\", \"status_field\": \"FixedSclPublicRoundScheduleRun.path_domain_check.failure_code\", \"work_count_field\": \"FixedSclPublicRoundScheduleRun.work_counts\"},\n",
            "    {\"wrapper\": \"try_fixed_scl_integer_metric_deltas\", \"failure_family\": \"integer_metric_domain_failure_codes\", \"status_field\": \"FixedSclIntegerMetricDeltaRun.domain_check.failure_code\", \"work_count_field\": \"none-single-round-delta\"},\n",
            "    {\"wrapper\": \"try_fixed_scl_integer_round_schedule\", \"failure_family\": \"integer_schedule_domain_failure_codes\", \"status_field\": \"FixedSclIntegerRoundScheduleBuild.domain_check.failure_code\", \"work_count_field\": \"FixedSclIntegerRoundScheduleBuild.round_slots_written\"},\n",
            "    {\"wrapper\": \"try_expand_then_compact_integer_round_schedule\", \"failure_family\": \"public_path_domain_failure_codes\", \"path_status_field\": \"FixedSclPathBufferIntegerScheduleRun.path_domain_check.failure_code\", \"integer_status_family\": \"integer_schedule_domain_failure_codes\", \"integer_status_field\": \"FixedSclPathBufferIntegerScheduleRun.domain_check.failure_code\", \"work_count_field\": \"FixedSclPathBufferIntegerScheduleRun.work_counts\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_prototype_building_blocks_json_lines {
    () => {
        concat!(
            "  \"prototype_building_blocks\": [\n",
            "    \"decode_scl_fixed_i64: active fixed-i64 validation entrypoint; not production constant-time; generated-code and timing audit pending\",\n",
            "    \"fixed_schedule_top_l_i64: source-level fixed schedule only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_schedule_top_l_selection_plan: execution-free top-L selection preflight for public width, list size, and compare-exchange count only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"FixedSclPathBuffer: fixed-capacity source-level slot buffer only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_binary_child_write_domain_check: public child-write domain validator for parent slot, destination capacity, and bit index before fixed-slot writes; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_child_write_parity_check: child-write run/preflight parity record that compares the non-panicking wrapper return check with an execution-free child-write preflight only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"try_write_binary_children_from: non-panicking child-write wrapper with status-selected fixed-slot updates; invalid public inputs preserve existing slots; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"write_binary_children_from: integer child expansion into fixed slots only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"expand_then_compact_one_bit: one-bit expand then compact source-level prototype only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"try_expand_then_compact_one_bit: non-panicking one-bit expand then compact wrapper with status-selected outputs on invalid path-domain checks; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_one_bit_run_plan_certificate: one-bit run/preflight plan certificate adapter for comparing source-level run status and work counts with execution-free one-bit preflight; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_one_bit_shape_parity_check: one-bit run/preflight shape parity record that compares run-derived and execution-free one-bit certificates only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"expand_then_compact_two_public_bits: two-round public-bit loop source-level prototype only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"try_expand_then_compact_two_public_bits: non-panicking two-round public-bit helper with status-selected outputs via public-round wrapper; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"two_public_bits_run_shape_certificate: two-public-bits run/preflight shape certificate adapter for comparing wrapper-specific source-level run status and work shape with execution-free public preflight; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"two_public_bits_shape_parity_check: two-public-bits run/preflight shape parity record that reuses the public-round parity surface for the dedicated wrapper; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"FixedSclRound + expand_then_compact_public_rounds: public round schedule source-level prototype only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"try_expand_then_compact_public_rounds: non-panicking multi-round public schedule wrapper with status-selected path buffers and top-L outputs on invalid path-domain checks; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_public_round_run_shape_certificate: public run-shape certificate adapter for comparing source-level run status and work counts with execution-free schedule-shape preflight; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_public_round_shape_parity_check: public run/preflight shape parity record that compares run-derived and execution-free certificates only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_round_schedule_plan: execution-free FixedSclRound schedule preflight that extracts public bit indices and pairs path-domain status with public work counts only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_round_schedule_plan_certificate: FixedSclRound schedule/public preflight certificate adapter that extracts public bit-index shape only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_round_schedule_plan_parity_check: FixedSclRound schedule/public preflight parity record that compares round-derived public shape with explicit public bit-index preflight only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_round_schedule_shape_plan: execution-free FixedSclRound schedule-shape preflight that extracts public bit indices and pairs path-domain status with top-L work-shape envelopes only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_round_schedule_shape_plan_certificate: FixedSclRound schedule/public shape certificate adapter that extracts public bit-index shape and top-L envelopes only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_round_schedule_shape_parity_check: FixedSclRound schedule/public shape parity record that compares round-derived public top-L envelopes with explicit public bit-index shape preflight only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_public_round_schedule_plan: execution-free public schedule preflight that pairs path-domain status with public work counts only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_public_round_schedule_shape_plan: execution-free public schedule shape certificate that pairs path-domain status with first/repeated top-L preflights and public work counts only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_public_round_schedule_shape_failure_family: public schedule-shape failure-family classifier over public path/top-L envelope statuses only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_public_round_work_counts: public work-count audit for fixed SCL schedule parameters only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_public_round_work_counts_with_capacities: public work-count audit with separate first and repeated child capacities only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_public_round_work_shape_plan: execution-free public round work-shape plan that pairs first/repeated top-L preflights with public work counts only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_metric_domain_check: single-round integer metric domain validator for public hard-bit and non-negative magnitude inputs only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_metric_deltas: integer metric delta audit for hard-bit penalties and frozen branch forbidding only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"try_fixed_scl_integer_metric_deltas: non-panicking integer metric delta wrapper that returns terminal sentinels on invalid public metric inputs; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_round_schedule: public integer round schedule audit from hard-bit penalties into FixedSclRound arrays only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_schedule_domain_check: active integer schedule domain validator for hard-bit and non-negative magnitude inputs only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_round_schedule_build_plan: execution-free integer schedule-build preflight that pairs integer status with public round-slot write count only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_round_build_certificate: integer schedule-build run certificate adapter for comparing returned domain status and round-slot write count with execution-free build preflight; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_round_build_parity_check: integer schedule-build run/preflight parity record that compares schedule-builder status and public round-slot write count only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_round_schedule_plan: execution-free integer schedule preflight that pairs integer status, path-domain status, and public work counts only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_round_schedule_shape_plan: execution-free integer schedule-shape preflight that pairs integer status, path-domain status, and public top-L work-shape envelopes only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_round_run_plan_certificate: integer run/preflight plan certificate adapter for comparing source-level run status and work counts with execution-free integer preflight; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_round_run_shape_certificate: integer run/preflight shape certificate adapter for comparing source-level run status and top-L envelopes with execution-free integer shape preflight; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_shape_parity_check: integer run/preflight shape parity record that compares run-derived and execution-free integer certificates only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_schedule_shape_parity_check: integer schedule/run shape parity record that compares run-derived and execution-free integer top-L envelope certificates only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_integer_schedule_shape_failure_family: integer schedule-shape failure-family classifier over public integer/path/top-L envelope statuses only; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"try_fixed_scl_integer_round_schedule: non-panicking integer schedule builder that returns domain-check status before FixedSclRound arrays; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"fixed_scl_path_buffer_schedule_domain_check: public path-buffer shape validator for capacities and bit indices before expansion; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"try_expand_then_compact_integer_round_schedule: non-panicking path-buffer schedule wrapper with status-selected zero outputs on invalid integer-domain checks; not wired into decode_scl; generated-code and timing audit pending\",\n",
            "    \"expand_then_compact_integer_round_schedule: integer schedule source-level loop over fixed path buffers only; not wired into decode_scl; generated-code and timing audit pending\"\n",
            "  ],\n",
            "  \"public_work_count_examples\": [\n",
            "    {\n",
            "      \"label\": \"parent_capacity_2_child_capacity_4_list_size_2_rounds_3\",\n",
            "      \"parent_capacity\": 2,\n",
            "      \"child_capacity\": 4,\n",
            "      \"list_size\": 2,\n",
            "      \"rounds\": 3,\n",
            "      \"top_l_compare_exchanges\": 18,\n",
            "      \"child_slots_written\": 12,\n",
            "      \"compacted_slots_written\": 6,\n",
            "      \"source\": \"fixed_scl_public_round_work_counts(2, 4, 2, 3)\"\n",
            "    },\n",
            "    {\n",
            "      \"label\": \"parent_capacity_3_first_child_6_repeated_child_4_list_size_2_rounds_3\",\n",
            "      \"parent_capacity\": 3,\n",
            "      \"first_child_capacity\": 6,\n",
            "      \"repeated_child_capacity\": 4,\n",
            "      \"list_size\": 2,\n",
            "      \"rounds\": 3,\n",
            "      \"top_l_compare_exchanges\": 27,\n",
            "      \"child_slots_written\": 14,\n",
            "      \"compacted_slots_written\": 6,\n",
            "      \"source\": \"fixed_scl_public_round_work_counts_with_capacities(3, 6, 4, 2, 3)\"\n",
            "    },\n",
            "    {\n",
            "      \"label\": \"zero_rounds_no_expansion_work\",\n",
            "      \"parent_capacity\": 3,\n",
            "      \"first_child_capacity\": 6,\n",
            "      \"repeated_child_capacity\": 4,\n",
            "      \"list_size\": 2,\n",
            "      \"rounds\": 0,\n",
            "      \"top_l_compare_exchanges\": 0,\n",
            "      \"child_slots_written\": 0,\n",
            "      \"compacted_slots_written\": 0,\n",
            "      \"source\": \"fixed_scl_public_round_work_counts_with_capacities(3, 6, 4, 2, 0)\"\n",
            "    }\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_fixed_work_count_guardrails_json_lines {
    () => {
        concat!(
            "  \"fixed_work_count_guardrails\": [\n",
            "    {\"surface\": \"top_l_selection\", \"public_domain\": \"public list width and list size\", \"work_count_source\": \"fixed_schedule_top_l_selection_plan compare-exchange count only\", \"posture\": \"audit-only fixed-work accounting; not a production constant-time claim\"},\n",
            "    {\"surface\": \"path_buffer_public_rounds\", \"public_domain\": \"public bit indices and path-buffer capacities\", \"work_count_source\": \"FixedSclPublicRoundScheduleRun.work_counts\", \"posture\": \"audit-only fixed-work accounting; not a production constant-time claim\"},\n",
            "    {\"surface\": \"integer_round_schedule\", \"public_domain\": \"public hard-bit and integer metric domains\", \"work_count_source\": \"FixedSclIntegerRoundScheduleBuild.round_slots_written\", \"posture\": \"audit-only fixed-work accounting; not a production constant-time claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_status_selection_guardrails_json_lines {
    () => {
        concat!(
            "  \"status_selection_guardrails\": [\n",
            "    {\"surface\": \"child_write_status_selection\", \"selector_source\": \"FixedSclBinaryChildWriteDomainCheck.failure_code\", \"selected_outputs\": \"existing slots are preserved on invalid public inputs\", \"posture\": \"source-level status-selected update only; not a production constant-time claim; not a security claim\"},\n",
            "    {\"surface\": \"path_buffer_status_selection\", \"selector_source\": \"FixedSclPublicRoundScheduleRun.path_domain_check.failure_code\", \"selected_outputs\": \"path buffers and top-L outputs are status-selected on invalid public path-domain checks\", \"posture\": \"source-level status-selected update only; not a production constant-time claim; not a security claim\"},\n",
            "    {\"surface\": \"integer_schedule_status_selection\", \"selector_source\": \"FixedSclIntegerRoundScheduleBuild.domain_check.failure_code\", \"selected_outputs\": \"integer schedule/status records are exposed as public audit fields\", \"posture\": \"source-level public status accounting only; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_domain_guardrails_json_lines {
    () => {
        concat!(
            "  \"public_domain_guardrails\": [\n",
            "    {\"surface\": \"top_l_public_shape\", \"public_inputs\": \"public list width, list size, and candidate count\", \"rejects\": \"empty candidate lists and invalid public top-L shape\", \"posture\": \"public invariant screen only; not a reduction; not a security claim\"},\n",
            "    {\"surface\": \"path_buffer_public_shape\", \"public_inputs\": \"public bit indices, parent slots, child capacities, and path bit width\", \"rejects\": \"empty schedules, child-capacity overflow, and bit-index overflow\", \"posture\": \"public domain/status handling only; not a production constant-time claim\"},\n",
            "    {\"surface\": \"integer_metric_public_domain\", \"public_inputs\": \"public hard decisions and integer metric magnitudes\", \"rejects\": \"non-bit hard decisions and negative integer magnitudes\", \"posture\": \"public integer-domain guardrail only; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_field_provenance_json_lines {
    () => {
        concat!(
            "  \"public_status_field_provenance\": [\n",
            "    {\"field\": \"FixedSclBinaryChildWriteDomainCheck.failure_code\", \"domain\": \"public status field\", \"source\": \"derived from public preflight/domain checks: child-write domain\", \"posture\": \"not secret-dependent branching evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"field\": \"FixedSclPublicRoundScheduleRun.path_domain_check.failure_code\", \"domain\": \"public status field\", \"source\": \"derived from public preflight/domain checks: path-domain preflight\", \"posture\": \"not secret-dependent branching evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"field\": \"FixedSclIntegerRoundScheduleBuild.domain_check.failure_code\", \"domain\": \"public status field\", \"source\": \"derived from public preflight/domain checks: integer schedule domain\", \"posture\": \"not secret-dependent branching evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"field\": \"FixedSclPathBufferIntegerScheduleRun.domain_check.failure_code\", \"domain\": \"public status field\", \"source\": \"derived from public preflight/domain checks: integer/path schedule domain\", \"posture\": \"not secret-dependent branching evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_selector_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_selector_obligations\": [\n",
            "    {\"selector\": \"FixedSclBinaryChildWriteDomainCheck.failure_code\", \"domain\": \"public status selector\", \"obligation\": \"mask/select obligation preserves existing child slots and publishable status while it must preserve slot shape\", \"posture\": \"not secret-dependent control flow evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"selector\": \"FixedSclPublicRoundScheduleRun.path_domain_check.failure_code\", \"domain\": \"public status selector\", \"obligation\": \"mask/select obligation preserves path buffers and top-L output shape while publishing public path-domain status\", \"posture\": \"not secret-dependent control flow evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"selector\": \"FixedSclIntegerRoundScheduleBuild.domain_check.failure_code\", \"domain\": \"public status selector\", \"obligation\": \"mask/select obligation preserves integer schedule shape and public status accounting\", \"posture\": \"not secret-dependent control flow evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_shape_preservation_obligations_json_lines {
    () => {
        concat!(
            "  \"public_shape_preservation_obligations\": [\n",
            "    {\"surface\": \"top_l_selection_output_shape\", \"domain\": \"public shape/work-count obligation\", \"obligation\": \"preserve output shape and fixed public work shape across public top-L list width and list-size checks\", \"posture\": \"audit-only source-level obligation; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"surface\": \"path_buffer_round_output_shape\", \"domain\": \"public shape/work-count obligation\", \"obligation\": \"preserve output shape and fixed public work shape across public path-buffer round status checks\", \"posture\": \"audit-only source-level obligation; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"surface\": \"integer_schedule_work_shape\", \"domain\": \"public shape/work-count obligation\", \"obligation\": \"preserve output shape and fixed public work shape across public integer schedule and metric-domain status checks\", \"posture\": \"audit-only source-level obligation; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_failure_code_wrapper_obligations_json_lines {
    () => {
        concat!(
            "  \"public_failure_code_wrapper_obligations\": [\n",
            "    {\"surface\": \"top_l_selection_status_wrapper\", \"domain\": \"public failure-code wrapper\", \"obligation\": \"return public failure codes instead of panic-as-control-flow for invalid public top-L shape\", \"posture\": \"audit-only wrapper obligation; not a panic-freedom proof; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"surface\": \"path_buffer_schedule_status_wrapper\", \"domain\": \"public failure-code wrapper\", \"obligation\": \"return public failure codes instead of panic-as-control-flow for invalid public path-buffer schedule shape\", \"posture\": \"audit-only wrapper obligation; not a panic-freedom proof; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"surface\": \"integer_schedule_status_wrapper\", \"domain\": \"public failure-code wrapper\", \"obligation\": \"return public failure codes instead of panic-as-control-flow for invalid public integer schedule or metric domain\", \"posture\": \"audit-only wrapper obligation; not a panic-freedom proof; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_preflight_coverage_obligations_json_lines {
    () => {
        concat!(
            "  \"public_preflight_coverage_obligations\": [\n",
            "    {\"preflight\": \"top_l_shape_preflight\", \"domain\": \"public preflight coverage\", \"obligation\": \"screen public list width, list size, and candidate count before schedule-dependent writes\", \"posture\": \"public invariant screen only; audit-only coverage obligation; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"preflight\": \"path_buffer_capacity_preflight\", \"domain\": \"public preflight coverage\", \"obligation\": \"screen public bit indices, parent slots, and child capacities before schedule-dependent writes\", \"posture\": \"public invariant screen only; audit-only coverage obligation; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"preflight\": \"integer_metric_domain_preflight\", \"domain\": \"public preflight coverage\", \"obligation\": \"screen public hard decisions and integer metric magnitudes before schedule-dependent writes\", \"posture\": \"public invariant screen only; audit-only coverage obligation; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_invalid_input_status_reporting_obligations_json_lines {
    () => {
        concat!(
            "  \"public_invalid_input_status_reporting_obligations\": [\n",
            "    {\"report\": \"top_l_invalid_shape_status\", \"domain\": \"public invalid-input status\", \"obligation\": \"record public audit status for invalid public top-L shape without secret-dependent status claims\", \"posture\": \"audit-only reporting obligation; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"report\": \"path_buffer_invalid_capacity_status\", \"domain\": \"public invalid-input status\", \"obligation\": \"record public audit status for invalid public path-buffer capacity without secret-dependent status claims\", \"posture\": \"audit-only reporting obligation; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"report\": \"integer_metric_invalid_domain_status\", \"domain\": \"public invalid-input status\", \"obligation\": \"record public audit status for invalid public integer metric domain without secret-dependent status claims\", \"posture\": \"audit-only reporting obligation; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_rejection_reason_taxonomy_json_lines {
    () => {
        concat!(
            "  \"public_rejection_reason_taxonomy\": [\n",
            "    {\"reason\": \"invalid_top_l_shape\", \"domain\": \"public rejection reason\", \"source\": \"derived from public domain checks over list width, list size, and candidate count\", \"posture\": \"audit-only reason taxonomy; not secret-derived evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"reason\": \"invalid_path_buffer_capacity\", \"domain\": \"public rejection reason\", \"source\": \"derived from public domain checks over bit indices, parent slots, and child capacities\", \"posture\": \"audit-only reason taxonomy; not secret-derived evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"reason\": \"invalid_integer_metric_domain\", \"domain\": \"public rejection reason\", \"source\": \"derived from public domain checks over hard decisions and integer metric magnitudes\", \"posture\": \"audit-only reason taxonomy; not secret-derived evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_lifecycle_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_lifecycle_obligations\": [\n",
            "    {\"phase\": \"public_preflight_status\", \"domain\": \"public status lifecycle\", \"source\": \"derived from public checks only before schedule-dependent writes\", \"posture\": \"audit-only lifecycle obligation; not secret-state evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"phase\": \"public_failure_code_status\", \"domain\": \"public status lifecycle\", \"source\": \"derived from public checks only when wrappers return public failure codes\", \"posture\": \"audit-only lifecycle obligation; not secret-state evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"phase\": \"public_audit_report_status\", \"domain\": \"public status lifecycle\", \"source\": \"derived from public checks only when audit JSON reports rejection reasons\", \"posture\": \"audit-only lifecycle obligation; not secret-state evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_transition_ordering_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_transition_ordering_obligations\": [\n",
            "    {\"transition\": \"preflight_before_failure_code\", \"domain\": \"public status transition ordering\", \"obligation\": \"public-before-public transition from public preflight checks to public failure codes\", \"posture\": \"audit-only ordering obligation; not secret-order evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"transition\": \"failure_code_before_mask_select\", \"domain\": \"public status transition ordering\", \"obligation\": \"public-before-public transition from public failure codes to mask/select status accounting\", \"posture\": \"audit-only ordering obligation; not secret-order evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"transition\": \"mask_select_before_audit_report\", \"domain\": \"public status transition ordering\", \"obligation\": \"public-before-public transition from mask/select status accounting to audit JSON reporting\", \"posture\": \"audit-only ordering obligation; not secret-order evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_stability_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_stability_obligations\": [\n",
            "    {\"field_family\": \"public_preflight_status_fields\", \"domain\": \"public status schema stability\", \"obligation\": \"keep stable audit JSON field names for public preflight status fields\", \"posture\": \"public schema obligation; not secret-schema evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"field_family\": \"public_failure_code_fields\", \"domain\": \"public status schema stability\", \"obligation\": \"keep stable audit JSON field names for public failure-code fields\", \"posture\": \"public schema obligation; not secret-schema evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"field_family\": \"public_audit_report_fields\", \"domain\": \"public status schema stability\", \"obligation\": \"keep stable audit JSON field names for public audit-report fields\", \"posture\": \"public schema obligation; not secret-schema evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_compatibility_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_compatibility_obligations\": [\n",
            "    {\"field_family\": \"public_status_schema_version_field\", \"domain\": \"public status schema compatibility\", \"obligation\": \"record a public schema version field when public audit status fields change\", \"posture\": \"public compatibility obligation; not secret-compatibility evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"field_family\": \"append_only_public_status_fields\", \"domain\": \"public status schema compatibility\", \"obligation\": \"prefer append-only compatibility for public audit status fields\", \"posture\": \"public compatibility obligation; not secret-compatibility evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"field_family\": \"unknown_public_status_field_handling\", \"domain\": \"public status schema compatibility\", \"obligation\": \"treat unknown public audit status fields as public compatibility data rather than secret evidence\", \"posture\": \"public compatibility obligation; not secret-compatibility evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_consumer_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_consumer_obligations\": [\n",
            "    {\"consumer\": \"public_only_status_consumers\", \"domain\": \"public status schema consumer\", \"obligation\": \"consume public audit JSON fields only when interpreting public status reports\", \"posture\": \"public consumer obligation; not secret-consumer evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"consumer\": \"unknown_public_status_field_policy\", \"domain\": \"public status schema consumer\", \"obligation\": \"unknown public fields stay public compatibility data rather than secret evidence\", \"posture\": \"public consumer obligation; not secret-consumer evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"consumer\": \"compatibility_report_consumers\", \"domain\": \"public status schema consumer\", \"obligation\": \"consume compatibility report fields as audit-only public metadata\", \"posture\": \"public consumer obligation; not secret-consumer evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_producer_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_producer_obligations\": [\n",
            "    {\"producer\": \"public_only_status_producers\", \"domain\": \"public status schema producer\", \"obligation\": \"emit public audit JSON fields only when producing public status reports\", \"posture\": \"public producer obligation; not secret-producer evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"producer\": \"public_status_field_source_tags\", \"domain\": \"public status schema producer\", \"obligation\": \"tag public field sources as public metadata in audit-only reports\", \"posture\": \"public producer obligation; not secret-producer evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"producer\": \"producer_side_secret_evidence_exclusion\", \"domain\": \"public status schema producer\", \"obligation\": \"exclude secret-evidence interpretations from producer-side public status schema fields\", \"posture\": \"public producer obligation; not secret-producer evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_round_trip_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_round_trip_obligations\": [\n",
            "    {\"round_trip\": \"public_status_json_generation\", \"domain\": \"public status schema round-trip\", \"obligation\": \"generated public audit JSON stays public metadata\", \"posture\": \"public round-trip obligation; not secret-round-trip evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"round_trip\": \"public_status_cli_write_check\", \"domain\": \"public status schema round-trip\", \"obligation\": \"CLI write/check round-trip remains audit-only public metadata\", \"posture\": \"public round-trip obligation; not secret-round-trip evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"round_trip\": \"public_status_consumer_replay\", \"domain\": \"public status schema round-trip\", \"obligation\": \"consumer replay of public audit fields remains public metadata\", \"posture\": \"public round-trip obligation; not secret-round-trip evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_drift_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_drift_obligations\": [\n",
            "    {\"drift\": \"public_status_field_addition_drift\", \"domain\": \"public status schema drift\", \"obligation\": \"field additions are public audit metadata\", \"posture\": \"public drift obligation; not secret-drift evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"drift\": \"public_status_field_rename_drift\", \"domain\": \"public status schema drift\", \"obligation\": \"field renames require public compatibility notes\", \"posture\": \"public drift obligation; not secret-drift evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"drift\": \"public_status_field_order_drift\", \"domain\": \"public status schema drift\", \"obligation\": \"field order drift remains public audit metadata\", \"posture\": \"public drift obligation; not secret-drift evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_migration_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_migration_obligations\": [\n",
            "    {\"migration\": \"public_status_schema_version_migration\", \"domain\": \"public status schema migration\", \"obligation\": \"schema version migration stays public compatibility metadata\", \"posture\": \"public migration obligation; not secret-migration evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"migration\": \"public_status_field_compatibility_map\", \"domain\": \"public status schema migration\", \"obligation\": \"compatibility maps are public audit metadata\", \"posture\": \"public migration obligation; not secret-migration evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"migration\": \"public_status_deprecated_field_notice\", \"domain\": \"public status schema migration\", \"obligation\": \"deprecated field notices stay public compatibility metadata\", \"posture\": \"public migration obligation; not secret-migration evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_rollback_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_rollback_obligations\": [\n",
            "    {\"rollback\": \"public_status_schema_version_rollback\", \"domain\": \"public status schema rollback\", \"obligation\": \"schema version rollback stays public compatibility metadata\", \"posture\": \"public rollback obligation; not secret-rollback evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"rollback\": \"public_status_field_compatibility_rollback\", \"domain\": \"public status schema rollback\", \"obligation\": \"rollback maps are public audit metadata\", \"posture\": \"public rollback obligation; not secret-rollback evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"rollback\": \"public_status_deprecated_field_restore\", \"domain\": \"public status schema rollback\", \"obligation\": \"restored deprecated fields stay public compatibility metadata\", \"posture\": \"public rollback obligation; not secret-rollback evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_deprecation_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_deprecation_obligations\": [\n",
            "    {\"deprecation\": \"public_status_field_deprecation_notice\", \"domain\": \"public status schema deprecation\", \"obligation\": \"deprecation notices stay public compatibility metadata\", \"posture\": \"public deprecation obligation; not secret-deprecation evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"deprecation\": \"public_status_field_replacement_hint\", \"domain\": \"public status schema deprecation\", \"obligation\": \"replacement hints are public audit metadata\", \"posture\": \"public deprecation obligation; not secret-deprecation evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"deprecation\": \"public_status_deprecated_field_retention\", \"domain\": \"public status schema deprecation\", \"obligation\": \"deprecated field retention remains public compatibility metadata\", \"posture\": \"public deprecation obligation; not secret-deprecation evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_removal_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_removal_obligations\": [\n",
            "    {\"removal\": \"public_status_field_removal_notice\", \"domain\": \"public status schema removal\", \"obligation\": \"field removal notices stay public compatibility metadata\", \"posture\": \"public removal obligation; not secret-removal evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"removal\": \"public_status_field_removal_window\", \"domain\": \"public status schema removal\", \"obligation\": \"field removal windows stay public compatibility metadata\", \"posture\": \"public removal obligation; not secret-removal evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"removal\": \"public_status_removed_field_tombstone\", \"domain\": \"public status schema removal\", \"obligation\": \"removed field tombstones are public audit metadata\", \"posture\": \"public removal obligation; not secret-removal evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_archival_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_archival_obligations\": [\n",
            "    {\"archive\": \"public_status_archived_field_catalog\", \"domain\": \"public status schema archival\", \"obligation\": \"archived field catalogs stay public audit metadata\", \"posture\": \"public archival obligation; not secret-archive evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"archive\": \"public_status_archived_transition_log\", \"domain\": \"public status schema archival\", \"obligation\": \"archived transition logs stay public compatibility metadata\", \"posture\": \"public archival obligation; not secret-archive evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"archive\": \"public_status_archive_replay_notice\", \"domain\": \"public status schema archival\", \"obligation\": \"archive replay notices are public reproducibility metadata\", \"posture\": \"public archival obligation; not secret-archive evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_sunset_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_sunset_obligations\": [\n",
            "    {\"sunset\": \"public_status_sunset_notice\", \"domain\": \"public status schema sunset\", \"obligation\": \"sunset notices stay public lifecycle metadata\", \"posture\": \"public sunset obligation; not secret-sunset evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"sunset\": \"public_status_sunset_cutover_epoch\", \"domain\": \"public status schema sunset\", \"obligation\": \"sunset cutover epochs stay public compatibility metadata\", \"posture\": \"public sunset obligation; not secret-sunset evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"sunset\": \"public_status_sunset_archive_reference\", \"domain\": \"public status schema sunset\", \"obligation\": \"archive references stay public reproducibility metadata\", \"posture\": \"public sunset obligation; not secret-sunset evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_retention_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_retention_obligations\": [\n",
            "    {\"retention\": \"public_status_retention_period\", \"domain\": \"public status schema retention\", \"obligation\": \"retention periods stay public lifecycle metadata\", \"posture\": \"public retention obligation; not secret-retention evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"retention\": \"public_status_retention_manifest\", \"domain\": \"public status schema retention\", \"obligation\": \"retention manifests stay public audit metadata\", \"posture\": \"public retention obligation; not secret-retention evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"retention\": \"public_status_retention_review_marker\", \"domain\": \"public status schema retention\", \"obligation\": \"retention review markers stay public audit metadata\", \"posture\": \"public retention obligation; not secret-retention evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_lifecycle_order_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_lifecycle_order_obligations\": [\n",
            "    {\"stage\": \"public_status_schema_deprecation\", \"domain\": \"public status schema lifecycle order\", \"obligation\": \"deprecation precedes removal as public lifecycle metadata\", \"posture\": \"public lifecycle-order obligation; not secret-order evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"stage\": \"public_status_schema_removal\", \"domain\": \"public status schema lifecycle order\", \"obligation\": \"removal precedes archival as public lifecycle metadata\", \"posture\": \"public lifecycle-order obligation; not secret-order evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"stage\": \"public_status_schema_archival\", \"domain\": \"public status schema lifecycle order\", \"obligation\": \"archival precedes sunset as public lifecycle metadata\", \"posture\": \"public lifecycle-order obligation; not secret-order evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"stage\": \"public_status_schema_sunset\", \"domain\": \"public status schema lifecycle order\", \"obligation\": \"sunset precedes retention as public lifecycle metadata\", \"posture\": \"public lifecycle-order obligation; not secret-order evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"stage\": \"public_status_schema_retention\", \"domain\": \"public status schema lifecycle order\", \"obligation\": \"retention closes the public lifecycle order\", \"posture\": \"public lifecycle-order obligation; not secret-order evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_lifecycle_shape_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_lifecycle_shape_obligations\": [\n",
            "    {\"shape\": \"public_status_lifecycle_stage_count\", \"domain\": \"public status schema lifecycle shape\", \"obligation\": \"stage counts stay public fixed-shape metadata\", \"posture\": \"public lifecycle-shape obligation; not secret-shape evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"shape\": \"public_status_lifecycle_stage_order\", \"domain\": \"public status schema lifecycle shape\", \"obligation\": \"stage order stays public fixed-work metadata\", \"posture\": \"public lifecycle-shape obligation; not secret-shape evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"shape\": \"public_status_lifecycle_stage_labels\", \"domain\": \"public status schema lifecycle shape\", \"obligation\": \"stage labels stay public audit metadata\", \"posture\": \"public lifecycle-shape obligation; not secret-shape evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_lifecycle_registry_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_lifecycle_registry_obligations\": [\n",
            "    {\"registry\": \"public_status_lifecycle_registry_name\", \"domain\": \"public status schema lifecycle registry\", \"obligation\": \"registry names stay public audit metadata\", \"posture\": \"public lifecycle-registry obligation; not secret-registry evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"registry\": \"public_status_lifecycle_registry_exposure\", \"domain\": \"public status schema lifecycle registry\", \"obligation\": \"registry exposure stays public lifecycle metadata\", \"posture\": \"public lifecycle-registry obligation; not secret-registry evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"registry\": \"public_status_lifecycle_registry_consumer_anchor\", \"domain\": \"public status schema lifecycle registry\", \"obligation\": \"consumer anchors stay public compatibility metadata\", \"posture\": \"public lifecycle-registry obligation; not secret-registry evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_lifecycle_manifest_obligations_json_lines {
    () => {
        concat!(
            "  \"public_status_schema_lifecycle_manifest_obligations\": [\n",
            "    {\"manifest\": \"public_status_lifecycle_manifest_id\", \"domain\": \"public status schema lifecycle manifest\", \"obligation\": \"manifest ids stay public audit metadata\", \"posture\": \"public lifecycle-manifest obligation; not secret-manifest evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"manifest\": \"public_status_lifecycle_manifest_stage_list\", \"domain\": \"public status schema lifecycle manifest\", \"obligation\": \"manifest stage lists stay public fixed-shape metadata\", \"posture\": \"public lifecycle-manifest obligation; not secret-manifest evidence; not a production constant-time claim; not a reduction; not a security claim\"},\n",
            "    {\"manifest\": \"public_status_lifecycle_manifest_consumer_notice\", \"domain\": \"public status schema lifecycle manifest\", \"obligation\": \"consumer notices stay public compatibility metadata\", \"posture\": \"public lifecycle-manifest obligation; not secret-manifest evidence; not a production constant-time claim; not a reduction; not a security claim\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_lifecycle_three_entry_obligations_json_lines {
    (
        $section:literal,
        $field:literal,
        $domain:literal,
        $posture:literal,
        ($entry0:literal, $obligation0:literal),
        ($entry1:literal, $obligation1:literal),
        ($entry2:literal, $obligation2:literal) $(,)?
) => {
        concat!(
            "  \"public_status_schema_lifecycle_",
            $section,
            "_obligations\": [\n",
            "    {\"",
            $field,
            "\": \"",
            $entry0,
            "\", \"domain\": \"",
            $domain,
            "\", \"obligation\": \"",
            $obligation0,
            "\", \"posture\": \"",
            $posture,
            "\"},\n",
            "    {\"",
            $field,
            "\": \"",
            $entry1,
            "\", \"domain\": \"",
            $domain,
            "\", \"obligation\": \"",
            $obligation1,
            "\", \"posture\": \"",
            $posture,
            "\"},\n",
            "    {\"",
            $field,
            "\": \"",
            $entry2,
            "\", \"domain\": \"",
            $domain,
            "\", \"obligation\": \"",
            $obligation2,
            "\", \"posture\": \"",
            $posture,
            "\"}\n",
            "  ],\n",
)
    };
}

macro_rules! scl_work_shape_public_status_schema_lifecycle_replay_obligations_json_lines {
    () => {
        scl_work_shape_public_status_schema_lifecycle_three_entry_obligations_json_lines!(
            "replay",
            "replay",
            "public status schema lifecycle replay",
            "public lifecycle-replay obligation; not secret-replay evidence; not a production constant-time claim; not a reduction; not a security claim",
            (
                "public_status_lifecycle_replay_label",
                "replay labels stay public reproducibility metadata"
),
            (
                "public_status_lifecycle_replay_stage_trace",
                "stage traces stay public fixed-shape metadata"
),
            (
                "public_status_lifecycle_replay_consumer_check",
                "consumer checks stay public compatibility metadata"
),
)
    };
}

macro_rules! scl_work_shape_public_status_schema_lifecycle_digest_obligations_json_lines {
    () => {
        scl_work_shape_public_status_schema_lifecycle_three_entry_obligations_json_lines!(
            "digest",
            "digest",
            "public status schema lifecycle digest",
            "public lifecycle-digest obligation; not secret-digest evidence; not a production constant-time claim; not a reduction; not a security claim",
            (
                "public_status_lifecycle_digest_label",
                "digest labels stay public reproducibility metadata"
),
            (
                "public_status_lifecycle_digest_stage_checksum",
                "stage checksums stay public fixed-shape metadata"
),
            (
                "public_status_lifecycle_digest_consumer_comparison",
                "consumer comparisons stay public compatibility metadata"
),
)
    };
}

macro_rules! scl_work_shape_public_status_schema_lifecycle_receipt_obligations_json_lines {
    () => {
        scl_work_shape_public_status_schema_lifecycle_three_entry_obligations_json_lines!(
            "receipt",
            "receipt",
            "public status schema lifecycle receipt",
            "public lifecycle-receipt obligation; not secret-receipt evidence; not a production constant-time claim; not a reduction; not a security claim",
            (
                "public_status_lifecycle_receipt_label",
                "receipt labels stay public reproducibility metadata"
),
            (
                "public_status_lifecycle_receipt_stage_ack",
                "stage acknowledgements stay public fixed-shape metadata"
),
            (
                "public_status_lifecycle_receipt_consumer_boundary",
                "consumer boundaries stay public compatibility metadata"
),
)
    };
}

macro_rules! scl_work_shape_public_status_schema_lifecycle_transcript_obligations_json_lines {
    () => {
        scl_work_shape_public_status_schema_lifecycle_three_entry_obligations_json_lines!(
            "transcript",
            "transcript",
            "public status schema lifecycle transcript",
            "public lifecycle-transcript obligation; not secret-transcript evidence; not a production constant-time claim; not a reduction; not a security claim",
            (
                "public_status_lifecycle_transcript_label",
                "transcript labels stay public reproducibility metadata"
),
            (
                "public_status_lifecycle_transcript_stage_entry",
                "stage entries stay public fixed-shape metadata"
),
            (
                "public_status_lifecycle_transcript_consumer_join",
                "consumer joins stay public compatibility metadata"
),
)
    };
}

macro_rules! scl_work_shape_public_status_schema_lifecycle_witness_obligations_json_lines {
    () => {
        scl_work_shape_public_status_schema_lifecycle_three_entry_obligations_json_lines!(
            "witness",
            "witness",
            "public status schema lifecycle witness",
            "public lifecycle-witness obligation; not secret-witness evidence; not a production constant-time claim; not a reduction; not a security claim",
            (
                "public_status_lifecycle_witness_label",
                "witness labels stay public reproducibility metadata"
),
            (
                "public_status_lifecycle_witness_stage_marker",
                "stage markers stay public fixed-shape metadata"
),
            (
                "public_status_lifecycle_witness_consumer_view",
                "consumer views stay public compatibility metadata"
),
)
    };
}

macro_rules! scl_work_shape_public_status_schema_lifecycle_audit_trail_obligations_json_lines {
    () => {
        scl_work_shape_public_status_schema_lifecycle_three_entry_obligations_json_lines!(
            "audit_trail",
            "audit_trail",
            "public status schema lifecycle audit trail",
            "public lifecycle-audit-trail obligation; not secret-audit-trail evidence; not a production constant-time claim; not a reduction; not a security claim",
            (
                "public_status_lifecycle_audit_trail_label",
                "audit trail labels stay public reproducibility metadata"
),
            (
                "public_status_lifecycle_audit_trail_stage_note",
                "stage notes stay public fixed-shape metadata"
),
            (
                "public_status_lifecycle_audit_trail_consumer_readback",
                "consumer readbacks stay public compatibility metadata"
),
)
    };
}

macro_rules! scl_work_shape_public_status_schema_lifecycle_checkpoint_obligations_json_lines {
    () => {
        scl_work_shape_public_status_schema_lifecycle_three_entry_obligations_json_lines!(
            "checkpoint",
            "checkpoint",
            "public status schema lifecycle checkpoint",
            "public lifecycle-checkpoint obligation; not secret-checkpoint evidence; not a production constant-time claim; not a reduction; not a security claim",
            (
                "public_status_lifecycle_checkpoint_label",
                "checkpoint labels stay public reproducibility metadata"
),
            (
                "public_status_lifecycle_checkpoint_stage_slot",
                "stage slots stay public fixed-shape metadata"
),
            (
                "public_status_lifecycle_checkpoint_consumer_resume",
                "consumer resumes stay public compatibility metadata"
),
)
    };
}

macro_rules! ofa_evidence_entry_scope_json_field {
    () => {
        concat!("\"evidence_scope\": \"", ofa_evidence_scope_value!(), "\"")
    };
}

macro_rules! ofa_evidence_entry_runtime_dependency_json_field {
    () => {
        concat!(
            "\"runtime_dependency\": ",
            ofa_evidence_runtime_dependency_value!()
)
    };
}

macro_rules! ofa_evidence_entry_paper_claim_dependency_json_field {
    () => {
        concat!(
            "\"paper_claim_dependency\": ",
            ofa_evidence_paper_claim_dependency_value!()
)
    };
}

macro_rules! ofa_evidence_entry_use_in_impl_json_field {
    () => {
        concat!(
            "\"use_in_impl\": \"",
            ofa_evidence_use_in_impl_value!(),
            "\""
)
    };
}

macro_rules! ofa_evidence_audit_posture_json_lines {
    () => {
        concat!(
            "  ",
            ofa_evidence_scope_json_field!(),
            ",\n",
            "  ",
            ofa_evidence_runtime_dependency_json_field!(),
            ",\n",
            "  ",
            ofa_evidence_paper_claim_dependency_json_field!(),
            ",\n",
)
    };
}

macro_rules! ofa_evidence_entry_audit_posture_json_lines {
    () => {
        concat!(
            "      ",
            ofa_evidence_entry_scope_json_field!(),
            ",\n",
            "      ",
            ofa_evidence_entry_paper_claim_dependency_json_field!(),
            ",\n",
            "      ",
            ofa_evidence_entry_runtime_dependency_json_field!(),
            ",\n",
            "      ",
            ofa_evidence_entry_use_in_impl_json_field!(),
            "\n",
)
    };
}

pub const FIXED_I64_VALIDATION_METRIC_SCALE: f64 = 1024.0;
pub const OFA_EVIDENCE_SCOPE: &str = ofa_evidence_scope_value!();
pub const OFA_EVIDENCE_RUNTIME_DEPENDENCY: bool = false;
pub const OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY: bool = false;
pub const OFA_EVIDENCE_USE_IN_IMPL: &str = ofa_evidence_use_in_impl_value!();
pub const MARGINAL_INVARIANT_OFA_EVIDENCE_IDS: [&str; 4] =
    ["OFA-359", "OFA-360", "OFA-398", "OFA-399"];
pub const SCL_WORK_SHAPE_OFA_EVIDENCE_IDS: [&str; 6] = [
    "OFA-359", "OFA-360", "OFA-388", "OFA-391", "OFA-398", "OFA-399",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OfaEvidenceEntry {
    pub ofa: &'static str,
    pub finding: &'static str,
    pub claim_posture: &'static str,
    pub evidence_scope: &'static str,
    pub runtime_dependency: bool,
    pub paper_claim_dependency: bool,
    pub use_in_impl: &'static str,
}

pub const MARGINAL_INVARIANT_OFA_EVIDENCE: [OfaEvidenceEntry; 4] = [
    OfaEvidenceEntry {
        ofa: "OFA-359",
        finding: "low_degree_cube_sparsity_wall",
        claim_posture: "NOT A REDUCTION; NOT A SECURITY CLAIM",
        evidence_scope: OFA_EVIDENCE_SCOPE,
        runtime_dependency: OFA_EVIDENCE_RUNTIME_DEPENDENCY,
        paper_claim_dependency: OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY,
        use_in_impl: OFA_EVIDENCE_USE_IN_IMPL,
    },
    OfaEvidenceEntry {
        ofa: "OFA-360",
        finding: "full_noise_law_not_preserved_by_nonlocal_sp",
        claim_posture: "NOT A REDUCTION; NOT A SECURITY CLAIM",
        evidence_scope: OFA_EVIDENCE_SCOPE,
        runtime_dependency: OFA_EVIDENCE_RUNTIME_DEPENDENCY,
        paper_claim_dependency: OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY,
        use_in_impl: OFA_EVIDENCE_USE_IN_IMPL,
    },
    OfaEvidenceEntry {
        ofa: "OFA-398",
        finding: "omega_stress_margin_live_signal_wall_bound",
        claim_posture: "live refinement signal; NOT A REDUCTION",
        evidence_scope: OFA_EVIDENCE_SCOPE,
        runtime_dependency: OFA_EVIDENCE_RUNTIME_DEPENDENCY,
        paper_claim_dependency: OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY,
        use_in_impl: OFA_EVIDENCE_USE_IN_IMPL,
    },
    OfaEvidenceEntry {
        ofa: "OFA-399",
        finding: "constant_rate_wall_n7",
        claim_posture: "wall-bound evidence; NOT A REDUCTION; NOT A SECURITY CLAIM",
        evidence_scope: OFA_EVIDENCE_SCOPE,
        runtime_dependency: OFA_EVIDENCE_RUNTIME_DEPENDENCY,
        paper_claim_dependency: OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY,
        use_in_impl: OFA_EVIDENCE_USE_IN_IMPL,
    },
];

pub const SCL_WORK_SHAPE_OFA_EVIDENCE: [OfaEvidenceEntry; 6] = [
    OfaEvidenceEntry {
        ofa: "OFA-359",
        finding: "low_degree_cube_sparsity_wall",
        claim_posture: "NOT A REDUCTION; NOT A SECURITY CLAIM",
        evidence_scope: OFA_EVIDENCE_SCOPE,
        runtime_dependency: OFA_EVIDENCE_RUNTIME_DEPENDENCY,
        paper_claim_dependency: OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY,
        use_in_impl: OFA_EVIDENCE_USE_IN_IMPL,
    },
    OfaEvidenceEntry {
        ofa: "OFA-360",
        finding: "full_noise_law_not_preserved_by_nonlocal_sp",
        claim_posture: "NOT A REDUCTION; NOT A SECURITY CLAIM",
        evidence_scope: OFA_EVIDENCE_SCOPE,
        runtime_dependency: OFA_EVIDENCE_RUNTIME_DEPENDENCY,
        paper_claim_dependency: OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY,
        use_in_impl: OFA_EVIDENCE_USE_IN_IMPL,
    },
    OfaEvidenceEntry {
        ofa: "OFA-388",
        finding: "k3_count_guardrail",
        claim_posture: "OPEN; proof-text guardrail, not implementation security",
        evidence_scope: OFA_EVIDENCE_SCOPE,
        runtime_dependency: OFA_EVIDENCE_RUNTIME_DEPENDENCY,
        paper_claim_dependency: OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY,
        use_in_impl: OFA_EVIDENCE_USE_IN_IMPL,
    },
    OfaEvidenceEntry {
        ofa: "OFA-391",
        finding: "corrected_inverse_correlation_bits_about_2n_minus_1",
        claim_posture: "OPEN; parameter-table guardrail, not implementation security",
        evidence_scope: OFA_EVIDENCE_SCOPE,
        runtime_dependency: OFA_EVIDENCE_RUNTIME_DEPENDENCY,
        paper_claim_dependency: OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY,
        use_in_impl: OFA_EVIDENCE_USE_IN_IMPL,
    },
    OfaEvidenceEntry {
        ofa: "OFA-398",
        finding: "omega_stress_margin_live_signal_wall_bound",
        claim_posture: "live refinement signal; NOT A REDUCTION",
        evidence_scope: OFA_EVIDENCE_SCOPE,
        runtime_dependency: OFA_EVIDENCE_RUNTIME_DEPENDENCY,
        paper_claim_dependency: OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY,
        use_in_impl: OFA_EVIDENCE_USE_IN_IMPL,
    },
    OfaEvidenceEntry {
        ofa: "OFA-399",
        finding: "constant_rate_wall_n7",
        claim_posture: "wall-bound evidence; NOT A REDUCTION; NOT A SECURITY CLAIM",
        evidence_scope: OFA_EVIDENCE_SCOPE,
        runtime_dependency: OFA_EVIDENCE_RUNTIME_DEPENDENCY,
        paper_claim_dependency: OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY,
        use_in_impl: OFA_EVIDENCE_USE_IN_IMPL,
    },
];

pub const MARGINAL_INVARIANT_OFA_EVIDENCE_ENTRY_COUNT: usize =
    MARGINAL_INVARIANT_OFA_EVIDENCE.len();
pub const SCL_WORK_SHAPE_OFA_EVIDENCE_ENTRY_COUNT: usize = SCL_WORK_SHAPE_OFA_EVIDENCE.len();

pub const fn marginal_invariant_ofa_evidence_entry_count() -> usize {
    MARGINAL_INVARIANT_OFA_EVIDENCE_ENTRY_COUNT
}

pub const fn scl_work_shape_ofa_evidence_entry_count() -> usize {
    SCL_WORK_SHAPE_OFA_EVIDENCE_ENTRY_COUNT
}

pub fn marginal_invariant_ofa_evidence_entry_count_json_field() -> &'static str {
    marginal_invariant_ofa_evidence_entry_count_json_field!()
}

pub fn scl_work_shape_ofa_evidence_entry_count_json_field() -> &'static str {
    scl_work_shape_ofa_evidence_entry_count_json_field!()
}

pub fn marginal_invariant_ofa_evidence_ids_json_field() -> &'static str {
    marginal_invariant_ofa_evidence_ids_json_field!()
}

pub fn scl_work_shape_ofa_evidence_ids_json_field() -> &'static str {
    scl_work_shape_ofa_evidence_ids_json_field!()
}

pub fn marginal_invariant_ofa_evidence_summary_json_lines() -> &'static str {
    marginal_invariant_ofa_evidence_summary_json_lines!()
}

pub fn scl_work_shape_ofa_evidence_summary_json_lines() -> &'static str {
    scl_work_shape_ofa_evidence_summary_json_lines!()
}

pub fn marginal_invariant_ofa_evidence_summary_json_fields() -> [&'static str; 5] {
    [
        ofa_evidence_scope_json_field(),
        ofa_evidence_runtime_dependency_json_field(),
        ofa_evidence_paper_claim_dependency_json_field(),
        marginal_invariant_ofa_evidence_entry_count_json_field(),
        marginal_invariant_ofa_evidence_ids_json_field(),
    ]
}

pub fn scl_work_shape_ofa_evidence_summary_json_fields() -> [&'static str; 5] {
    [
        ofa_evidence_scope_json_field(),
        ofa_evidence_runtime_dependency_json_field(),
        ofa_evidence_paper_claim_dependency_json_field(),
        scl_work_shape_ofa_evidence_entry_count_json_field(),
        scl_work_shape_ofa_evidence_ids_json_field(),
    ]
}

pub fn marginal_invariant_public_artifact_json_fields() -> [&'static str; 8] {
    [
        "\"experiment\": \"-lem-m2-marginal-invariant-smoke\"",
        "\"artifact_scope\": \"public audit artifact only\"",
        "\"source_track\": \"impl/polar_validation Rust\"",
        "\"python_research_harness_dependency\": false",
        "\"implementation_posture\": \"Rust-side smoke guard; not production constant-time evidence\"",
        "\"checked_invariant\": \"B A equals sampled C\"",
        "\"out_of_scope\": \"statistical indistinguishability, reduction, and security claims\"",
        "\"screen\": \"public marginal-invariant algebra guard for B=section_A(C)+K with K A=0\"",
    ]
}

pub fn marginal_invariant_public_artifact_json_lines() -> &'static str {
    marginal_invariant_public_artifact_json_lines!()
}

pub fn scl_work_shape_public_artifact_json_fields() -> [&'static str; 6] {
    [
        "\"experiment\": \"-polar-scl-workshape-audit\"",
        "\"ct_surface\": \"ct-003\"",
        "\"component\": \"impl/polar_validation SCL decoder\"",
        "\"source_track\": \"impl/polar_validation Rust\"",
        "\"python_research_harness_dependency\": false",
        "\"artifact_scope\": \"public audit artifact only\"",
    ]
}

pub fn scl_work_shape_public_artifact_json_lines() -> &'static str {
    scl_work_shape_public_artifact_json_lines!()
}

pub fn marginal_invariant_claim_posture_json_fields() -> [&'static str; 6] {
    [
        "\"status\": \"OPEN\"",
        "\"production_constant_time_claim\": false",
        "\"reduction_claim\": false",
        "\"security_claim\": false",
        "\"reduction\": \"NOT A REDUCTION\"",
        "\"security\": \"NOT A SECURITY CLAIM\"",
    ]
}

pub fn marginal_invariant_claim_posture_json_lines() -> &'static str {
    marginal_invariant_claim_posture_json_lines!()
}

pub fn marginal_invariant_audit_boundary_json_lines() -> &'static str {
    marginal_invariant_audit_boundary_json_lines!()
}

pub fn marginal_invariant_adjudication_json_field() -> &'static str {
    marginal_invariant_adjudication_json_field!()
}

pub fn scl_work_shape_claim_posture_json_fields() -> [&'static str; 5] {
    [
        "\"status\": \"audit boundary only; current decoder is variable-shape reference code\"",
        "\"current_verdict\": \"not_constant_time\"",
        "\"production_constant_time_claim\": false",
        "\"reduction_claim\": false",
        "\"security_claim\": false",
    ]
}

pub fn scl_work_shape_claim_posture_json_lines() -> &'static str {
    scl_work_shape_claim_posture_json_lines!()
}

pub fn scl_work_shape_audit_boundary_json_lines() -> &'static str {
    scl_work_shape_audit_boundary_json_lines!()
}

pub fn scl_work_shape_adjudication_json_field() -> &'static str {
    scl_work_shape_adjudication_json_field!()
}

pub fn scl_work_shape_required_action_json_field() -> &'static str {
    scl_work_shape_required_action_json_field!()
}

pub fn scl_work_shape_closure_boundary_json_lines() -> &'static str {
    scl_work_shape_closure_boundary_json_lines!()
}

pub fn scl_related_public_screen_json_fields() -> [&'static str; 10] {
    [
        "\"experiment\": \"-lem-m2-marginal-invariant-smoke\"",
        "\"source_track\": \"impl/polar_validation Rust\"",
        "\"python_research_harness_dependency\": false",
        "\"artifact_scope\": \"public audit artifact only\"",
        "\"status\": \"OPEN\"",
        "\"production_constant_time_claim\": false",
        "\"reduction_claim\": false",
        "\"security_claim\": false",
        "\"checked_invariant\": \"B A equals sampled C\"",
        "\"out_of_scope\": \"statistical indistinguishability, reduction, and security claims\"",
    ]
}

pub fn scl_related_public_screen_json_lines() -> &'static str {
    scl_related_public_screen_json_lines!()
}

pub fn scl_related_public_screens_json_lines() -> &'static str {
    scl_related_public_screens_json_lines!()
}

pub fn scl_work_shape_screen_linkage_json_lines() -> &'static str {
    scl_work_shape_screen_linkage_json_lines!()
}

pub fn scl_work_shape_related_ofa_evidence_json_lines() -> &'static str {
    scl_work_shape_related_ofa_evidence_json_lines!()
}

pub fn scl_work_shape_audited_functions_json_lines() -> &'static str {
    scl_work_shape_audited_functions_json_lines!()
}

pub fn scl_work_shape_active_decoder_entrypoints_json_lines() -> &'static str {
    scl_work_shape_active_decoder_entrypoints_json_lines!()
}

pub fn scl_work_shape_variable_shape_surfaces_json_lines() -> &'static str {
    scl_work_shape_variable_shape_surfaces_json_lines!()
}

pub fn scl_work_shape_fixed_schedule_requirements_json_lines() -> &'static str {
    scl_work_shape_fixed_schedule_requirements_json_lines!()
}

pub fn scl_work_shape_metric_domain_assumptions_json_lines() -> &'static str {
    scl_work_shape_metric_domain_assumptions_json_lines!()
}

pub fn scl_work_shape_top_l_selection_domain_failure_codes_json_lines() -> &'static str {
    scl_work_shape_top_l_selection_domain_failure_codes_json_lines!()
}

pub fn scl_work_shape_public_path_domain_failure_codes_json_lines() -> &'static str {
    scl_work_shape_public_path_domain_failure_codes_json_lines!()
}

pub fn scl_work_shape_public_child_write_failure_codes_json_lines() -> &'static str {
    scl_work_shape_public_child_write_failure_codes_json_lines!()
}

pub fn scl_work_shape_integer_schedule_domain_failure_codes_json_lines() -> &'static str {
    scl_work_shape_integer_schedule_domain_failure_codes_json_lines!()
}

pub fn scl_work_shape_integer_metric_domain_failure_codes_json_lines() -> &'static str {
    scl_work_shape_integer_metric_domain_failure_codes_json_lines!()
}

pub fn scl_work_shape_integer_schedule_shape_failure_families_json_lines() -> &'static str {
    scl_work_shape_integer_schedule_shape_failure_families_json_lines!()
}

pub fn scl_work_shape_public_round_schedule_shape_failure_families_json_lines() -> &'static str {
    scl_work_shape_public_round_schedule_shape_failure_families_json_lines!()
}

pub fn scl_work_shape_non_panicking_wrapper_failure_code_map_json_lines() -> &'static str {
    scl_work_shape_non_panicking_wrapper_failure_code_map_json_lines!()
}

pub fn scl_work_shape_prototype_building_blocks_json_lines() -> &'static str {
    scl_work_shape_prototype_building_blocks_json_lines!()
}

pub fn scl_work_shape_fixed_work_count_guardrails_json_lines() -> &'static str {
    scl_work_shape_fixed_work_count_guardrails_json_lines!()
}

pub fn scl_work_shape_status_selection_guardrails_json_lines() -> &'static str {
    scl_work_shape_status_selection_guardrails_json_lines!()
}

pub fn scl_work_shape_public_domain_guardrails_json_lines() -> &'static str {
    scl_work_shape_public_domain_guardrails_json_lines!()
}

pub fn scl_work_shape_public_status_field_provenance_json_lines() -> &'static str {
    scl_work_shape_public_status_field_provenance_json_lines!()
}

pub fn scl_work_shape_public_status_selector_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_selector_obligations_json_lines!()
}

pub fn scl_work_shape_public_shape_preservation_obligations_json_lines() -> &'static str {
    scl_work_shape_public_shape_preservation_obligations_json_lines!()
}

pub fn scl_work_shape_public_failure_code_wrapper_obligations_json_lines() -> &'static str {
    scl_work_shape_public_failure_code_wrapper_obligations_json_lines!()
}

pub fn scl_work_shape_public_preflight_coverage_obligations_json_lines() -> &'static str {
    scl_work_shape_public_preflight_coverage_obligations_json_lines!()
}

pub fn scl_work_shape_public_invalid_input_status_reporting_obligations_json_lines() -> &'static str
{
    scl_work_shape_public_invalid_input_status_reporting_obligations_json_lines!()
}

pub fn scl_work_shape_public_rejection_reason_taxonomy_json_lines() -> &'static str {
    scl_work_shape_public_rejection_reason_taxonomy_json_lines!()
}

pub fn scl_work_shape_public_status_lifecycle_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_lifecycle_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_transition_ordering_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_transition_ordering_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_stability_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_stability_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_compatibility_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_compatibility_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_consumer_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_consumer_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_producer_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_producer_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_round_trip_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_round_trip_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_drift_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_drift_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_migration_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_migration_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_rollback_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_rollback_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_deprecation_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_deprecation_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_removal_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_removal_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_archival_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_archival_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_sunset_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_sunset_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_retention_obligations_json_lines() -> &'static str {
    scl_work_shape_public_status_schema_retention_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_lifecycle_order_obligations_json_lines() -> &'static str
{
    scl_work_shape_public_status_schema_lifecycle_order_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_lifecycle_shape_obligations_json_lines() -> &'static str
{
    scl_work_shape_public_status_schema_lifecycle_shape_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_lifecycle_registry_obligations_json_lines(
) -> &'static str {
    scl_work_shape_public_status_schema_lifecycle_registry_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_lifecycle_manifest_obligations_json_lines(
) -> &'static str {
    scl_work_shape_public_status_schema_lifecycle_manifest_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_lifecycle_replay_obligations_json_lines() -> &'static str
{
    scl_work_shape_public_status_schema_lifecycle_replay_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_lifecycle_digest_obligations_json_lines() -> &'static str
{
    scl_work_shape_public_status_schema_lifecycle_digest_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_lifecycle_receipt_obligations_json_lines() -> &'static str
{
    scl_work_shape_public_status_schema_lifecycle_receipt_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_lifecycle_transcript_obligations_json_lines(
) -> &'static str {
    scl_work_shape_public_status_schema_lifecycle_transcript_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_lifecycle_witness_obligations_json_lines() -> &'static str
{
    scl_work_shape_public_status_schema_lifecycle_witness_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_lifecycle_audit_trail_obligations_json_lines(
) -> &'static str {
    scl_work_shape_public_status_schema_lifecycle_audit_trail_obligations_json_lines!()
}

pub fn scl_work_shape_public_status_schema_lifecycle_checkpoint_obligations_json_lines(
) -> &'static str {
    scl_work_shape_public_status_schema_lifecycle_checkpoint_obligations_json_lines!()
}

pub fn ofa_evidence_scope_json_field() -> &'static str {
    ofa_evidence_scope_json_field!()
}

pub fn ofa_evidence_runtime_dependency_json_field() -> &'static str {
    ofa_evidence_runtime_dependency_json_field!()
}

pub fn ofa_evidence_paper_claim_dependency_json_field() -> &'static str {
    ofa_evidence_paper_claim_dependency_json_field!()
}

pub fn ofa_evidence_audit_posture_json_fields() -> [&'static str; 3] {
    [
        ofa_evidence_scope_json_field(),
        ofa_evidence_runtime_dependency_json_field(),
        ofa_evidence_paper_claim_dependency_json_field(),
    ]
}

pub fn ofa_evidence_entry_scope_json_field() -> &'static str {
    ofa_evidence_entry_scope_json_field!()
}

pub fn ofa_evidence_entry_runtime_dependency_json_field() -> &'static str {
    ofa_evidence_entry_runtime_dependency_json_field!()
}

pub fn ofa_evidence_entry_paper_claim_dependency_json_field() -> &'static str {
    ofa_evidence_entry_paper_claim_dependency_json_field!()
}

pub fn ofa_evidence_entry_use_in_impl_json_field() -> &'static str {
    ofa_evidence_entry_use_in_impl_json_field!()
}

pub fn ofa_evidence_entry_audit_posture_json_fields() -> [&'static str; 4] {
    [
        ofa_evidence_entry_scope_json_field(),
        ofa_evidence_entry_paper_claim_dependency_json_field(),
        ofa_evidence_entry_runtime_dependency_json_field(),
        ofa_evidence_entry_use_in_impl_json_field(),
    ]
}

pub fn ofa_evidence_audit_posture_json_lines() -> &'static str {
    ofa_evidence_audit_posture_json_lines!()
}

pub fn ofa_evidence_entry_audit_posture_json_lines() -> &'static str {
    ofa_evidence_entry_audit_posture_json_lines!()
}

pub fn ofa_evidence_identity_json_fields(entry: OfaEvidenceEntry) -> [String; 3] {
    [
        format!("\"ofa\": \"{}\"", entry.ofa),
        format!("\"finding\": \"{}\"", entry.finding),
        format!("\"claim_posture\": \"{}\"", entry.claim_posture),
    ]
}

pub fn ofa_evidence_entry_json_fields(entry: OfaEvidenceEntry) -> [String; 7] {
    [
        format!("\"ofa\": \"{}\"", entry.ofa),
        format!("\"finding\": \"{}\"", entry.finding),
        format!("\"claim_posture\": \"{}\"", entry.claim_posture),
        format!("\"evidence_scope\": \"{}\"", entry.evidence_scope),
        format!(
            "\"paper_claim_dependency\": {}",
            entry.paper_claim_dependency
),
        format!("\"runtime_dependency\": {}", entry.runtime_dependency),
        format!("\"use_in_impl\": \"{}\"", entry.use_in_impl),
    ]
}

#[derive(Clone, Debug)]
pub struct PolarCode {
    pub n: usize,
    pub k: usize,
    pub p: f64,
    pub frozen: Vec<usize>,
    pub info_set: Vec<usize>,
    frozen_mask: Vec<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimulationResult {
    pub n: usize,
    pub k: usize,
    pub p: f64,
    pub trials: usize,
    pub errors: usize,
    pub seed: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FixedI64DecoderAgreement {
    pub n: usize,
    pub k: usize,
    pub p: f64,
    pub trials: usize,
    pub decoded_mismatches: usize,
    pub fast_errors: usize,
    pub fixed_errors: usize,
    pub seed: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportanceSamplingResult {
    pub n: usize,
    pub k: usize,
    pub target_p: f64,
    pub proposal_p: f64,
    pub trials: usize,
    pub proposal_errors: usize,
    pub weighted_bler_estimate: f64,
    pub mean_likelihood_ratio: f64,
    pub effective_sample_size: f64,
    pub seed: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PolarRateRow {
    pub n: usize,
    pub k: usize,
    pub p: f64,
    pub rate: f64,
    pub bhattacharyya_sum_bound: f64,
    pub half_sum_bound: f64,
    pub log2_bhattacharyya_sum_bound: f64,
    pub log2_half_sum_bound: f64,
    pub target_log2_half_sum_bound: f64,
    pub passes_half_sum_target: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedTopLEntry {
    pub metric: i64,
    pub index: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclCandidate<const N: usize> {
    pub metric: i64,
    pub bits: [u8; N],
    pub active: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclRound {
    pub bit_index: usize,
    pub bit0_metric_delta: i64,
    pub bit1_metric_delta: i64,
}

pub const FIXED_SCL_FORBIDDEN_METRIC_DELTA: i64 = i64::MAX;
pub const FIXED_SCL_NO_INVALID_ROUND: usize = usize::MAX;
pub const FIXED_SCL_PATH_DOMAIN_OK: u8 = 0;
pub const FIXED_SCL_PATH_DOMAIN_EMPTY_SCHEDULE: u8 = 1;
pub const FIXED_SCL_PATH_DOMAIN_FIRST_CHILD_CAPACITY: u8 = 2;
pub const FIXED_SCL_PATH_DOMAIN_REPEATED_CHILD_CAPACITY: u8 = 3;
pub const FIXED_SCL_PATH_DOMAIN_TOP_L_WIDTH: u8 = 4;
pub const FIXED_SCL_PATH_DOMAIN_BIT_INDEX: u8 = 5;
pub const FIXED_SCL_CHILD_WRITE_DOMAIN_OK: u8 = 0;
pub const FIXED_SCL_CHILD_WRITE_DOMAIN_PARENT_SLOT: u8 = 1;
pub const FIXED_SCL_CHILD_WRITE_DOMAIN_DST_CAPACITY: u8 = 2;
pub const FIXED_SCL_CHILD_WRITE_DOMAIN_BIT_INDEX: u8 = 3;
pub const FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_OK: u8 = 0;
pub const FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_HARD_BIT: u8 = 1;
pub const FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_MAGNITUDE: u8 = 2;
pub const FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_FAMILY_OK: u8 = 0;
pub const FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_FAMILY_INTEGER_DOMAIN: u8 = 1;
pub const FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_FAMILY_PATH_DOMAIN: u8 = 2;
pub const FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_FAMILY_WORK_SHAPE: u8 = 3;
pub const FIXED_SCL_PUBLIC_ROUND_SCHEDULE_SHAPE_FAILURE_FAMILY_OK: u8 = 0;
pub const FIXED_SCL_PUBLIC_ROUND_SCHEDULE_SHAPE_FAILURE_FAMILY_PATH_DOMAIN: u8 = 1;
pub const FIXED_SCL_PUBLIC_ROUND_SCHEDULE_SHAPE_FAILURE_FAMILY_WORK_SHAPE: u8 = 2;
pub const FIXED_TOP_L_SELECTION_DOMAIN_OK: u8 = 0;
pub const FIXED_TOP_L_SELECTION_DOMAIN_WIDTH: u8 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedScheduleTopLSelectionDomainFailureLabel {
    pub code: u8,
    pub name: &'static str,
    pub meaning: &'static str,
}

pub const FIXED_TOP_L_SELECTION_DOMAIN_FAILURE_LABELS:
    [FixedScheduleTopLSelectionDomainFailureLabel; 2] = [
    FixedScheduleTopLSelectionDomainFailureLabel {
        code: FIXED_TOP_L_SELECTION_DOMAIN_OK,
        name: "ok",
        meaning: "valid public top-L selection shape",
    },
    FixedScheduleTopLSelectionDomainFailureLabel {
        code: FIXED_TOP_L_SELECTION_DOMAIN_WIDTH,
        name: "width",
        meaning: "list size must be no larger than selection width",
    },
];

pub fn fixed_top_l_selection_domain_failure_label(code: u8) -> &'static str {
    for label in FIXED_TOP_L_SELECTION_DOMAIN_FAILURE_LABELS {
        if label.code == code {
            return label.name;
        }
    }
    "unknown"
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerScheduleDomainFailureLabel {
    pub code: u8,
    pub name: &'static str,
    pub meaning: &'static str,
}

pub const FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_FAILURE_LABELS:
    [FixedSclIntegerScheduleDomainFailureLabel; 3] = [
    FixedSclIntegerScheduleDomainFailureLabel {
        code: FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_OK,
        name: "ok",
        meaning: "valid public integer schedule inputs",
    },
    FixedSclIntegerScheduleDomainFailureLabel {
        code: FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_HARD_BIT,
        name: "hard_bit",
        meaning: "hard decisions must be public bits",
    },
    FixedSclIntegerScheduleDomainFailureLabel {
        code: FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_MAGNITUDE,
        name: "magnitude",
        meaning: "integer metric magnitudes must be non-negative",
    },
];

pub fn fixed_scl_integer_schedule_domain_failure_label(code: u8) -> &'static str {
    for label in FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_FAILURE_LABELS {
        if label.code == code {
            return label.name;
        }
    }
    "unknown"
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerMetricDomainFailureLabel {
    pub code: u8,
    pub name: &'static str,
    pub meaning: &'static str,
}

pub const FIXED_SCL_INTEGER_METRIC_DOMAIN_FAILURE_LABELS:
    [FixedSclIntegerMetricDomainFailureLabel; 3] = [
    FixedSclIntegerMetricDomainFailureLabel {
        code: FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_OK,
        name: "ok",
        meaning: "valid public integer metric inputs",
    },
    FixedSclIntegerMetricDomainFailureLabel {
        code: FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_HARD_BIT,
        name: "hard_bit",
        meaning: "hard decisions must be public bits",
    },
    FixedSclIntegerMetricDomainFailureLabel {
        code: FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_MAGNITUDE,
        name: "magnitude",
        meaning: "integer metric magnitudes must be non-negative",
    },
];

pub fn fixed_scl_integer_metric_domain_failure_label(code: u8) -> &'static str {
    for label in FIXED_SCL_INTEGER_METRIC_DOMAIN_FAILURE_LABELS {
        if label.code == code {
            return label.name;
        }
    }
    "unknown"
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerScheduleShapeFailureLabel {
    pub code: u8,
    pub name: &'static str,
    pub meaning: &'static str,
}

pub const FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_LABELS:
    [FixedSclIntegerScheduleShapeFailureLabel; 4] = [
    FixedSclIntegerScheduleShapeFailureLabel {
        code: FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_FAMILY_OK,
        name: "ok",
        meaning: "valid integer schedule-shape preflight",
    },
    FixedSclIntegerScheduleShapeFailureLabel {
        code: FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_FAMILY_INTEGER_DOMAIN,
        name: "integer_domain",
        meaning: "integer hard-bit or metric-magnitude domain failed first",
    },
    FixedSclIntegerScheduleShapeFailureLabel {
        code: FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_FAMILY_PATH_DOMAIN,
        name: "path_domain",
        meaning: "public path-buffer schedule domain failed first",
    },
    FixedSclIntegerScheduleShapeFailureLabel {
        code: FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_FAMILY_WORK_SHAPE,
        name: "work_shape",
        meaning: "public top-L work-shape envelope failed after domain checks",
    },
];

pub fn fixed_scl_integer_schedule_shape_failure_label(code: u8) -> &'static str {
    for label in FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_LABELS {
        if label.code == code {
            return label.name;
        }
    }
    "unknown"
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclPublicRoundScheduleShapeFailureLabel {
    pub code: u8,
    pub name: &'static str,
    pub meaning: &'static str,
}

pub const FIXED_SCL_PUBLIC_ROUND_SCHEDULE_SHAPE_FAILURE_LABELS:
    [FixedSclPublicRoundScheduleShapeFailureLabel; 3] = [
    FixedSclPublicRoundScheduleShapeFailureLabel {
        code: FIXED_SCL_PUBLIC_ROUND_SCHEDULE_SHAPE_FAILURE_FAMILY_OK,
        name: "ok",
        meaning: "valid public schedule-shape preflight",
    },
    FixedSclPublicRoundScheduleShapeFailureLabel {
        code: FIXED_SCL_PUBLIC_ROUND_SCHEDULE_SHAPE_FAILURE_FAMILY_PATH_DOMAIN,
        name: "path_domain",
        meaning: "public path-buffer schedule domain failed first",
    },
    FixedSclPublicRoundScheduleShapeFailureLabel {
        code: FIXED_SCL_PUBLIC_ROUND_SCHEDULE_SHAPE_FAILURE_FAMILY_WORK_SHAPE,
        name: "work_shape",
        meaning: "public top-L work-shape envelope failed after path-domain checks",
    },
];

pub fn fixed_scl_public_round_schedule_shape_failure_label(code: u8) -> &'static str {
    for label in FIXED_SCL_PUBLIC_ROUND_SCHEDULE_SHAPE_FAILURE_LABELS {
        if label.code == code {
            return label.name;
        }
    }
    "unknown"
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclChildWriteDomainFailureLabel {
    pub code: u8,
    pub name: &'static str,
    pub meaning: &'static str,
}

pub const FIXED_SCL_CHILD_WRITE_DOMAIN_FAILURE_LABELS: [FixedSclChildWriteDomainFailureLabel; 4] = [
    FixedSclChildWriteDomainFailureLabel {
        code: FIXED_SCL_CHILD_WRITE_DOMAIN_OK,
        name: "ok",
        meaning: "valid public fixed child-write domain",
    },
    FixedSclChildWriteDomainFailureLabel {
        code: FIXED_SCL_CHILD_WRITE_DOMAIN_PARENT_SLOT,
        name: "parent_slot",
        meaning: "parent slot must be inside the fixed parent buffer",
    },
    FixedSclChildWriteDomainFailureLabel {
        code: FIXED_SCL_CHILD_WRITE_DOMAIN_DST_CAPACITY,
        name: "dst_capacity",
        meaning: "destination child buffer must have room for both children",
    },
    FixedSclChildWriteDomainFailureLabel {
        code: FIXED_SCL_CHILD_WRITE_DOMAIN_BIT_INDEX,
        name: "bit_index",
        meaning: "public bit index must be inside the path bit width",
    },
];

pub fn fixed_scl_child_write_domain_failure_label(code: u8) -> &'static str {
    for label in FIXED_SCL_CHILD_WRITE_DOMAIN_FAILURE_LABELS {
        if label.code == code {
            return label.name;
        }
    }
    "unknown"
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclPathDomainFailureLabel {
    pub code: u8,
    pub name: &'static str,
    pub meaning: &'static str,
}

pub const FIXED_SCL_PATH_DOMAIN_FAILURE_LABELS: [FixedSclPathDomainFailureLabel; 6] = [
    FixedSclPathDomainFailureLabel {
        code: FIXED_SCL_PATH_DOMAIN_OK,
        name: "ok",
        meaning: "valid public path-buffer schedule shape",
    },
    FixedSclPathDomainFailureLabel {
        code: FIXED_SCL_PATH_DOMAIN_EMPTY_SCHEDULE,
        name: "empty_schedule",
        meaning: "round schedule must contain at least one public round",
    },
    FixedSclPathDomainFailureLabel {
        code: FIXED_SCL_PATH_DOMAIN_FIRST_CHILD_CAPACITY,
        name: "first_child_capacity",
        meaning: "first child buffer must hold two children per parent slot",
    },
    FixedSclPathDomainFailureLabel {
        code: FIXED_SCL_PATH_DOMAIN_REPEATED_CHILD_CAPACITY,
        name: "repeated_child_capacity",
        meaning: "repeated child buffer must hold two children per compacted path",
    },
    FixedSclPathDomainFailureLabel {
        code: FIXED_SCL_PATH_DOMAIN_TOP_L_WIDTH,
        name: "top_l_width",
        meaning: "list size must fit the parent and child selection widths",
    },
    FixedSclPathDomainFailureLabel {
        code: FIXED_SCL_PATH_DOMAIN_BIT_INDEX,
        name: "bit_index",
        meaning: "every public bit index must be inside the path bit width",
    },
];

pub fn fixed_scl_path_domain_failure_label(code: u8) -> &'static str {
    for label in FIXED_SCL_PATH_DOMAIN_FAILURE_LABELS {
        if label.code == code {
            return label.name;
        }
    }
    "unknown"
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclMetricDeltas {
    pub bit0_metric_delta: i64,
    pub bit1_metric_delta: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerMetricDomainCheck {
    pub valid: bool,
    pub failure_code: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerMetricDeltaRun {
    pub domain_check: FixedSclIntegerMetricDomainCheck,
    pub deltas: FixedSclMetricDeltas,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedScheduleTopLSelectionPlan {
    pub width: usize,
    pub list_size: usize,
    pub valid: bool,
    pub failure_code: u8,
    pub compare_exchanges: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerScheduleDomainCheck {
    pub rounds: usize,
    pub valid: bool,
    pub failure_code: u8,
    pub first_invalid_round: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerRoundScheduleBuild<const ROUNDS: usize> {
    pub domain_check: FixedSclIntegerScheduleDomainCheck,
    pub round_slots_written: usize,
    pub rounds: [FixedSclRound; ROUNDS],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerRoundScheduleBuildPlan {
    pub domain_check: FixedSclIntegerScheduleDomainCheck,
    pub round_slots_written: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerRoundScheduleBuildParityCheck {
    pub matches: bool,
    pub run_build_certificate: FixedSclIntegerRoundScheduleBuildPlan,
    pub expected_plan: FixedSclIntegerRoundScheduleBuildPlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclPathBufferScheduleDomainCheck {
    pub parent_capacity: usize,
    pub first_child_capacity: usize,
    pub repeated_child_capacity: usize,
    pub list_size: usize,
    pub rounds: usize,
    pub bit_width: usize,
    pub valid: bool,
    pub failure_code: u8,
    pub first_invalid_round: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclPublicRoundSchedulePlan {
    pub path_domain_check: FixedSclPathBufferScheduleDomainCheck,
    pub work_counts: FixedSclPublicRoundWorkCounts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclRoundSchedulePlanParityCheck {
    pub matches: bool,
    pub round_schedule_plan: FixedSclPublicRoundSchedulePlan,
    pub expected_public_plan: FixedSclPublicRoundSchedulePlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclRoundScheduleShapeParityCheck {
    pub matches: bool,
    pub round_shape_plan: FixedSclPublicRoundScheduleShapePlan,
    pub expected_shape_plan: FixedSclPublicRoundScheduleShapePlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclPublicRoundScheduleShapePlan {
    pub valid: bool,
    pub path_domain_check: FixedSclPathBufferScheduleDomainCheck,
    pub work_shape_plan: FixedSclPublicRoundWorkShapePlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclPublicRoundShapeParityCheck {
    pub matches: bool,
    pub run_shape_certificate: FixedSclPublicRoundScheduleShapePlan,
    pub expected_shape_plan: FixedSclPublicRoundScheduleShapePlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclOneBitShapeParityCheck {
    pub matches: bool,
    pub run_plan_certificate: FixedSclPublicRoundSchedulePlan,
    pub expected_plan: FixedSclPublicRoundSchedulePlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerRoundSchedulePlan {
    pub domain_check: FixedSclIntegerScheduleDomainCheck,
    pub path_domain_check: FixedSclPathBufferScheduleDomainCheck,
    pub work_counts: FixedSclPublicRoundWorkCounts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerRoundScheduleShapePlan {
    pub valid: bool,
    pub domain_check: FixedSclIntegerScheduleDomainCheck,
    pub path_domain_check: FixedSclPathBufferScheduleDomainCheck,
    pub work_shape_plan: FixedSclPublicRoundWorkShapePlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerShapeParityCheck {
    pub matches: bool,
    pub run_plan_certificate: FixedSclIntegerRoundSchedulePlan,
    pub expected_plan: FixedSclIntegerRoundSchedulePlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclIntegerScheduleShapeParityCheck {
    pub matches: bool,
    pub run_shape_certificate: FixedSclIntegerRoundScheduleShapePlan,
    pub expected_shape_plan: FixedSclIntegerRoundScheduleShapePlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclBinaryChildWriteDomainCheck {
    pub parent_capacity: usize,
    pub child_capacity: usize,
    pub bit_width: usize,
    pub parent_slot: usize,
    pub dst_start: usize,
    pub bit_index: usize,
    pub child_slots_written: usize,
    pub valid: bool,
    pub failure_code: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclChildWriteParityCheck {
    pub matches: bool,
    pub run_domain_check: FixedSclBinaryChildWriteDomainCheck,
    pub expected_domain_check: FixedSclBinaryChildWriteDomainCheck,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclOneBitExpansionRun<const CHILD_CAP: usize, const L: usize, const N: usize> {
    pub path_domain_check: FixedSclPathBufferScheduleDomainCheck,
    pub work_counts: FixedSclPublicRoundWorkCounts,
    pub children: FixedSclPathBuffer<CHILD_CAP, N>,
    pub top: [FixedTopLEntry; L],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclPublicRoundScheduleRun<const L: usize, const N: usize> {
    pub path_domain_check: FixedSclPathBufferScheduleDomainCheck,
    pub work_counts: FixedSclPublicRoundWorkCounts,
    pub paths: FixedSclPathBuffer<L, N>,
    pub top: [FixedTopLEntry; L],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclPathBufferIntegerScheduleRun<const L: usize, const N: usize> {
    pub domain_check: FixedSclIntegerScheduleDomainCheck,
    pub path_domain_check: FixedSclPathBufferScheduleDomainCheck,
    pub work_counts: FixedSclPublicRoundWorkCounts,
    pub paths: FixedSclPathBuffer<L, N>,
    pub top: [FixedTopLEntry; L],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclPublicRoundWorkCounts {
    pub parent_capacity: usize,
    pub first_child_capacity: usize,
    pub repeated_child_capacity: usize,
    pub list_size: usize,
    pub rounds: usize,
    pub top_l_compare_exchanges: usize,
    pub child_slots_written: usize,
    pub compacted_slots_written: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclPublicRoundWorkShapePlan {
    pub parent_capacity: usize,
    pub first_child_capacity: usize,
    pub repeated_child_capacity: usize,
    pub list_size: usize,
    pub rounds: usize,
    pub valid: bool,
    pub first_top_l_plan: FixedScheduleTopLSelectionPlan,
    pub repeated_top_l_plan: FixedScheduleTopLSelectionPlan,
    pub work_counts: FixedSclPublicRoundWorkCounts,
}

impl FixedSclRound {
    pub const fn new(bit_index: usize, bit0_metric_delta: i64, bit1_metric_delta: i64) -> Self {
        Self {
            bit_index,
            bit0_metric_delta,
            bit1_metric_delta,
        }
    }
}

impl<const N: usize> FixedSclCandidate<N> {
    pub const EMPTY: Self = Self {
        metric: i64::MAX,
        bits: [0; N],
        active: 0,
    };

    pub fn effective_metric(self) -> i64 {
        let active = self.active & 1;
        let active_i64_mask = 0i64.wrapping_sub(i64::from(active));
        select_i64(active_i64_mask, i64::MAX, self.metric)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedSclPathBuffer<const CAP: usize, const N: usize> {
    slots: [FixedSclCandidate<N>; CAP],
}

impl<const CAP: usize, const N: usize> FixedSclPathBuffer<CAP, N> {
    pub fn new() -> Self {
        Self {
            slots: [FixedSclCandidate::EMPTY; CAP],
        }
    }

    pub fn capacity(&self) -> usize {
        CAP
    }

    pub fn bit_width(&self) -> usize {
        N
    }

    pub fn active_count(&self) -> usize {
        self.slots
.iter()
.map(|candidate| usize::from(candidate.active & 1))
.sum()
    }

    pub fn set_candidate(&mut self, slot: usize, metric: i64, bits: [u8; N]) {
        assert!(slot < CAP, "SCL fixed path slot is outside capacity");
        self.slots[slot] = FixedSclCandidate {
            metric,
            bits,
            active: 1,
        };
    }

    pub fn clear_slot(&mut self, slot: usize) {
        assert!(slot < CAP, "SCL fixed path slot is outside capacity");
        self.slots[slot] = FixedSclCandidate::EMPTY;
    }

    pub fn bits(&self, slot: usize) -> [u8; N] {
        assert!(slot < CAP, "SCL fixed path slot is outside capacity");
        self.slots[slot].bits
    }

    pub fn metric_entries(&self) -> [FixedTopLEntry; CAP] {
        let mut entries = [FixedTopLEntry {
            metric: i64::MAX,
            index: usize::MAX,
        }; CAP];
        for (index, entry) in entries.iter_mut().enumerate() {
            *entry = FixedTopLEntry {
                metric: self.slots[index].effective_metric(),
                index,
            };
        }
        entries
    }

    pub fn top_l_entries<const L: usize>(&self) -> [FixedTopLEntry; L] {
        let entries = self.metric_entries();
        let mut metrics = [i64::MAX; CAP];
        for (index, metric) in metrics.iter_mut().enumerate() {
            *metric = entries[index].metric;
        }
        fixed_schedule_top_l_i64::<CAP, L>(metrics)
    }

    fn top_l_entries_padded<const L: usize>(&self) -> [FixedTopLEntry; L] {
        let mut entries = self.metric_entries();
        for i in 0..CAP {
            for j in (i + 1)..CAP {
                fixed_compare_exchange(&mut entries, i, j);
            }
        }

        let mut top = [FixedTopLEntry {
            metric: i64::MAX,
            index: usize::MAX,
        }; L];
        for (dst_index, top_entry) in top.iter_mut().enumerate() {
            let mut selected = FixedTopLEntry {
                metric: i64::MAX,
                index: usize::MAX,
            };
            for (src_index, entry) in entries.iter().enumerate() {
                let take = usize::from(dst_index == src_index);
                let take_usize_mask = 0usize.wrapping_sub(take);
                let take_i64_mask = 0i64.wrapping_sub(take as i64);
                selected = select_top_l_entry(take_usize_mask, take_i64_mask, selected, *entry);
            }
            *top_entry = selected;
        }
        top
    }

    pub fn write_binary_children_from<const SRC_CAP: usize>(
        &mut self,
        parents: &FixedSclPathBuffer<SRC_CAP, N>,
        parent_slot: usize,
        dst_start: usize,
        bit_index: usize,
        bit0_metric_delta: i64,
        bit1_metric_delta: i64,
) {
        assert!(
            parent_slot < SRC_CAP,
            "binary child parent slot is outside capacity"
);
        assert!(
            dst_start + 1 < CAP,
            "binary child destination requires two slots"
);
        assert!(bit_index < N, "binary child bit index is outside width");

        let parent = parents.slots[parent_slot];
        let mut bit0 = parent.bits;
        let mut bit1 = parent.bits;
        bit0[bit_index] = 0;
        bit1[bit_index] = 1;

        let active = parent.active & 1;
        let active_i64_mask = 0i64.wrapping_sub(i64::from(active));
        let active_u8_mask = 0u8.wrapping_sub(active);

        self.slots[dst_start] = FixedSclCandidate {
            metric: select_i64(
                active_i64_mask,
                i64::MAX,
                fixed_scl_metric_add(parent.metric, bit0_metric_delta),
),
            bits: mask_bits(bit0, active_u8_mask),
            active,
        };
        self.slots[dst_start + 1] = FixedSclCandidate {
            metric: select_i64(
                active_i64_mask,
                i64::MAX,
                fixed_scl_metric_add(parent.metric, bit1_metric_delta),
),
            bits: mask_bits(bit1, active_u8_mask),
            active,
        };
    }

    pub fn try_write_binary_children_from<const SRC_CAP: usize>(
        &mut self,
        parents: &FixedSclPathBuffer<SRC_CAP, N>,
        parent_slot: usize,
        dst_start: usize,
        bit_index: usize,
        bit0_metric_delta: i64,
        bit1_metric_delta: i64,
) -> FixedSclBinaryChildWriteDomainCheck {
        let domain_check = fixed_scl_binary_child_write_domain_check::<SRC_CAP, CAP, N>(
            parent_slot,
            dst_start,
            bit_index,
);

        let valid_usize = usize::from(domain_check.valid);
        let mut parent = FixedSclCandidate::EMPTY;
        for index in 0..SRC_CAP {
            let take = usize::from(index == parent_slot);
            let take_i64_mask = 0i64.wrapping_sub(take as i64);
            let take_u8_mask = 0u8.wrapping_sub(take as u8);
            parent = select_candidate(take_i64_mask, take_u8_mask, parent, parents.slots[index]);
        }

        let mut bit0 = parent.bits;
        let mut bit1 = parent.bits;
        for index in 0..N {
            let take = u8::from(index == bit_index);
            let take_u8_mask = 0u8.wrapping_sub(take);
            bit0[index] = select_u8(take_u8_mask, bit0[index], 0);
            bit1[index] = select_u8(take_u8_mask, bit1[index], 1);
        }

        let active = parent.active & 1;
        let active_i64_mask = 0i64.wrapping_sub(i64::from(active));
        let active_u8_mask = 0u8.wrapping_sub(active);
        let child0 = FixedSclCandidate {
            metric: select_i64(
                active_i64_mask,
                i64::MAX,
                fixed_scl_metric_add(parent.metric, bit0_metric_delta),
),
            bits: mask_bits(bit0, active_u8_mask),
            active,
        };
        let child1 = FixedSclCandidate {
            metric: select_i64(
                active_i64_mask,
                i64::MAX,
                fixed_scl_metric_add(parent.metric, bit1_metric_delta),
),
            bits: mask_bits(bit1, active_u8_mask),
            active,
        };

        let second_dst = dst_start.saturating_add(1);
        for index in 0..CAP {
            let write0 = usize::from(index == dst_start) & valid_usize;
            let write0_i64_mask = 0i64.wrapping_sub(write0 as i64);
            let write0_u8_mask = 0u8.wrapping_sub(write0 as u8);
            self.slots[index] =
                select_candidate(write0_i64_mask, write0_u8_mask, self.slots[index], child0);

            let write1 = usize::from(index == second_dst) & valid_usize;
            let write1_i64_mask = 0i64.wrapping_sub(write1 as i64);
            let write1_u8_mask = 0u8.wrapping_sub(write1 as u8);
            self.slots[index] =
                select_candidate(write1_i64_mask, write1_u8_mask, self.slots[index], child1);
        }

        domain_check
    }

    pub fn expand_then_compact_one_bit<const CHILD_CAP: usize, const L: usize>(
        &self,
        bit_index: usize,
        bit0_metric_delta: i64,
        bit1_metric_delta: i64,
) -> (FixedSclPathBuffer<CHILD_CAP, N>, [FixedTopLEntry; L]) {
        assert!(
            CAP.saturating_mul(2) <= CHILD_CAP,
            "expand-then-compact child capacity requires two slots per parent"
);
        let mut children = FixedSclPathBuffer::<CHILD_CAP, N>::new();
        for parent_slot in 0..CAP {
            children.write_binary_children_from(
                self,
                parent_slot,
                parent_slot * 2,
                bit_index,
                bit0_metric_delta,
                bit1_metric_delta,
);
        }
        let top = children.top_l_entries::<L>();
        (children, top)
    }

    pub fn try_expand_then_compact_one_bit<const CHILD_CAP: usize, const L: usize>(
        &self,
        bit_index: usize,
        bit0_metric_delta: i64,
        bit1_metric_delta: i64,
) -> FixedSclOneBitExpansionRun<CHILD_CAP, L, N> {
        let path_domain_check =
            fixed_scl_path_buffer_schedule_domain_check::<CAP, N, CHILD_CAP, CHILD_CAP, L, 1>([
                bit_index,
            ]);
        let zero_work_counts =
            fixed_scl_public_round_work_counts_with_capacities(CAP, CHILD_CAP, CHILD_CAP, L, 0);
        let invalid_usize = usize::from(path_domain_check.valid) ^ 1;
        let invalid_i64 = invalid_usize as i64;
        let invalid_u8 = invalid_usize as u8;
        let invalid_mask_usize = 0usize.wrapping_sub(invalid_usize);
        let invalid_mask_i64 = 0i64.wrapping_sub(invalid_i64);
        let invalid_mask_u8 = 0u8.wrapping_sub(invalid_u8);

        let mut children = FixedSclPathBuffer::<CHILD_CAP, N>::new();
        for parent_slot in 0..CAP {
            let _ = children.try_write_binary_children_from(
                self,
                parent_slot,
                parent_slot * 2,
                bit_index,
                bit0_metric_delta,
                bit1_metric_delta,
);
        }
        let top = children.top_l_entries_padded::<L>();
        let zero_children = FixedSclPathBuffer::<CHILD_CAP, N>::new();
        let zero_top = [FixedTopLEntry {
            metric: i64::MAX,
            index: usize::MAX,
        }; L];
        FixedSclOneBitExpansionRun {
            path_domain_check,
            work_counts: select_public_round_work_counts(
                invalid_mask_usize,
                fixed_scl_public_round_work_counts_with_capacities(CAP, CHILD_CAP, CHILD_CAP, L, 1),
                zero_work_counts,
),
            children: select_path_buffer(
                invalid_mask_i64,
                invalid_mask_u8,
                children,
                zero_children,
),
            top: select_top_l_entries(invalid_mask_usize, invalid_mask_i64, top, zero_top),
        }
    }

    fn from_top_entries<const SRC_CAP: usize>(
        source: &FixedSclPathBuffer<SRC_CAP, N>,
        top: [FixedTopLEntry; CAP],
) -> Self {
        let mut compacted = Self::new();
        for (dst_slot, entry) in top.iter().enumerate() {
            let mut selected = FixedSclCandidate::<N>::EMPTY;
            for source_slot in 0..SRC_CAP {
                let take_source = usize::from(source_slot == entry.index);
                let take_i64_mask = 0i64.wrapping_sub(take_source as i64);
                let take_u8_mask = 0u8.wrapping_sub(take_source as u8);
                selected = select_candidate(
                    take_i64_mask,
                    take_u8_mask,
                    selected,
                    source.slots[source_slot],
);
            }
            compacted.slots[dst_slot] = sanitize_candidate_active(selected);
        }
        compacted
    }

    pub fn expand_then_compact_two_public_bits<
        const FIRST_CHILD_CAP: usize,
        const SECOND_CHILD_CAP: usize,
        const L: usize,
    >(
        &self,
        first_round: (usize, i64, i64),
        second_round: (usize, i64, i64),
) -> (FixedSclPathBuffer<L, N>, [FixedTopLEntry; L]) {
        let (first_bit, first_bit0_delta, first_bit1_delta) = first_round;
        let (first_children, first_top) = self.expand_then_compact_one_bit::<FIRST_CHILD_CAP, L>(
            first_bit,
            first_bit0_delta,
            first_bit1_delta,
);
        let first_compacted =
            FixedSclPathBuffer::<L, N>::from_top_entries(&first_children, first_top);

        let (second_bit, second_bit0_delta, second_bit1_delta) = second_round;
        let (second_children, second_top) = first_compacted
.expand_then_compact_one_bit::<SECOND_CHILD_CAP, L>(
                second_bit,
                second_bit0_delta,
                second_bit1_delta,
);
        let second_compacted =
            FixedSclPathBuffer::<L, N>::from_top_entries(&second_children, second_top);
        (second_compacted, second_top)
    }

    pub fn try_expand_then_compact_two_public_bits<
        const FIRST_CHILD_CAP: usize,
        const SECOND_CHILD_CAP: usize,
        const L: usize,
    >(
        &self,
        first_round: (usize, i64, i64),
        second_round: (usize, i64, i64),
) -> FixedSclPublicRoundScheduleRun<L, N> {
        let (first_bit, first_bit0_delta, first_bit1_delta) = first_round;
        let (second_bit, second_bit0_delta, second_bit1_delta) = second_round;
        self.try_expand_then_compact_public_rounds::<FIRST_CHILD_CAP, SECOND_CHILD_CAP, L, 2>([
            FixedSclRound::new(first_bit, first_bit0_delta, first_bit1_delta),
            FixedSclRound::new(second_bit, second_bit0_delta, second_bit1_delta),
        ])
    }

    pub fn expand_then_compact_public_rounds<
        const FIRST_CHILD_CAP: usize,
        const CHILD_CAP: usize,
        const L: usize,
        const ROUNDS: usize,
    >(
        &self,
        rounds: [FixedSclRound; ROUNDS],
) -> (FixedSclPathBuffer<L, N>, [FixedTopLEntry; L]) {
        assert!(
            ROUNDS > 0,
            "public round schedule requires at least one round"
);

        let first_round = rounds[0];
        let (first_children, first_top) = self.expand_then_compact_one_bit::<FIRST_CHILD_CAP, L>(
            first_round.bit_index,
            first_round.bit0_metric_delta,
            first_round.bit1_metric_delta,
);
        let mut compacted =
            FixedSclPathBuffer::<L, N>::from_top_entries(&first_children, first_top);
        let mut final_top = first_top;

        for round in rounds.iter().skip(1) {
            let (children, top) = compacted.expand_then_compact_one_bit::<CHILD_CAP, L>(
                round.bit_index,
                round.bit0_metric_delta,
                round.bit1_metric_delta,
);
            compacted = FixedSclPathBuffer::<L, N>::from_top_entries(&children, top);
            final_top = top;
        }

        (compacted, final_top)
    }

    pub fn try_expand_then_compact_public_rounds<
        const FIRST_CHILD_CAP: usize,
        const CHILD_CAP: usize,
        const L: usize,
        const ROUNDS: usize,
    >(
        &self,
        rounds: [FixedSclRound; ROUNDS],
) -> FixedSclPublicRoundScheduleRun<L, N> {
        let plan =
            fixed_scl_round_schedule_plan::<CAP, N, FIRST_CHILD_CAP, CHILD_CAP, L, ROUNDS>(rounds);
        let invalid_usize = usize::from(plan.path_domain_check.valid) ^ 1;
        let invalid_i64 = invalid_usize as i64;
        let invalid_u8 = invalid_usize as u8;
        let invalid_mask_usize = 0usize.wrapping_sub(invalid_usize);
        let invalid_mask_i64 = 0i64.wrapping_sub(invalid_i64);
        let invalid_mask_u8 = 0u8.wrapping_sub(invalid_u8);

        let mut compacted = FixedSclPathBuffer::<L, N>::new();
        let mut final_top = [FixedTopLEntry {
            metric: i64::MAX,
            index: usize::MAX,
        }; L];

        if ROUNDS > 0 {
            let first_round = rounds[0];
            let first_run = self.try_expand_then_compact_one_bit::<FIRST_CHILD_CAP, L>(
                first_round.bit_index,
                first_round.bit0_metric_delta,
                first_round.bit1_metric_delta,
);
            compacted =
                FixedSclPathBuffer::<L, N>::from_top_entries(&first_run.children, first_run.top);
            final_top = first_run.top;

            for round in rounds.iter().skip(1) {
                let run = compacted.try_expand_then_compact_one_bit::<CHILD_CAP, L>(
                    round.bit_index,
                    round.bit0_metric_delta,
                    round.bit1_metric_delta,
);
                compacted = FixedSclPathBuffer::<L, N>::from_top_entries(&run.children, run.top);
                final_top = run.top;
            }
        }

        let zero_work_counts = fixed_scl_public_round_work_counts_with_capacities(
            CAP,
            FIRST_CHILD_CAP,
            CHILD_CAP,
            L,
            0,
);
        let zero_paths = FixedSclPathBuffer::<L, N>::new();
        let zero_top = [FixedTopLEntry {
            metric: i64::MAX,
            index: usize::MAX,
        }; L];

        FixedSclPublicRoundScheduleRun {
            path_domain_check: plan.path_domain_check,
            work_counts: select_public_round_work_counts(
                invalid_mask_usize,
                plan.work_counts,
                zero_work_counts,
),
            paths: select_path_buffer(invalid_mask_i64, invalid_mask_u8, compacted, zero_paths),
            top: select_top_l_entries(invalid_mask_usize, invalid_mask_i64, final_top, zero_top),
        }
    }

    pub fn expand_then_compact_integer_round_schedule<
        const FIRST_CHILD_CAP: usize,
        const CHILD_CAP: usize,
        const L: usize,
        const ROUNDS: usize,
    >(
        &self,
        bit_indices: [usize; ROUNDS],
        frozen_bits: [bool; ROUNDS],
        hard_bits: [u8; ROUNDS],
        magnitudes: [i64; ROUNDS],
) -> (FixedSclPathBuffer<L, N>, [FixedTopLEntry; L]) {
        let rounds =
            fixed_scl_integer_round_schedule(bit_indices, frozen_bits, hard_bits, magnitudes);
        self.expand_then_compact_public_rounds::<FIRST_CHILD_CAP, CHILD_CAP, L, ROUNDS>(rounds)
    }

    pub fn try_expand_then_compact_integer_round_schedule<
        const FIRST_CHILD_CAP: usize,
        const CHILD_CAP: usize,
        const L: usize,
        const ROUNDS: usize,
    >(
        &self,
        bit_indices: [usize; ROUNDS],
        frozen_bits: [bool; ROUNDS],
        hard_bits: [u8; ROUNDS],
        magnitudes: [i64; ROUNDS],
) -> FixedSclPathBufferIntegerScheduleRun<L, N> {
        let plan =
            fixed_scl_integer_round_schedule_plan::<CAP, N, FIRST_CHILD_CAP, CHILD_CAP, L, ROUNDS>(
                bit_indices,
                hard_bits,
                magnitudes,
);
        let schedule =
            try_fixed_scl_integer_round_schedule(bit_indices, frozen_bits, hard_bits, magnitudes);

        let invalid_usize = usize::from(plan.domain_check.valid) ^ 1;
        let invalid_i64 = invalid_usize as i64;
        let invalid_u8 = invalid_usize as u8;
        let invalid_mask_usize = 0usize.wrapping_sub(invalid_usize);
        let invalid_mask_i64 = 0i64.wrapping_sub(invalid_i64);
        let zero_work_counts = fixed_scl_public_round_work_counts_with_capacities(
            CAP,
            FIRST_CHILD_CAP,
            CHILD_CAP,
            L,
            0,
);
        let zero_paths = FixedSclPathBuffer::<L, N>::new();
        let zero_top = [FixedTopLEntry {
            metric: i64::MAX,
            index: usize::MAX,
        }; L];

        let run = self
.try_expand_then_compact_public_rounds::<FIRST_CHILD_CAP, CHILD_CAP, L, ROUNDS>(
                schedule.rounds,
);
        FixedSclPathBufferIntegerScheduleRun {
            domain_check: plan.domain_check,
            path_domain_check: plan.path_domain_check,
            work_counts: select_public_round_work_counts(
                invalid_mask_usize,
                plan.work_counts,
                zero_work_counts,
),
            paths: select_path_buffer(invalid_mask_i64, invalid_u8, run.paths, zero_paths),
            top: select_top_l_entries(invalid_mask_usize, invalid_mask_i64, run.top, zero_top),
        }
    }
}

impl<const CAP: usize, const N: usize> Default for FixedSclPathBuffer<CAP, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimulationConfig {
    pub n: usize,
    pub k: usize,
    pub p: f64,
    pub trials: usize,
    pub seed: u64,
}

impl SimulationResult {
    pub fn bler(&self) -> f64 {
        if self.trials == 0 {
            0.0
        } else {
            self.errors as f64 / self.trials as f64
        }
    }

    pub fn zero_error_upper_95(&self) -> Option<f64> {
        if self.errors == 0 && self.trials > 0 {
            Some(zero_error_upper_bound(self.trials, 0.05))
        } else {
            None
        }
    }
}

impl ImportanceSamplingResult {
    pub fn proposal_error_rate(&self) -> f64 {
        if self.trials == 0 {
            0.0
        } else {
            self.proposal_errors as f64 / self.trials as f64
        }
    }
}

pub fn zero_error_upper_bound(trials: usize, alpha: f64) -> f64 {
    assert!(trials > 0, "trials must be positive");
    assert!((0.0..1.0).contains(&alpha), "alpha must be in (0, 1)");
    1.0 - alpha.powf(1.0 / trials as f64)
}

pub fn baseline_reproduction_configs(trials: usize, seed: u64) -> Vec<SimulationConfig> {
    [
        (128, 16, 0.0706),
        (128, 16, 0.0343),
        (256, 32, 0.0706),
        (256, 32, 0.0343),
        (512, 64, 0.0706),
        (512, 64, 0.0343),
    ]
.into_iter()
.enumerate()
.map(|(i, (n, k, p))| SimulationConfig {
        n,
        k,
        p,
        trials,
        seed: seed.wrapping_add((i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)),
    })
.collect()
}

pub fn target_n2048_configs(trials: usize, seed: u64) -> Vec<SimulationConfig> {
    [(2048, 256, 0.0706), (2048, 256, 0.0343)]
.into_iter()
.enumerate()
.map(|(i, (n, k, p))| SimulationConfig {
            n,
            k,
            p,
            trials,
            seed: seed.wrapping_add((i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)),
        })
.collect()
}

pub fn high_noise_control_configs(trials: usize, seed: u64) -> Vec<SimulationConfig> {
    [(2048, 256, 0.3), (2048, 256, 0.4), (2048, 256, 0.5)]
.into_iter()
.enumerate()
.map(|(i, (n, k, p))| SimulationConfig {
            n,
            k,
            p,
            trials,
            seed: seed.wrapping_add((i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)),
        })
.collect()
}

pub fn fixed_i64_high_noise_control_configs(trials: usize, seed: u64) -> Vec<SimulationConfig> {
    [(128, 16, 0.3), (128, 16, 0.4), (128, 16, 0.5)]
.into_iter()
.enumerate()
.map(|(i, (n, k, p))| SimulationConfig {
            n,
            k,
            p,
            trials,
            seed: seed.wrapping_add((i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)),
        })
.collect()
}

pub fn fixed_i64_l8_validation_dispatch(cfg: &SimulationConfig) -> SimulationResult {
    match cfg.n {
        128 | 256 | 512 | 2048 => {
            simulate_bsc_scl_fixed_i64_l8_validation(cfg.n, cfg.k, cfg.p, cfg.trials, cfg.seed)
        }
        other => panic!("fixed-i64 validation dispatch does not support N={other}"),
    }
}

pub fn fixed_schedule_top_l_compare_count(width: usize) -> usize {
    width.saturating_mul(width.saturating_sub(1)) / 2
}

pub fn fixed_schedule_top_l_selection_plan(
    width: usize,
    list_size: usize,
) -> FixedScheduleTopLSelectionPlan {
    let valid = list_size <= width;
    let invalid_usize = usize::from(valid) ^ 1;
    let invalid_mask_usize = 0usize.wrapping_sub(invalid_usize);
    let invalid_u8 = u8::from(valid) ^ 1;
    let invalid_mask_u8 = 0u8.wrapping_sub(invalid_u8);
    FixedScheduleTopLSelectionPlan {
        width,
        list_size,
        valid,
        failure_code: select_u8(
            invalid_mask_u8,
            FIXED_TOP_L_SELECTION_DOMAIN_OK,
            FIXED_TOP_L_SELECTION_DOMAIN_WIDTH,
),
        compare_exchanges: select_usize(
            invalid_mask_usize,
            fixed_schedule_top_l_compare_count(width),
            0,
),
    }
}

pub fn fixed_scl_round_schedule_plan<
    const CAP: usize,
    const N: usize,
    const FIRST_CHILD_CAP: usize,
    const CHILD_CAP: usize,
    const L: usize,
    const ROUNDS: usize,
>(
    rounds: [FixedSclRound; ROUNDS],
) -> FixedSclPublicRoundSchedulePlan {
    let mut bit_indices = [0usize; ROUNDS];
    for (index, round) in rounds.iter().enumerate() {
        bit_indices[index] = round.bit_index;
    }
    fixed_scl_public_round_schedule_plan::<CAP, N, FIRST_CHILD_CAP, CHILD_CAP, L, ROUNDS>(
        bit_indices,
)
}

pub fn fixed_scl_round_schedule_plan_certificate<
    const CAP: usize,
    const N: usize,
    const FIRST_CHILD_CAP: usize,
    const CHILD_CAP: usize,
    const L: usize,
    const ROUNDS: usize,
>(
    rounds: [FixedSclRound; ROUNDS],
) -> FixedSclPublicRoundSchedulePlan {
    fixed_scl_round_schedule_plan::<CAP, N, FIRST_CHILD_CAP, CHILD_CAP, L, ROUNDS>(rounds)
}

pub fn fixed_scl_round_schedule_plan_parity_check<
    const CAP: usize,
    const N: usize,
    const FIRST_CHILD_CAP: usize,
    const CHILD_CAP: usize,
    const L: usize,
    const ROUNDS: usize,
>(
    rounds: [FixedSclRound; ROUNDS],
    expected_public_plan: FixedSclPublicRoundSchedulePlan,
) -> FixedSclRoundSchedulePlanParityCheck {
    let round_schedule_plan =
        fixed_scl_round_schedule_plan_certificate::<CAP, N, FIRST_CHILD_CAP, CHILD_CAP, L, ROUNDS>(
            rounds,
);

    FixedSclRoundSchedulePlanParityCheck {
        matches: round_schedule_plan == expected_public_plan,
        round_schedule_plan,
        expected_public_plan,
    }
}

pub fn fixed_scl_round_schedule_shape_plan<
    const CAP: usize,
    const N: usize,
    const FIRST_CHILD_CAP: usize,
    const CHILD_CAP: usize,
    const L: usize,
    const ROUNDS: usize,
>(
    rounds: [FixedSclRound; ROUNDS],
) -> FixedSclPublicRoundScheduleShapePlan {
    let mut bit_indices = [0usize; ROUNDS];
    for (index, round) in rounds.iter().enumerate() {
        bit_indices[index] = round.bit_index;
    }
    fixed_scl_public_round_schedule_shape_plan::<CAP, N, FIRST_CHILD_CAP, CHILD_CAP, L, ROUNDS>(
        bit_indices,
)
}

pub fn fixed_scl_round_schedule_shape_plan_certificate<
    const CAP: usize,
    const N: usize,
    const FIRST_CHILD_CAP: usize,
    const CHILD_CAP: usize,
    const L: usize,
    const ROUNDS: usize,
>(
    rounds: [FixedSclRound; ROUNDS],
) -> FixedSclPublicRoundScheduleShapePlan {
    fixed_scl_round_schedule_shape_plan::<CAP, N, FIRST_CHILD_CAP, CHILD_CAP, L, ROUNDS>(rounds)
}

pub fn fixed_scl_round_schedule_shape_parity_check<
    const CAP: usize,
    const N: usize,
    const FIRST_CHILD_CAP: usize,
    const CHILD_CAP: usize,
    const L: usize,
    const ROUNDS: usize,
>(
    rounds: [FixedSclRound; ROUNDS],
    expected_shape_plan: FixedSclPublicRoundScheduleShapePlan,
) -> FixedSclRoundScheduleShapeParityCheck {
    let round_shape_plan = fixed_scl_round_schedule_shape_plan_certificate::<
        CAP,
        N,
        FIRST_CHILD_CAP,
        CHILD_CAP,
        L,
        ROUNDS,
    >(rounds);

    FixedSclRoundScheduleShapeParityCheck {
        matches: round_shape_plan == expected_shape_plan,
        round_shape_plan,
        expected_shape_plan,
    }
}

pub fn fixed_scl_public_round_work_counts(
    parent_capacity: usize,
    child_capacity: usize,
    list_size: usize,
    rounds: usize,
) -> FixedSclPublicRoundWorkCounts {
    fixed_scl_public_round_work_counts_with_capacities(
        parent_capacity,
        child_capacity,
        child_capacity,
        list_size,
        rounds,
)
}

pub fn fixed_scl_public_round_work_counts_with_capacities(
    parent_capacity: usize,
    first_child_capacity: usize,
    repeated_child_capacity: usize,
    list_size: usize,
    rounds: usize,
) -> FixedSclPublicRoundWorkCounts {
    let first_rounds = usize::from(rounds > 0);
    let repeated_rounds = rounds.saturating_sub(1);
    FixedSclPublicRoundWorkCounts {
        parent_capacity,
        first_child_capacity,
        repeated_child_capacity,
        list_size,
        rounds,
        top_l_compare_exchanges: first_rounds
.saturating_mul(fixed_schedule_top_l_compare_count(first_child_capacity))
.saturating_add(
                repeated_rounds
.saturating_mul(fixed_schedule_top_l_compare_count(repeated_child_capacity)),
),
        child_slots_written: first_rounds
.saturating_mul(parent_capacity)
.saturating_mul(2)
.saturating_add(repeated_rounds.saturating_mul(list_size.saturating_mul(2))),
        compacted_slots_written: rounds.saturating_mul(list_size),
    }
}

pub fn fixed_scl_public_round_work_shape_plan(
    parent_capacity: usize,
    first_child_capacity: usize,
    repeated_child_capacity: usize,
    list_size: usize,
    rounds: usize,
) -> FixedSclPublicRoundWorkShapePlan {
    let first_top_l_plan = fixed_schedule_top_l_selection_plan(first_child_capacity, list_size);
    let repeated_top_l_plan =
        fixed_schedule_top_l_selection_plan(repeated_child_capacity, list_size);
    let first_round_active = rounds > 0;
    let repeated_round_active = rounds > 1;
    let valid = (!first_round_active || first_top_l_plan.valid)
        && (!repeated_round_active || repeated_top_l_plan.valid);
    let invalid_usize = usize::from(valid) ^ 1;
    let invalid_mask_usize = 0usize.wrapping_sub(invalid_usize);
    let work_rounds = select_usize(invalid_mask_usize, rounds, 0);

    FixedSclPublicRoundWorkShapePlan {
        parent_capacity,
        first_child_capacity,
        repeated_child_capacity,
        list_size,
        rounds,
        valid,
        first_top_l_plan,
        repeated_top_l_plan,
        work_counts: fixed_scl_public_round_work_counts_with_capacities(
            parent_capacity,
            first_child_capacity,
            repeated_child_capacity,
            list_size,
            work_rounds,
),
    }
}

pub fn fixed_scl_public_round_schedule_shape_plan<
    const CAP: usize,
    const N: usize,
    const FIRST_CHILD_CAP: usize,
    const CHILD_CAP: usize,
    const L: usize,
    const ROUNDS: usize,
>(
    bit_indices: [usize; ROUNDS],
) -> FixedSclPublicRoundScheduleShapePlan {
    let path_domain_check = fixed_scl_path_buffer_schedule_domain_check::<
        CAP,
        N,
        FIRST_CHILD_CAP,
        CHILD_CAP,
        L,
        ROUNDS,
    >(bit_indices);
    let invalid_usize = usize::from(path_domain_check.valid) ^ 1;
    let invalid_mask_usize = 0usize.wrapping_sub(invalid_usize);
    let active_rounds = select_usize(invalid_mask_usize, ROUNDS, 0);
    let work_shape_plan =
        fixed_scl_public_round_work_shape_plan(CAP, FIRST_CHILD_CAP, CHILD_CAP, L, active_rounds);

    FixedSclPublicRoundScheduleShapePlan {
        valid: path_domain_check.valid && work_shape_plan.valid,
        path_domain_check,
        work_shape_plan,
    }
}

pub fn fixed_scl_public_round_run_shape_certificate<const L: usize, const N: usize>(
    run: &FixedSclPublicRoundScheduleRun<L, N>,
) -> FixedSclPublicRoundScheduleShapePlan {
    let counts = run.work_counts;
    let work_shape_plan = fixed_scl_public_round_work_shape_plan(
        counts.parent_capacity,
        counts.first_child_capacity,
        counts.repeated_child_capacity,
        counts.list_size,
        counts.rounds,
);

    FixedSclPublicRoundScheduleShapePlan {
        valid: run.path_domain_check.valid && work_shape_plan.valid,
        path_domain_check: run.path_domain_check,
        work_shape_plan,
    }
}

pub fn fixed_scl_public_round_shape_parity_check<const L: usize, const N: usize>(
    run: &FixedSclPublicRoundScheduleRun<L, N>,
    expected_shape_plan: FixedSclPublicRoundScheduleShapePlan,
) -> FixedSclPublicRoundShapeParityCheck {
    let run_shape_certificate = fixed_scl_public_round_run_shape_certificate(run);

    FixedSclPublicRoundShapeParityCheck {
        matches: run_shape_certificate == expected_shape_plan,
        run_shape_certificate,
        expected_shape_plan,
    }
}

pub fn two_public_bits_run_shape_certificate<const L: usize, const N: usize>(
    run: &FixedSclPublicRoundScheduleRun<L, N>,
) -> FixedSclPublicRoundScheduleShapePlan {
    fixed_scl_public_round_run_shape_certificate(run)
}

pub fn two_public_bits_shape_parity_check<const L: usize, const N: usize>(
    run: &FixedSclPublicRoundScheduleRun<L, N>,
    expected_shape_plan: FixedSclPublicRoundScheduleShapePlan,
) -> FixedSclPublicRoundShapeParityCheck {
    fixed_scl_public_round_shape_parity_check(run, expected_shape_plan)
}

pub fn fixed_scl_one_bit_run_plan_certificate<
    const CHILD_CAP: usize,
    const L: usize,
    const N: usize,
>(
    run: &FixedSclOneBitExpansionRun<CHILD_CAP, L, N>,
) -> FixedSclPublicRoundSchedulePlan {
    FixedSclPublicRoundSchedulePlan {
        path_domain_check: run.path_domain_check,
        work_counts: run.work_counts,
    }
}

pub fn fixed_scl_one_bit_shape_parity_check<
    const CHILD_CAP: usize,
    const L: usize,
    const N: usize,
>(
    run: &FixedSclOneBitExpansionRun<CHILD_CAP, L, N>,
    expected_plan: FixedSclPublicRoundSchedulePlan,
) -> FixedSclOneBitShapeParityCheck {
    let run_plan_certificate = fixed_scl_one_bit_run_plan_certificate(run);

    FixedSclOneBitShapeParityCheck {
        matches: run_plan_certificate == expected_plan,
        run_plan_certificate,
        expected_plan,
    }
}

pub fn fixed_scl_public_round_schedule_plan<
    const CAP: usize,
    const N: usize,
    const FIRST_CHILD_CAP: usize,
    const CHILD_CAP: usize,
    const L: usize,
    const ROUNDS: usize,
>(
    bit_indices: [usize; ROUNDS],
) -> FixedSclPublicRoundSchedulePlan {
    let path_domain_check = fixed_scl_path_buffer_schedule_domain_check::<
        CAP,
        N,
        FIRST_CHILD_CAP,
        CHILD_CAP,
        L,
        ROUNDS,
    >(bit_indices);
    let invalid_usize = usize::from(path_domain_check.valid) ^ 1;
    let invalid_mask_usize = 0usize.wrapping_sub(invalid_usize);
    let full_work_counts = fixed_scl_public_round_work_counts_with_capacities(
        CAP,
        FIRST_CHILD_CAP,
        CHILD_CAP,
        L,
        ROUNDS,
);
    let zero_work_counts =
        fixed_scl_public_round_work_counts_with_capacities(CAP, FIRST_CHILD_CAP, CHILD_CAP, L, 0);

    FixedSclPublicRoundSchedulePlan {
        path_domain_check,
        work_counts: select_public_round_work_counts(
            invalid_mask_usize,
            full_work_counts,
            zero_work_counts,
),
    }
}

pub fn fixed_scl_integer_round_schedule_plan<
    const CAP: usize,
    const N: usize,
    const FIRST_CHILD_CAP: usize,
    const CHILD_CAP: usize,
    const L: usize,
    const ROUNDS: usize,
>(
    bit_indices: [usize; ROUNDS],
    hard_bits: [u8; ROUNDS],
    magnitudes: [i64; ROUNDS],
) -> FixedSclIntegerRoundSchedulePlan {
    let domain_check = fixed_scl_integer_schedule_domain_check(hard_bits, magnitudes);
    let public_plan =
        fixed_scl_public_round_schedule_plan::<CAP, N, FIRST_CHILD_CAP, CHILD_CAP, L, ROUNDS>(
            bit_indices,
);
    let invalid_usize = usize::from(domain_check.valid) ^ 1;
    let invalid_mask_usize = 0usize.wrapping_sub(invalid_usize);
    let zero_work_counts =
        fixed_scl_public_round_work_counts_with_capacities(CAP, FIRST_CHILD_CAP, CHILD_CAP, L, 0);

    FixedSclIntegerRoundSchedulePlan {
        domain_check,
        path_domain_check: public_plan.path_domain_check,
        work_counts: select_public_round_work_counts(
            invalid_mask_usize,
            public_plan.work_counts,
            zero_work_counts,
),
    }
}

pub fn fixed_scl_integer_round_schedule_shape_plan<
    const CAP: usize,
    const N: usize,
    const FIRST_CHILD_CAP: usize,
    const CHILD_CAP: usize,
    const L: usize,
    const ROUNDS: usize,
>(
    bit_indices: [usize; ROUNDS],
    hard_bits: [u8; ROUNDS],
    magnitudes: [i64; ROUNDS],
) -> FixedSclIntegerRoundScheduleShapePlan {
    let domain_check = fixed_scl_integer_schedule_domain_check(hard_bits, magnitudes);
    let public_shape_plan = fixed_scl_public_round_schedule_shape_plan::<
        CAP,
        N,
        FIRST_CHILD_CAP,
        CHILD_CAP,
        L,
        ROUNDS,
    >(bit_indices);
    let invalid_usize = usize::from(domain_check.valid) ^ 1;
    let invalid_mask_usize = 0usize.wrapping_sub(invalid_usize);
    let zero_work_shape_plan =
        fixed_scl_public_round_work_shape_plan(CAP, FIRST_CHILD_CAP, CHILD_CAP, L, 0);
    let work_shape_plan = select_public_round_work_shape_plan(
        invalid_mask_usize,
        public_shape_plan.work_shape_plan,
        zero_work_shape_plan,
);

    FixedSclIntegerRoundScheduleShapePlan {
        valid: domain_check.valid
            && public_shape_plan.path_domain_check.valid
            && work_shape_plan.valid,
        domain_check,
        path_domain_check: public_shape_plan.path_domain_check,
        work_shape_plan,
    }
}

fn select_top_l_selection_plan(
    mask_usize: usize,
    mask_u8: u8,
    keep: FixedScheduleTopLSelectionPlan,
    replace: FixedScheduleTopLSelectionPlan,
) -> FixedScheduleTopLSelectionPlan {
    FixedScheduleTopLSelectionPlan {
        width: select_usize(mask_usize, keep.width, replace.width),
        list_size: select_usize(mask_usize, keep.list_size, replace.list_size),
        valid: select_u8(mask_u8, u8::from(keep.valid), u8::from(replace.valid)) != 0,
        failure_code: select_u8(mask_u8, keep.failure_code, replace.failure_code),
        compare_exchanges: select_usize(
            mask_usize,
            keep.compare_exchanges,
            replace.compare_exchanges,
),
    }
}

fn select_public_round_work_shape_plan(
    mask_usize: usize,
    keep: FixedSclPublicRoundWorkShapePlan,
    replace: FixedSclPublicRoundWorkShapePlan,
) -> FixedSclPublicRoundWorkShapePlan {
    let mask_u8 = u8::from(mask_usize != 0).wrapping_mul(u8::MAX);
    FixedSclPublicRoundWorkShapePlan {
        parent_capacity: select_usize(mask_usize, keep.parent_capacity, replace.parent_capacity),
        first_child_capacity: select_usize(
            mask_usize,
            keep.first_child_capacity,
            replace.first_child_capacity,
),
        repeated_child_capacity: select_usize(
            mask_usize,
            keep.repeated_child_capacity,
            replace.repeated_child_capacity,
),
        list_size: select_usize(mask_usize, keep.list_size, replace.list_size),
        rounds: select_usize(mask_usize, keep.rounds, replace.rounds),
        valid: select_u8(mask_u8, u8::from(keep.valid), u8::from(replace.valid)) != 0,
        first_top_l_plan: select_top_l_selection_plan(
            mask_usize,
            mask_u8,
            keep.first_top_l_plan,
            replace.first_top_l_plan,
),
        repeated_top_l_plan: select_top_l_selection_plan(
            mask_usize,
            mask_u8,
            keep.repeated_top_l_plan,
            replace.repeated_top_l_plan,
),
        work_counts: select_public_round_work_counts(
            mask_usize,
            keep.work_counts,
            replace.work_counts,
),
    }
}

pub fn fixed_scl_integer_round_run_plan_certificate<const L: usize, const N: usize>(
    run: &FixedSclPathBufferIntegerScheduleRun<L, N>,
) -> FixedSclIntegerRoundSchedulePlan {
    FixedSclIntegerRoundSchedulePlan {
        domain_check: run.domain_check,
        path_domain_check: run.path_domain_check,
        work_counts: run.work_counts,
    }
}

pub fn fixed_scl_integer_round_run_shape_certificate<const L: usize, const N: usize>(
    run: &FixedSclPathBufferIntegerScheduleRun<L, N>,
) -> FixedSclIntegerRoundScheduleShapePlan {
    let counts = run.work_counts;
    let work_shape_plan = fixed_scl_public_round_work_shape_plan(
        counts.parent_capacity,
        counts.first_child_capacity,
        counts.repeated_child_capacity,
        counts.list_size,
        counts.rounds,
);

    FixedSclIntegerRoundScheduleShapePlan {
        valid: run.domain_check.valid && run.path_domain_check.valid && work_shape_plan.valid,
        domain_check: run.domain_check,
        path_domain_check: run.path_domain_check,
        work_shape_plan,
    }
}

pub fn fixed_scl_integer_shape_parity_check<const L: usize, const N: usize>(
    run: &FixedSclPathBufferIntegerScheduleRun<L, N>,
    expected_plan: FixedSclIntegerRoundSchedulePlan,
) -> FixedSclIntegerShapeParityCheck {
    let run_plan_certificate = fixed_scl_integer_round_run_plan_certificate(run);

    FixedSclIntegerShapeParityCheck {
        matches: run_plan_certificate == expected_plan,
        run_plan_certificate,
        expected_plan,
    }
}

pub fn fixed_scl_integer_schedule_shape_parity_check<const L: usize, const N: usize>(
    run: &FixedSclPathBufferIntegerScheduleRun<L, N>,
    expected_shape_plan: FixedSclIntegerRoundScheduleShapePlan,
) -> FixedSclIntegerScheduleShapeParityCheck {
    let run_shape_certificate = fixed_scl_integer_round_run_shape_certificate(run);

    FixedSclIntegerScheduleShapeParityCheck {
        matches: run_shape_certificate == expected_shape_plan,
        run_shape_certificate,
        expected_shape_plan,
    }
}

pub fn fixed_scl_integer_schedule_shape_failure_family(
    plan: FixedSclIntegerRoundScheduleShapePlan,
) -> u8 {
    let domain_invalid = u8::from(!plan.domain_check.valid);
    let path_invalid = u8::from(!plan.path_domain_check.valid);
    let work_invalid = u8::from(!plan.work_shape_plan.valid);
    let domain_valid = domain_invalid ^ 1;
    let path_valid = path_invalid ^ 1;
    let path_selected = path_invalid & domain_valid;
    let work_selected = work_invalid & domain_valid & path_valid;
    let domain_mask = 0u8.wrapping_sub(domain_invalid);
    let path_mask = 0u8.wrapping_sub(path_selected);
    let work_mask = 0u8.wrapping_sub(work_selected);
    let failure_after_work = select_u8(
        work_mask,
        FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_FAMILY_OK,
        FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_FAMILY_WORK_SHAPE,
);
    let failure_after_path = select_u8(
        path_mask,
        failure_after_work,
        FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_FAMILY_PATH_DOMAIN,
);

    select_u8(
        domain_mask,
        failure_after_path,
        FIXED_SCL_INTEGER_SCHEDULE_SHAPE_FAILURE_FAMILY_INTEGER_DOMAIN,
)
}

pub fn fixed_scl_public_round_schedule_shape_failure_family(
    plan: FixedSclPublicRoundScheduleShapePlan,
) -> u8 {
    let path_invalid = u8::from(!plan.path_domain_check.valid);
    let work_invalid = u8::from(!plan.work_shape_plan.valid);
    let path_valid = path_invalid ^ 1;
    let work_selected = work_invalid & path_valid;
    let path_mask = 0u8.wrapping_sub(path_invalid);
    let work_mask = 0u8.wrapping_sub(work_selected);
    let failure_after_work = select_u8(
        work_mask,
        FIXED_SCL_PUBLIC_ROUND_SCHEDULE_SHAPE_FAILURE_FAMILY_OK,
        FIXED_SCL_PUBLIC_ROUND_SCHEDULE_SHAPE_FAILURE_FAMILY_WORK_SHAPE,
);

    select_u8(
        path_mask,
        failure_after_work,
        FIXED_SCL_PUBLIC_ROUND_SCHEDULE_SHAPE_FAILURE_FAMILY_PATH_DOMAIN,
)
}

pub fn fixed_scl_binary_child_write_domain_check<
    const SRC_CAP: usize,
    const CHILD_CAP: usize,
    const N: usize,
>(
    parent_slot: usize,
    dst_start: usize,
    bit_index: usize,
) -> FixedSclBinaryChildWriteDomainCheck {
    let parent_invalid = u8::from(parent_slot >= SRC_CAP);
    let dst_invalid = u8::from(dst_start >= CHILD_CAP || dst_start.saturating_add(1) >= CHILD_CAP);
    let bit_invalid = u8::from(bit_index >= N);
    let parent_valid = parent_invalid ^ 1;
    let dst_valid = dst_invalid ^ 1;
    let dst_selected = dst_invalid & parent_valid;
    let bit_selected = bit_invalid & parent_valid & dst_valid;
    let invalid = parent_invalid | dst_invalid | bit_invalid;
    let parent_mask = 0u8.wrapping_sub(parent_invalid);
    let dst_mask = 0u8.wrapping_sub(dst_selected);
    let bit_mask = 0u8.wrapping_sub(bit_selected);
    let valid_mask = 0usize.wrapping_sub(usize::from(invalid ^ 1));
    let failure_after_bit = select_u8(
        bit_mask,
        FIXED_SCL_CHILD_WRITE_DOMAIN_OK,
        FIXED_SCL_CHILD_WRITE_DOMAIN_BIT_INDEX,
);
    let failure_after_dst = select_u8(
        dst_mask,
        failure_after_bit,
        FIXED_SCL_CHILD_WRITE_DOMAIN_DST_CAPACITY,
);
    let failure_code = select_u8(
        parent_mask,
        failure_after_dst,
        FIXED_SCL_CHILD_WRITE_DOMAIN_PARENT_SLOT,
);
    let child_slots_written = select_usize(valid_mask, 0, 2);

    FixedSclBinaryChildWriteDomainCheck {
        parent_capacity: SRC_CAP,
        child_capacity: CHILD_CAP,
        bit_width: N,
        parent_slot,
        dst_start,
        bit_index,
        child_slots_written,
        valid: invalid == 0,
        failure_code,
    }
}

pub fn fixed_scl_child_write_parity_check(
    run_domain_check: FixedSclBinaryChildWriteDomainCheck,
    expected_domain_check: FixedSclBinaryChildWriteDomainCheck,
) -> FixedSclChildWriteParityCheck {
    FixedSclChildWriteParityCheck {
        matches: run_domain_check == expected_domain_check,
        run_domain_check,
        expected_domain_check,
    }
}

pub fn fixed_scl_path_buffer_schedule_domain_check<
    const CAP: usize,
    const N: usize,
    const FIRST_CHILD_CAP: usize,
    const CHILD_CAP: usize,
    const L: usize,
    const ROUNDS: usize,
>(
    bit_indices: [usize; ROUNDS],
) -> FixedSclPathBufferScheduleDomainCheck {
    let empty_invalid = u8::from(ROUNDS == 0);
    let first_child_invalid = u8::from(CAP.saturating_mul(2) > FIRST_CHILD_CAP);
    let top_l_invalid = u8::from(L > FIRST_CHILD_CAP || (ROUNDS > 1 && L > CHILD_CAP));
    let repeated_child_invalid = u8::from(ROUNDS > 1 && L.saturating_mul(2) > CHILD_CAP);
    let empty_valid = empty_invalid ^ 1;
    let first_child_valid = first_child_invalid ^ 1;
    let top_l_valid = top_l_invalid ^ 1;
    let first_child_selected = first_child_invalid & empty_valid;
    let top_l_selected = top_l_invalid & empty_valid & first_child_valid;
    let repeated_child_selected =
        repeated_child_invalid & empty_valid & first_child_valid & top_l_valid;
    let shape_invalid =
        empty_invalid | first_child_invalid | top_l_invalid | repeated_child_invalid;
    let mut bit_invalid_seen = 0u8;
    let mut first_bit_invalid_round = FIXED_SCL_NO_INVALID_ROUND;

    for (round, bit_index) in bit_indices.iter().enumerate() {
        let bit_invalid = u8::from(*bit_index >= N);
        let first_bit_for_round = (bit_invalid_seen ^ 1) & bit_invalid;
        let first_bit_mask = 0usize.wrapping_sub(usize::from(first_bit_for_round));

        first_bit_invalid_round = select_usize(first_bit_mask, first_bit_invalid_round, round);
        bit_invalid_seen |= bit_invalid;
    }

    let bit_selected = bit_invalid_seen & (shape_invalid ^ 1);
    let empty_mask = 0u8.wrapping_sub(empty_invalid);
    let first_child_mask = 0u8.wrapping_sub(first_child_selected);
    let top_l_mask = 0u8.wrapping_sub(top_l_selected);
    let repeated_child_mask = 0u8.wrapping_sub(repeated_child_selected);
    let bit_mask = 0u8.wrapping_sub(bit_selected);
    let bit_round_mask = 0usize.wrapping_sub(usize::from(bit_selected));
    let failure_after_bit = select_u8(
        bit_mask,
        FIXED_SCL_PATH_DOMAIN_OK,
        FIXED_SCL_PATH_DOMAIN_BIT_INDEX,
);
    let failure_after_repeated_child = select_u8(
        repeated_child_mask,
        failure_after_bit,
        FIXED_SCL_PATH_DOMAIN_REPEATED_CHILD_CAPACITY,
);
    let failure_after_top_l = select_u8(
        top_l_mask,
        failure_after_repeated_child,
        FIXED_SCL_PATH_DOMAIN_TOP_L_WIDTH,
);
    let failure_after_first_child = select_u8(
        first_child_mask,
        failure_after_top_l,
        FIXED_SCL_PATH_DOMAIN_FIRST_CHILD_CAPACITY,
);
    let failure_code = select_u8(
        empty_mask,
        failure_after_first_child,
        FIXED_SCL_PATH_DOMAIN_EMPTY_SCHEDULE,
);
    let first_invalid_round = select_usize(
        bit_round_mask,
        FIXED_SCL_NO_INVALID_ROUND,
        first_bit_invalid_round,
);
    let invalid = shape_invalid | bit_invalid_seen;

    FixedSclPathBufferScheduleDomainCheck {
        parent_capacity: CAP,
        first_child_capacity: FIRST_CHILD_CAP,
        repeated_child_capacity: CHILD_CAP,
        list_size: L,
        rounds: ROUNDS,
        bit_width: N,
        valid: invalid == 0,
        failure_code,
        first_invalid_round,
    }
}

pub fn fixed_scl_integer_schedule_domain_check<const ROUNDS: usize>(
    hard_bits: [u8; ROUNDS],
    magnitudes: [i64; ROUNDS],
) -> FixedSclIntegerScheduleDomainCheck {
    let mut invalid_seen = 0u8;
    let mut failure_code = FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_OK;
    let mut first_invalid_round = FIXED_SCL_NO_INVALID_ROUND;

    for index in 0..ROUNDS {
        let hard_invalid = u8::from(hard_bits[index] > 1);
        let magnitude_invalid = u8::from(magnitudes[index] < 0);
        let hard_valid = hard_invalid ^ 1;
        let magnitude_selected = magnitude_invalid & hard_valid;
        let round_invalid = hard_invalid | magnitude_invalid;
        let first_for_round = (invalid_seen ^ 1) & round_invalid;
        let magnitude_mask = 0u8.wrapping_sub(magnitude_selected);
        let hard_mask = 0u8.wrapping_sub(hard_invalid);
        let first_mask = 0u8.wrapping_sub(first_for_round);
        let first_round_mask = 0usize.wrapping_sub(usize::from(first_for_round));
        let failure_after_magnitude = select_u8(
            magnitude_mask,
            FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_OK,
            FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_MAGNITUDE,
);
        let round_failure_code = select_u8(
            hard_mask,
            failure_after_magnitude,
            FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_HARD_BIT,
);

        failure_code = select_u8(first_mask, failure_code, round_failure_code);
        first_invalid_round = select_usize(first_round_mask, first_invalid_round, index);
        invalid_seen |= round_invalid;
    }

    FixedSclIntegerScheduleDomainCheck {
        rounds: ROUNDS,
        valid: invalid_seen == 0,
        failure_code,
        first_invalid_round,
    }
}

pub fn fixed_scl_integer_metric_domain_check(
    hard_bit: u8,
    magnitude: i64,
) -> FixedSclIntegerMetricDomainCheck {
    let hard_invalid = u8::from(hard_bit > 1);
    let magnitude_invalid = u8::from(magnitude < 0);
    let hard_mask = 0u8.wrapping_sub(hard_invalid);
    let magnitude_selected = magnitude_invalid & (hard_invalid ^ 1);
    let magnitude_mask = 0u8.wrapping_sub(magnitude_selected);
    let failure_after_magnitude = select_u8(
        magnitude_mask,
        FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_OK,
        FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_MAGNITUDE,
);
    let failure_code = select_u8(
        hard_mask,
        failure_after_magnitude,
        FIXED_SCL_INTEGER_SCHEDULE_DOMAIN_HARD_BIT,
);
    let invalid = hard_invalid | magnitude_invalid;

    FixedSclIntegerMetricDomainCheck {
        valid: invalid == 0,
        failure_code,
    }
}

pub fn fixed_scl_integer_metric_deltas(
    frozen_bit: bool,
    hard_bit: u8,
    magnitude: i64,
) -> FixedSclMetricDeltas {
    assert!(hard_bit <= 1, "integer metric hard bit must be 0 or 1");
    assert!(
        magnitude >= 0,
        "integer metric magnitude must be non-negative"
);

    let hard_bit_i64 = i64::from(hard_bit & 1);
    let hard_bit_mask = 0i64.wrapping_sub(hard_bit_i64);
    let frozen_i64 = i64::from(frozen_bit);
    let frozen_mask = 0i64.wrapping_sub(frozen_i64);

    let bit0_metric_delta = select_i64(hard_bit_mask, 0, magnitude);
    let unfrozen_bit1_delta = select_i64(hard_bit_mask, magnitude, 0);
    let bit1_metric_delta = select_i64(
        frozen_mask,
        unfrozen_bit1_delta,
        FIXED_SCL_FORBIDDEN_METRIC_DELTA,
);

    FixedSclMetricDeltas {
        bit0_metric_delta,
        bit1_metric_delta,
    }
}

pub fn try_fixed_scl_integer_metric_deltas(
    frozen_bit: bool,
    hard_bit: u8,
    magnitude: i64,
) -> FixedSclIntegerMetricDeltaRun {
    let domain_check = fixed_scl_integer_metric_domain_check(hard_bit, magnitude);
    let invalid_i64 = i64::from(domain_check.valid) ^ 1;
    let invalid_mask = 0i64.wrapping_sub(invalid_i64);
    let magnitude_negative = i64::from(magnitude < 0);
    let magnitude_negative_mask = 0i64.wrapping_sub(magnitude_negative);
    let safe_magnitude = select_i64(magnitude_negative_mask, magnitude, 0);
    let hard_bit_i64 = i64::from(hard_bit & 1);
    let hard_bit_mask = 0i64.wrapping_sub(hard_bit_i64);
    let frozen_i64 = i64::from(frozen_bit);
    let frozen_mask = 0i64.wrapping_sub(frozen_i64);
    let bit0_metric_delta = select_i64(hard_bit_mask, 0, safe_magnitude);
    let unfrozen_bit1_delta = select_i64(hard_bit_mask, safe_magnitude, 0);
    let bit1_metric_delta = select_i64(
        frozen_mask,
        unfrozen_bit1_delta,
        FIXED_SCL_FORBIDDEN_METRIC_DELTA,
);

    FixedSclIntegerMetricDeltaRun {
        domain_check,
        deltas: FixedSclMetricDeltas {
            bit0_metric_delta: select_i64(
                invalid_mask,
                bit0_metric_delta,
                FIXED_SCL_FORBIDDEN_METRIC_DELTA,
),
            bit1_metric_delta: select_i64(
                invalid_mask,
                bit1_metric_delta,
                FIXED_SCL_FORBIDDEN_METRIC_DELTA,
),
        },
    }
}

pub fn fixed_scl_integer_round_schedule<const ROUNDS: usize>(
    bit_indices: [usize; ROUNDS],
    frozen_bits: [bool; ROUNDS],
    hard_bits: [u8; ROUNDS],
    magnitudes: [i64; ROUNDS],
) -> [FixedSclRound; ROUNDS] {
    let mut rounds = [FixedSclRound::new(0, 0, 0); ROUNDS];
    for index in 0..ROUNDS {
        let deltas = fixed_scl_integer_metric_deltas(
            frozen_bits[index],
            hard_bits[index],
            magnitudes[index],
);
        rounds[index] = FixedSclRound::new(
            bit_indices[index],
            deltas.bit0_metric_delta,
            deltas.bit1_metric_delta,
);
    }
    rounds
}

pub fn try_fixed_scl_integer_round_schedule<const ROUNDS: usize>(
    bit_indices: [usize; ROUNDS],
    frozen_bits: [bool; ROUNDS],
    hard_bits: [u8; ROUNDS],
    magnitudes: [i64; ROUNDS],
) -> FixedSclIntegerRoundScheduleBuild<ROUNDS> {
    let domain_check = fixed_scl_integer_schedule_domain_check(hard_bits, magnitudes);
    let invalid_usize = usize::from(domain_check.valid) ^ 1;
    let invalid_i64 = i64::from(domain_check.valid) ^ 1;
    let invalid_mask_usize = 0usize.wrapping_sub(invalid_usize);
    let invalid_mask_i64 = 0i64.wrapping_sub(invalid_i64);
    let zero_round = FixedSclRound::new(0, 0, 0);
    let mut rounds = [zero_round; ROUNDS];

    for index in 0..ROUNDS {
        let delta_run = try_fixed_scl_integer_metric_deltas(
            frozen_bits[index],
            hard_bits[index],
            magnitudes[index],
);
        let candidate_round = FixedSclRound::new(
            bit_indices[index],
            delta_run.deltas.bit0_metric_delta,
            delta_run.deltas.bit1_metric_delta,
);
        rounds[index] = select_round(
            invalid_mask_usize,
            invalid_mask_i64,
            candidate_round,
            zero_round,
);
    }

    FixedSclIntegerRoundScheduleBuild {
        domain_check,
        round_slots_written: select_usize(invalid_mask_usize, ROUNDS, 0),
        rounds,
    }
}

pub fn fixed_scl_integer_round_schedule_build_plan<const ROUNDS: usize>(
    hard_bits: [u8; ROUNDS],
    magnitudes: [i64; ROUNDS],
) -> FixedSclIntegerRoundScheduleBuildPlan {
    let domain_check = fixed_scl_integer_schedule_domain_check(hard_bits, magnitudes);
    let invalid_usize = usize::from(domain_check.valid) ^ 1;
    let invalid_mask_usize = 0usize.wrapping_sub(invalid_usize);

    FixedSclIntegerRoundScheduleBuildPlan {
        domain_check,
        round_slots_written: select_usize(invalid_mask_usize, ROUNDS, 0),
    }
}

pub fn fixed_scl_integer_round_build_certificate<const ROUNDS: usize>(
    build: &FixedSclIntegerRoundScheduleBuild<ROUNDS>,
) -> FixedSclIntegerRoundScheduleBuildPlan {
    FixedSclIntegerRoundScheduleBuildPlan {
        domain_check: build.domain_check,
        round_slots_written: build.round_slots_written,
    }
}

pub fn fixed_scl_integer_round_build_parity_check<const ROUNDS: usize>(
    build: &FixedSclIntegerRoundScheduleBuild<ROUNDS>,
    expected_plan: FixedSclIntegerRoundScheduleBuildPlan,
) -> FixedSclIntegerRoundScheduleBuildParityCheck {
    let run_build_certificate = fixed_scl_integer_round_build_certificate(build);

    FixedSclIntegerRoundScheduleBuildParityCheck {
        matches: run_build_certificate == expected_plan,
        run_build_certificate,
        expected_plan,
    }
}

pub fn fixed_schedule_top_l_i64<const WIDTH: usize, const L: usize>(
    metrics: [i64; WIDTH],
) -> [FixedTopLEntry; L] {
    assert!(L <= WIDTH, "top-L selector requires L <= WIDTH");
    let mut entries = [FixedTopLEntry {
        metric: i64::MAX,
        index: usize::MAX,
    }; WIDTH];
    for i in 0..WIDTH {
        entries[i] = FixedTopLEntry {
            metric: metrics[i],
            index: i,
        };
    }

    for i in 0..WIDTH {
        for j in (i + 1)..WIDTH {
            fixed_compare_exchange(&mut entries, i, j);
        }
    }

    let mut top = [FixedTopLEntry {
        metric: i64::MAX,
        index: usize::MAX,
    }; L];
    top[..L].copy_from_slice(&entries[..L]);
    top
}

fn fixed_compare_exchange(entries: &mut [FixedTopLEntry], left: usize, right: usize) {
    let a = entries[left];
    let b = entries[right];
    let take_b = entry_less_flag(b, a);
    let mask_usize = 0usize.wrapping_sub(take_b);
    let mask_i64 = 0i64.wrapping_sub(take_b as i64);

    entries[left] = FixedTopLEntry {
        metric: select_i64(mask_i64, a.metric, b.metric),
        index: select_usize(mask_usize, a.index, b.index),
    };
    entries[right] = FixedTopLEntry {
        metric: select_i64(mask_i64, b.metric, a.metric),
        index: select_usize(mask_usize, b.index, a.index),
    };
}

fn entry_less_flag(a: FixedTopLEntry, b: FixedTopLEntry) -> usize {
    let metric_lt = usize::from(a.metric < b.metric);
    let metric_eq = usize::from(a.metric == b.metric);
    let index_lt = usize::from(a.index < b.index);

    metric_lt | (metric_eq & index_lt)
}

fn select_i64(mask: i64, keep: i64, replace: i64) -> i64 {
    (keep & !mask) | (replace & mask)
}

fn select_usize(mask: usize, keep: usize, replace: usize) -> usize {
    (keep & !mask) | (replace & mask)
}

fn select_u8(mask: u8, keep: u8, replace: u8) -> u8 {
    (keep & !mask) | (replace & mask)
}

fn select_top_l_entry(
    mask_usize: usize,
    mask_i64: i64,
    keep: FixedTopLEntry,
    replace: FixedTopLEntry,
) -> FixedTopLEntry {
    FixedTopLEntry {
        metric: select_i64(mask_i64, keep.metric, replace.metric),
        index: select_usize(mask_usize, keep.index, replace.index),
    }
}

fn select_top_l_entries<const L: usize>(
    mask_usize: usize,
    mask_i64: i64,
    keep: [FixedTopLEntry; L],
    replace: [FixedTopLEntry; L],
) -> [FixedTopLEntry; L] {
    let mut selected = keep;
    for index in 0..L {
        selected[index] = select_top_l_entry(mask_usize, mask_i64, keep[index], replace[index]);
    }
    selected
}

fn select_public_round_work_counts(
    mask_usize: usize,
    keep: FixedSclPublicRoundWorkCounts,
    replace: FixedSclPublicRoundWorkCounts,
) -> FixedSclPublicRoundWorkCounts {
    FixedSclPublicRoundWorkCounts {
        parent_capacity: select_usize(mask_usize, keep.parent_capacity, replace.parent_capacity),
        first_child_capacity: select_usize(
            mask_usize,
            keep.first_child_capacity,
            replace.first_child_capacity,
),
        repeated_child_capacity: select_usize(
            mask_usize,
            keep.repeated_child_capacity,
            replace.repeated_child_capacity,
),
        list_size: select_usize(mask_usize, keep.list_size, replace.list_size),
        rounds: select_usize(mask_usize, keep.rounds, replace.rounds),
        top_l_compare_exchanges: select_usize(
            mask_usize,
            keep.top_l_compare_exchanges,
            replace.top_l_compare_exchanges,
),
        child_slots_written: select_usize(
            mask_usize,
            keep.child_slots_written,
            replace.child_slots_written,
),
        compacted_slots_written: select_usize(
            mask_usize,
            keep.compacted_slots_written,
            replace.compacted_slots_written,
),
    }
}

fn select_path_buffer<const CAP: usize, const N: usize>(
    mask_i64: i64,
    mask_u8: u8,
    keep: FixedSclPathBuffer<CAP, N>,
    replace: FixedSclPathBuffer<CAP, N>,
) -> FixedSclPathBuffer<CAP, N> {
    let mut selected = keep;
    for index in 0..CAP {
        selected.slots[index] =
            select_candidate(mask_i64, mask_u8, keep.slots[index], replace.slots[index]);
    }
    selected
}

fn select_round(
    mask_usize: usize,
    mask_i64: i64,
    keep: FixedSclRound,
    replace: FixedSclRound,
) -> FixedSclRound {
    FixedSclRound {
        bit_index: select_usize(mask_usize, keep.bit_index, replace.bit_index),
        bit0_metric_delta: select_i64(mask_i64, keep.bit0_metric_delta, replace.bit0_metric_delta),
        bit1_metric_delta: select_i64(mask_i64, keep.bit1_metric_delta, replace.bit1_metric_delta),
    }
}

fn select_candidate<const N: usize>(
    mask_i64: i64,
    mask_u8: u8,
    keep: FixedSclCandidate<N>,
    replace: FixedSclCandidate<N>,
) -> FixedSclCandidate<N> {
    let mut bits = [0u8; N];
    for index in 0..N {
        bits[index] = select_u8(mask_u8, keep.bits[index], replace.bits[index]);
    }
    FixedSclCandidate {
        metric: select_i64(mask_i64, keep.metric, replace.metric),
        bits,
        active: select_u8(mask_u8, keep.active, replace.active),
    }
}

fn sanitize_candidate_active<const N: usize>(
    candidate: FixedSclCandidate<N>,
) -> FixedSclCandidate<N> {
    let active = candidate.active & 1;
    let active_i64_mask = 0i64.wrapping_sub(i64::from(active));
    let active_u8_mask = 0u8.wrapping_sub(active);
    FixedSclCandidate {
        metric: select_i64(active_i64_mask, i64::MAX, candidate.metric),
        bits: mask_bits(candidate.bits, active_u8_mask),
        active,
    }
}

fn mask_bits<const N: usize>(bits: [u8; N], mask: u8) -> [u8; N] {
    let mut masked = [0u8; N];
    for index in 0..N {
        masked[index] = bits[index] & mask;
    }
    masked
}

fn fixed_scl_metric_add(parent_metric: i64, metric_delta: i64) -> i64 {
    let parent_forbidden = (parent_metric == i64::MAX) as i64;
    let delta_forbidden = (metric_delta == FIXED_SCL_FORBIDDEN_METRIC_DELTA) as i64;
    let forbidden = parent_forbidden | delta_forbidden;
    let forbidden_mask = 0i64.wrapping_sub(forbidden);
    let sum = parent_metric.saturating_add(metric_delta);

    select_i64(forbidden_mask, sum, FIXED_SCL_FORBIDDEN_METRIC_DELTA)
}

macro_rules! dev_research_evidence_sources_json_field {
    () => {
        "\"dev_research_evidence_sources\": \"916 profile distinguishing test; 917 C-only invariant identity; Cauchy-Schwarz reach/weight lens; conj:pencil route-a obstruction\""
    };
}

macro_rules! dev_research_residual_boundary_json_field {
    () => {
        "\"lem_m2_residual\": \"correlated Be noise detection gap\""
    };
}

macro_rules! dev_research_status_json_field {
    () => {
        "\"research_status\": \"OPEN\""
    };
}

macro_rules! dev_research_c_only_invariant_limit_json_field {
    () => {
        "\"c_only_invariant_limit\": \"dev 917: C-only quadratic-form invariant depends on rank, not isotropy; not a closure route\""
    };
}

macro_rules! dev_research_weight_observable_limit_json_field {
    () => {
        "\"weight_observable_limit\": \"Cauchy-Schwarz lens: B-agnostic weight and min-syndrome-weight signals vanish; remaining signal is <=2n-dimensional correlation of Be\""
    };
}

macro_rules! dev_research_runtime_dependency_json_field {
    () => {
        "\"dev_research_runtime_dependency\": false"
    };
}

macro_rules! dev_research_paper_claim_dependency_json_field {
    () => {
        "\"dev_research_paper_claim_dependency\": false"
    };
}

macro_rules! dev_research_use_json_field {
    () => {
        "\"dev_research_use\": \"audit metadata only; does not change decoder behavior or paper claim level\""
    };
}

macro_rules! dev_research_next_step_json_field {
    () => {
        "\"dev_research_next_step\": \"look for a B-agnostic functional of correlated Be; not C-only, not weight-only, and no claim upgrade without proof\""
    };
}

macro_rules! dev_research_limit_json_field {
    () => {
        "\"dev_research_limit\": \"sharpened but not closed: lem:m2 remains OPEN and conj:pencil remains OPEN; no break, no reduction, and no security claim\""
    };
}

macro_rules! dev_research_claim_boundary_json_field {
    () => {
        "\"research_claim_boundary\": \"OPEN; NOT A REDUCTION; NOT A SECURITY CLAIM; no production constant-time claim\""
    };
}

macro_rules! dev_research_closes_lem_m2_json_field {
    () => {
        "\"dev_research_closes_lem_m2\": false"
    };
}

macro_rules! dev_research_closes_conj_pencil_json_field {
    () => {
        "\"dev_research_closes_conj_pencil\": false"
    };
}

macro_rules! dev_research_reduction_claim_json_field {
    () => {
        "\"dev_research_reduction_claim\": false"
    };
}

macro_rules! dev_research_security_claim_json_field {
    () => {
        "\"dev_research_security_claim\": false"
    };
}

macro_rules! dev_research_claim_status_summary_json_field {
    () => {
        "\"dev_research_claim_status_summary\": \"audit metadata only: lem:m2 not closed, conj:pencil not closed, no reduction claim, and no security claim\""
    };
}

macro_rules! dev_research_handoff_summary_json_field {
    () => {
        "\"dev_research_handoff_summary\": \"handoff summary: randomized marginal-adaptive lem:m2 residual remains open; matched-rate LPN_{p_eff(n)} comparison against the marginal-uniform B=g(A) candidate family on the same public invariant screen; C=BA public min-coset-weight / syndrome-coset observable; no reduction claim and no security claim\""
    };
}

macro_rules! dev_research_pr_summary_json_field {
    () => {
        "\"dev_research_pr_summary\": \"PR summary: Rust audit metadata only; origin/dev-aligned open lem:m2 residual on the same public invariant screen; no production constant-time claim; no reduction claim and no security claim\""
    };
}

macro_rules! dev_research_alignment_summary_json_field {
    () => {
        "\"dev_research_alignment_summary\": \"alignment summary: origin/dev research remains open; matched-rate LPN_{p_eff(n)} and marginal-uniform B=g(A) are compared on the same public invariant screen; C=BA public min-coset-weight / syndrome-coset observable; randomized marginal-adaptive single-block / fresh-B residual; no proof upgrade, no reduction claim and no security claim\""
    };
}

macro_rules! dev_research_candidate_family_json_field {
    () => {
        "\"dev_research_candidate_family\": \"candidate-family requirement: compare an actual candidate family, marginal-uniform B=g(A), against matched-rate LPN_{p_eff(n)} on the same public invariant screen using the C=BA public min-coset-weight / syndrome-coset observable; audit-only, not a reduction, and not a security claim\""
    };
}

macro_rules! dev_research_matched_rate_comparison_json_field {
    () => {
        "\"dev_research_matched_rate_comparison\": \"matched-rate comparison: compare matched-rate LPN_{p_eff(n)} with the marginal-uniform B=g(A) candidate family on the same public invariant screen using the C=BA public min-coset-weight / syndrome-coset observable in the randomized marginal-adaptive setting; lem:m2 remains OPEN, not a reduction, and not a security claim\""
    };
}

macro_rules! dev_research_public_observable_json_field {
    () => {
        "\"dev_research_public_observable\": \"public observable boundary: C=BA, public min-coset-weight, and syndrome-coset observable are public invariant screen metadata only; no secret recovery, no trapdoor, no hidden-state oracle, and not a security claim\""
    };
}

macro_rules! dev_research_invariant_screen_json_field {
    () => {
        "\"dev_research_invariant_screen\": \"same public invariant screen: matched-rate LPN_{p_eff(n)} target; marginal-uniform B=g(A) candidate family; C=BA public min-coset-weight / syndrome-coset observable\""
    };
}

macro_rules! dev_research_marginal_adaptive_residual_json_field {
    () => {
        "\"dev_research_marginal_adaptive_residual\": \"randomized marginal-adaptive lem:m2 residual in the single-block / fresh-B setting; audit-only metadata with no reduction claim and no security claim\""
    };
}

macro_rules! dev_research_single_block_fresh_b_json_field {
    () => {
        "\"dev_research_single_block_fresh_b\": \"single-block / fresh-B residual screen: randomized marginal-adaptive lem:m2 remains OPEN on the public invariant screen; compare marginal-uniform B=g(A) with matched-rate LPN_{p_eff(n)}; audit metadata only, not a reduction, and not a security claim\""
    };
}

macro_rules! dev_research_fresh_evidence_target_json_field {
    () => {
        "\"dev_research_fresh_evidence_target\": \"conj:m2 fresh evidence target: n=3,4 marginal-adaptive fresh evidence on the same public invariant screen; remains OPEN and audit-only\""
    };
}

macro_rules! dev_research_archive_plan_use_json_field {
    () => {
        "\"dev_research_archive_plan_use\": \"archive/-paper-alignment-next-increment is a method reference only; do not reuse archived numeric outputs; no drop-in numbers\""
    };
}

macro_rules! dev_research_fresh_recompute_requirement_json_field {
    () => {
        "\"dev_research_fresh_recompute_requirement\": \"fresh recomputation requirement: Rust-first n=3,4 evidence must be recomputed on current dev; Track A verifies before paper use; NOT A REDUCTION and NOT A SECURITY CLAIM\""
    };
}

macro_rules! dev_research_dev_sync_json_field {
    () => {
        "\"dev_research_dev_sync\": \"origin/dev research sync: Cauchy-Schwarz lens sharpens lem:m2 but does not close it; joint-C + distinguishing reduce to lem:m2; conj:pencil attack routes not closed; flawed Fourier shortcut rejected\""
    };
}

macro_rules! dev_research_evidence_limit_json_field {
    () => {
        "\"dev_research_evidence_limit\": \"external confirmation does not upgrade the claim; public data only; analytic open problem remains\""
    };
}

macro_rules! dev_research_public_metadata_boundary_json_field {
    () => {
        "\"dev_research_public_metadata_boundary\": \"public metadata boundary: C, BA, and invariant-screen outputs only; no secret recovery, no trapdoor, and no hidden-state oracle\""
    };
}

macro_rules! dev_research_analytic_open_problem_json_field {
    () => {
        "\"dev_research_analytic_open_problem\": \"analytic open problem: separate marginal-uniform B=g(A) candidate family from matched-rate LPN_{p_eff(n)} on the same public invariant screen in the randomized marginal-adaptive single-block / fresh-B setting\""
    };
}

macro_rules! dev_research_evidence_source_set_json_field {
    () => {
        "\"dev_research_evidence_source_set\": \"OFA,, and  evidence source set; external confirmation and cross-check only; no proof upgrade\""
    };
}

macro_rules! dev_research_scl_audit_boundary_json_field {
    () => {
        "\"dev_research_scl_audit_boundary\": \"SCL audit boundary: variable-shape reference decoder; audit-only contract; not production constant-time; does not discharge lem:m2 or conj:pencil\""
    };
}

macro_rules! dev_research_audit_contract_json_lines {
    () => {
        concat!(
            "  ",
            dev_research_status_json_field!(),
            ",\n",
            "  ",
            dev_research_residual_boundary_json_field!(),
            ",\n",
            "  ",
            dev_research_c_only_invariant_limit_json_field!(),
            ",\n",
            "  ",
            dev_research_weight_observable_limit_json_field!(),
            ",\n",
            "  ",
            dev_research_runtime_dependency_json_field!(),
            ",\n",
            "  ",
            dev_research_paper_claim_dependency_json_field!(),
            ",\n",
            "  ",
            dev_research_use_json_field!(),
            ",\n",
            "  ",
            dev_research_evidence_sources_json_field!(),
            ",\n",
            "  ",
            dev_research_next_step_json_field!(),
            ",\n",
            "  ",
            dev_research_limit_json_field!(),
            ",\n",
            "  ",
            dev_research_closes_lem_m2_json_field!(),
            ",\n",
            "  ",
            dev_research_closes_conj_pencil_json_field!(),
            ",\n",
            "  ",
            dev_research_reduction_claim_json_field!(),
            ",\n",
            "  ",
            dev_research_security_claim_json_field!(),
            ",\n",
            "  ",
            dev_research_claim_status_summary_json_field!(),
            ",\n",
            "  ",
            dev_research_handoff_summary_json_field!(),
            ",\n",
            "  ",
            dev_research_pr_summary_json_field!(),
            ",\n",
            "  ",
            dev_research_alignment_summary_json_field!(),
            ",\n",
            "  ",
            dev_research_candidate_family_json_field!(),
            ",\n",
            "  ",
            dev_research_matched_rate_comparison_json_field!(),
            ",\n",
            "  ",
            dev_research_public_observable_json_field!(),
            ",\n",
            "  ",
            dev_research_invariant_screen_json_field!(),
            ",\n",
            "  ",
            dev_research_marginal_adaptive_residual_json_field!(),
            ",\n",
            "  ",
            dev_research_single_block_fresh_b_json_field!(),
            ",\n",
            "  ",
            dev_research_fresh_evidence_target_json_field!(),
            ",\n",
            "  ",
            dev_research_archive_plan_use_json_field!(),
            ",\n",
            "  ",
            dev_research_fresh_recompute_requirement_json_field!(),
            ",\n",
            "  ",
            dev_research_dev_sync_json_field!(),
            ",\n",
            "  ",
            dev_research_evidence_limit_json_field!(),
            ",\n",
            "  ",
            dev_research_evidence_source_set_json_field!(),
            ",\n",
            "  ",
            dev_research_public_metadata_boundary_json_field!(),
            ",\n",
            "  ",
            dev_research_analytic_open_problem_json_field!(),
            ",\n",
            "  ",
            dev_research_scl_audit_boundary_json_field!(),
            ",\n",
            "  ",
            dev_research_claim_boundary_json_field!(),
            ",\n",
)
    };
}

pub fn dev_research_evidence_sources_json_field() -> &'static str {
    dev_research_evidence_sources_json_field!()
}

pub fn dev_research_residual_boundary_json_field() -> &'static str {
    dev_research_residual_boundary_json_field!()
}

pub fn dev_research_status_json_field() -> &'static str {
    dev_research_status_json_field!()
}

pub fn dev_research_c_only_invariant_limit_json_field() -> &'static str {
    dev_research_c_only_invariant_limit_json_field!()
}

pub fn dev_research_weight_observable_limit_json_field() -> &'static str {
    dev_research_weight_observable_limit_json_field!()
}

pub fn dev_research_runtime_dependency_json_field() -> &'static str {
    dev_research_runtime_dependency_json_field!()
}

pub fn dev_research_paper_claim_dependency_json_field() -> &'static str {
    dev_research_paper_claim_dependency_json_field!()
}

pub fn dev_research_use_json_field() -> &'static str {
    dev_research_use_json_field!()
}

pub fn dev_research_next_step_json_field() -> &'static str {
    dev_research_next_step_json_field!()
}

pub fn dev_research_limit_json_field() -> &'static str {
    dev_research_limit_json_field!()
}

pub fn dev_research_claim_boundary_json_field() -> &'static str {
    dev_research_claim_boundary_json_field!()
}

pub fn dev_research_closes_lem_m2_json_field() -> &'static str {
    dev_research_closes_lem_m2_json_field!()
}

pub fn dev_research_closes_conj_pencil_json_field() -> &'static str {
    dev_research_closes_conj_pencil_json_field!()
}

pub fn dev_research_reduction_claim_json_field() -> &'static str {
    dev_research_reduction_claim_json_field!()
}

pub fn dev_research_security_claim_json_field() -> &'static str {
    dev_research_security_claim_json_field!()
}

pub fn dev_research_claim_status_summary_json_field() -> &'static str {
    dev_research_claim_status_summary_json_field!()
}

pub fn dev_research_handoff_summary_json_field() -> &'static str {
    dev_research_handoff_summary_json_field!()
}

pub fn dev_research_pr_summary_json_field() -> &'static str {
    dev_research_pr_summary_json_field!()
}

pub fn dev_research_alignment_summary_json_field() -> &'static str {
    dev_research_alignment_summary_json_field!()
}

pub fn dev_research_candidate_family_json_field() -> &'static str {
    dev_research_candidate_family_json_field!()
}

pub fn dev_research_matched_rate_comparison_json_field() -> &'static str {
    dev_research_matched_rate_comparison_json_field!()
}

pub fn dev_research_public_observable_json_field() -> &'static str {
    dev_research_public_observable_json_field!()
}

pub fn dev_research_invariant_screen_json_field() -> &'static str {
    dev_research_invariant_screen_json_field!()
}

pub fn dev_research_marginal_adaptive_residual_json_field() -> &'static str {
    dev_research_marginal_adaptive_residual_json_field!()
}

pub fn dev_research_single_block_fresh_b_json_field() -> &'static str {
    dev_research_single_block_fresh_b_json_field!()
}

pub fn dev_research_fresh_evidence_target_json_field() -> &'static str {
    dev_research_fresh_evidence_target_json_field!()
}

pub fn dev_research_archive_plan_use_json_field() -> &'static str {
    dev_research_archive_plan_use_json_field!()
}

pub fn dev_research_fresh_recompute_requirement_json_field() -> &'static str {
    dev_research_fresh_recompute_requirement_json_field!()
}

pub fn dev_research_dev_sync_json_field() -> &'static str {
    dev_research_dev_sync_json_field!()
}

pub fn dev_research_evidence_limit_json_field() -> &'static str {
    dev_research_evidence_limit_json_field!()
}

pub fn dev_research_public_metadata_boundary_json_field() -> &'static str {
    dev_research_public_metadata_boundary_json_field!()
}

pub fn dev_research_analytic_open_problem_json_field() -> &'static str {
    dev_research_analytic_open_problem_json_field!()
}

pub fn dev_research_evidence_source_set_json_field() -> &'static str {
    dev_research_evidence_source_set_json_field!()
}

pub fn dev_research_scl_audit_boundary_json_field() -> &'static str {
    dev_research_scl_audit_boundary_json_field!()
}

pub fn dev_research_claim_flag_json_fields() -> [&'static str; 4] {
    [
        dev_research_closes_lem_m2_json_field(),
        dev_research_closes_conj_pencil_json_field(),
        dev_research_reduction_claim_json_field(),
        dev_research_security_claim_json_field(),
    ]
}

pub const fn dev_research_audit_contract_field_count() -> usize {
    34
}

pub fn dev_research_audit_contract_field_keys(
) -> [&'static str; dev_research_audit_contract_field_count()] {
    [
        "\"research_status\":",
        "\"lem_m2_residual\":",
        "\"c_only_invariant_limit\":",
        "\"weight_observable_limit\":",
        "\"dev_research_runtime_dependency\":",
        "\"dev_research_paper_claim_dependency\":",
        "\"dev_research_use\":",
        "\"dev_research_evidence_sources\":",
        "\"dev_research_next_step\":",
        "\"dev_research_limit\":",
        "\"dev_research_closes_lem_m2\":",
        "\"dev_research_closes_conj_pencil\":",
        "\"dev_research_reduction_claim\":",
        "\"dev_research_security_claim\":",
        "\"dev_research_claim_status_summary\":",
        "\"dev_research_handoff_summary\":",
        "\"dev_research_pr_summary\":",
        "\"dev_research_alignment_summary\":",
        "\"dev_research_candidate_family\":",
        "\"dev_research_matched_rate_comparison\":",
        "\"dev_research_public_observable\":",
        "\"dev_research_invariant_screen\":",
        "\"dev_research_marginal_adaptive_residual\":",
        "\"dev_research_single_block_fresh_b\":",
        "\"dev_research_fresh_evidence_target\":",
        "\"dev_research_archive_plan_use\":",
        "\"dev_research_fresh_recompute_requirement\":",
        "\"dev_research_dev_sync\":",
        "\"dev_research_evidence_limit\":",
        "\"dev_research_evidence_source_set\":",
        "\"dev_research_public_metadata_boundary\":",
        "\"dev_research_analytic_open_problem\":",
        "\"dev_research_scl_audit_boundary\":",
        "\"research_claim_boundary\":",
    ]
}

pub fn dev_research_audit_contract_boundary_keys() -> [&'static str; 4] {
    [
        "\"dev_research_public_metadata_boundary\":",
        "\"dev_research_analytic_open_problem\":",
        "\"dev_research_scl_audit_boundary\":",
        "\"research_claim_boundary\":",
    ]
}

pub fn dev_research_audit_contract_boundary_fields() -> [&'static str; 4] {
    [
        dev_research_public_metadata_boundary_json_field(),
        dev_research_analytic_open_problem_json_field(),
        dev_research_scl_audit_boundary_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_evidence_fields() -> [&'static str; 3] {
    [
        dev_research_dev_sync_json_field(),
        dev_research_evidence_limit_json_field(),
        dev_research_evidence_source_set_json_field(),
    ]
}

pub fn dev_research_audit_contract_tail_fields() -> [&'static str; 7] {
    [
        dev_research_dev_sync_json_field(),
        dev_research_evidence_limit_json_field(),
        dev_research_evidence_source_set_json_field(),
        dev_research_public_metadata_boundary_json_field(),
        dev_research_analytic_open_problem_json_field(),
        dev_research_scl_audit_boundary_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_handoff_fields() -> [&'static str; 3] {
    [
        dev_research_handoff_summary_json_field(),
        dev_research_next_step_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_pr_fields() -> [&'static str; 3] {
    [
        dev_research_pr_summary_json_field(),
        dev_research_handoff_summary_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_claim_posture_fields() -> [&'static str; 8] {
    [
        dev_research_closes_lem_m2_json_field(),
        dev_research_closes_conj_pencil_json_field(),
        dev_research_reduction_claim_json_field(),
        dev_research_security_claim_json_field(),
        dev_research_claim_status_summary_json_field(),
        dev_research_handoff_summary_json_field(),
        dev_research_pr_summary_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_research_target_fields() -> [&'static str; 5] {
    [
        dev_research_handoff_summary_json_field(),
        dev_research_invariant_screen_json_field(),
        dev_research_marginal_adaptive_residual_json_field(),
        dev_research_public_metadata_boundary_json_field(),
        dev_research_analytic_open_problem_json_field(),
    ]
}

pub fn dev_research_audit_contract_candidate_family_fields() -> [&'static str; 5] {
    [
        dev_research_candidate_family_json_field(),
        dev_research_alignment_summary_json_field(),
        dev_research_invariant_screen_json_field(),
        dev_research_analytic_open_problem_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_comparison_fields() -> [&'static str; 5] {
    [
        dev_research_matched_rate_comparison_json_field(),
        dev_research_candidate_family_json_field(),
        dev_research_invariant_screen_json_field(),
        dev_research_marginal_adaptive_residual_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_observable_fields() -> [&'static str; 5] {
    [
        dev_research_public_observable_json_field(),
        dev_research_invariant_screen_json_field(),
        dev_research_public_metadata_boundary_json_field(),
        dev_research_matched_rate_comparison_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_residual_screen_fields() -> [&'static str; 6] {
    [
        dev_research_single_block_fresh_b_json_field(),
        dev_research_marginal_adaptive_residual_json_field(),
        dev_research_matched_rate_comparison_json_field(),
        dev_research_candidate_family_json_field(),
        dev_research_invariant_screen_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_fresh_evidence_fields() -> [&'static str; 4] {
    [
        dev_research_fresh_evidence_target_json_field(),
        dev_research_archive_plan_use_json_field(),
        dev_research_fresh_recompute_requirement_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_public_screen_fields() -> [&'static str; 7] {
    [
        dev_research_candidate_family_json_field(),
        dev_research_matched_rate_comparison_json_field(),
        dev_research_public_observable_json_field(),
        dev_research_invariant_screen_json_field(),
        dev_research_marginal_adaptive_residual_json_field(),
        dev_research_single_block_fresh_b_json_field(),
        dev_research_public_metadata_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_open_problem_fields() -> [&'static str; 6] {
    [
        dev_research_analytic_open_problem_json_field(),
        dev_research_next_step_json_field(),
        dev_research_evidence_limit_json_field(),
        dev_research_dev_sync_json_field(),
        dev_research_alignment_summary_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_alignment_fields() -> [&'static str; 7] {
    [
        dev_research_alignment_summary_json_field(),
        dev_research_invariant_screen_json_field(),
        dev_research_marginal_adaptive_residual_json_field(),
        dev_research_dev_sync_json_field(),
        dev_research_evidence_limit_json_field(),
        dev_research_evidence_source_set_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_json_fields(
) -> [&'static str; dev_research_audit_contract_field_count()] {
    [
        dev_research_status_json_field(),
        dev_research_residual_boundary_json_field(),
        dev_research_c_only_invariant_limit_json_field(),
        dev_research_weight_observable_limit_json_field(),
        dev_research_runtime_dependency_json_field(),
        dev_research_paper_claim_dependency_json_field(),
        dev_research_use_json_field(),
        dev_research_evidence_sources_json_field(),
        dev_research_next_step_json_field(),
        dev_research_limit_json_field(),
        dev_research_closes_lem_m2_json_field(),
        dev_research_closes_conj_pencil_json_field(),
        dev_research_reduction_claim_json_field(),
        dev_research_security_claim_json_field(),
        dev_research_claim_status_summary_json_field(),
        dev_research_handoff_summary_json_field(),
        dev_research_pr_summary_json_field(),
        dev_research_alignment_summary_json_field(),
        dev_research_candidate_family_json_field(),
        dev_research_matched_rate_comparison_json_field(),
        dev_research_public_observable_json_field(),
        dev_research_invariant_screen_json_field(),
        dev_research_marginal_adaptive_residual_json_field(),
        dev_research_single_block_fresh_b_json_field(),
        dev_research_fresh_evidence_target_json_field(),
        dev_research_archive_plan_use_json_field(),
        dev_research_fresh_recompute_requirement_json_field(),
        dev_research_dev_sync_json_field(),
        dev_research_evidence_limit_json_field(),
        dev_research_evidence_source_set_json_field(),
        dev_research_public_metadata_boundary_json_field(),
        dev_research_analytic_open_problem_json_field(),
        dev_research_scl_audit_boundary_json_field(),
        dev_research_claim_boundary_json_field(),
    ]
}

pub fn dev_research_audit_contract_json_lines() -> &'static str {
    dev_research_audit_contract_json_lines!()
}

pub fn marginal_invariant_smoke_claim_posture_json() -> &'static str {
    concat!(
        "{\n",
        marginal_invariant_audit_boundary_json_lines!(),
        "  \"related_ofa_evidence\": [\n",
        "    {\n",
        "      \"ofa\": \"OFA-359\",\n",
        "      \"finding\": \"low_degree_cube_sparsity_wall\",\n",
        "      \"claim_posture\": \"NOT A REDUCTION; NOT A SECURITY CLAIM\",\n",
        ofa_evidence_entry_audit_posture_json_lines!(),        "    },\n",
        "    {\n",
        "      \"ofa\": \"OFA-360\",\n",
        "      \"finding\": \"full_noise_law_not_preserved_by_nonlocal_sp\",\n",
        "      \"claim_posture\": \"NOT A REDUCTION; NOT A SECURITY CLAIM\",\n",
        ofa_evidence_entry_audit_posture_json_lines!(),        "    },\n",
        "    {\n",
        "      \"ofa\": \"OFA-398\",\n",
        "      \"finding\": \"omega_stress_margin_live_signal_wall_bound\",\n",
        "      \"claim_posture\": \"live refinement signal; NOT A REDUCTION\",\n",
        ofa_evidence_entry_audit_posture_json_lines!(),        "    },\n",
        "    {\n",
        "      \"ofa\": \"OFA-399\",\n",
        "      \"finding\": \"constant_rate_wall_n7\",\n",
        "      \"claim_posture\": \"wall-bound evidence; NOT A REDUCTION; NOT A SECURITY CLAIM\",\n",
        ofa_evidence_entry_audit_posture_json_lines!(),        "    }\n",
        "  ],\n",
        "  ",
        marginal_invariant_adjudication_json_field!(),
        "\n",
        "}\n",
)
}

pub fn scl_work_shape_audit_json() -> &'static str {
    concat!(
        "{\n",
        scl_work_shape_audit_boundary_json_lines!(),
        dev_research_audit_contract_json_lines!(),
        scl_work_shape_related_ofa_evidence_json_lines!(),
        scl_work_shape_audited_functions_json_lines!(),
        scl_work_shape_active_decoder_entrypoints_json_lines!(),
        scl_work_shape_variable_shape_surfaces_json_lines!(),
        scl_work_shape_fixed_schedule_requirements_json_lines!(),
        scl_work_shape_metric_domain_assumptions_json_lines!(),
        scl_work_shape_top_l_selection_domain_failure_codes_json_lines!(),
        scl_work_shape_public_path_domain_failure_codes_json_lines!(),
        scl_work_shape_public_child_write_failure_codes_json_lines!(),
        scl_work_shape_integer_schedule_domain_failure_codes_json_lines!(),
        scl_work_shape_integer_metric_domain_failure_codes_json_lines!(),
        scl_work_shape_integer_schedule_shape_failure_families_json_lines!(),
        scl_work_shape_public_round_schedule_shape_failure_families_json_lines!(),
        scl_work_shape_non_panicking_wrapper_failure_code_map_json_lines!(),
        scl_work_shape_prototype_building_blocks_json_lines!(),
        scl_work_shape_fixed_work_count_guardrails_json_lines!(),
        scl_work_shape_status_selection_guardrails_json_lines!(),
        scl_work_shape_public_domain_guardrails_json_lines!(),
        scl_work_shape_public_status_field_provenance_json_lines!(),
        scl_work_shape_public_status_selector_obligations_json_lines!(),
        scl_work_shape_public_shape_preservation_obligations_json_lines!(),
        scl_work_shape_public_failure_code_wrapper_obligations_json_lines!(),
        scl_work_shape_public_preflight_coverage_obligations_json_lines!(),
        scl_work_shape_public_invalid_input_status_reporting_obligations_json_lines!(),
        scl_work_shape_public_rejection_reason_taxonomy_json_lines!(),
        scl_work_shape_public_status_lifecycle_obligations_json_lines!(),
        scl_work_shape_public_status_transition_ordering_obligations_json_lines!(),
        scl_work_shape_public_status_schema_stability_obligations_json_lines!(),
        scl_work_shape_public_status_schema_compatibility_obligations_json_lines!(),
        scl_work_shape_public_status_schema_consumer_obligations_json_lines!(),
        scl_work_shape_public_status_schema_producer_obligations_json_lines!(),
        scl_work_shape_public_status_schema_round_trip_obligations_json_lines!(),
        scl_work_shape_public_status_schema_drift_obligations_json_lines!(),
        scl_work_shape_public_status_schema_migration_obligations_json_lines!(),
        scl_work_shape_public_status_schema_rollback_obligations_json_lines!(),
        scl_work_shape_public_status_schema_deprecation_obligations_json_lines!(),
        scl_work_shape_public_status_schema_removal_obligations_json_lines!(),
        scl_work_shape_public_status_schema_archival_obligations_json_lines!(),
        scl_work_shape_public_status_schema_sunset_obligations_json_lines!(),
        scl_work_shape_public_status_schema_retention_obligations_json_lines!(),
        scl_work_shape_public_status_schema_lifecycle_order_obligations_json_lines!(),
        scl_work_shape_public_status_schema_lifecycle_shape_obligations_json_lines!(),
        scl_work_shape_public_status_schema_lifecycle_registry_obligations_json_lines!(),
        scl_work_shape_public_status_schema_lifecycle_manifest_obligations_json_lines!(),
        scl_work_shape_public_status_schema_lifecycle_replay_obligations_json_lines!(),
        scl_work_shape_public_status_schema_lifecycle_digest_obligations_json_lines!(),
        scl_work_shape_public_status_schema_lifecycle_receipt_obligations_json_lines!(),
        scl_work_shape_public_status_schema_lifecycle_transcript_obligations_json_lines!(),
        scl_work_shape_public_status_schema_lifecycle_witness_obligations_json_lines!(),
        scl_work_shape_public_status_schema_lifecycle_audit_trail_obligations_json_lines!(),
        scl_work_shape_public_status_schema_lifecycle_checkpoint_obligations_json_lines!(),
        scl_work_shape_closure_boundary_json_lines!(),
        "}\n",
)
}

pub fn run_configs(configs: &[SimulationConfig]) -> Vec<SimulationResult> {
    configs
.iter()
.map(|cfg| simulate_bsc_sc(cfg.n, cfg.k, cfg.p, cfg.trials, cfg.seed))
.collect()
}

pub fn results_to_json(experiment: &str, results: &[SimulationResult]) -> String {
    results_to_json_with_decoder(experiment, "successive_cancellation_exact_llr", results)
}

pub fn results_to_json_with_decoder(
    experiment: &str,
    decoder: &str,
    results: &[SimulationResult],
) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!(
        "  \"experiment\": \"{}\",\n",
        escape_json(experiment)
));
    out.push_str(&format!("  \"decoder\": \"{}\",\n", escape_json(decoder)));
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"artifact_scope\": \"public audit artifact only\",\n");
    out.push_str("  \"frozen_set\": \"natural_order_bhattacharyya_z2i_bad_z2i1_good\",\n");
    out.push_str("  \"adjudication\": \"polar validation simulation only; no reduction claim, no security claim, OPEN = LSN\",\n");
    out.push_str("  \"results\": [\n");
    for (i, result) in results.iter().enumerate() {
        out.push_str("    {\n");
        out.push_str(&format!("      \"N\": {},\n", result.n));
        out.push_str(&format!("      \"K\": {},\n", result.k));
        out.push_str(&format!("      \"p\": {:.10},\n", result.p));
        out.push_str(&format!("      \"trials\": {},\n", result.trials));
        out.push_str(&format!("      \"errors\": {},\n", result.errors));
        out.push_str(&format!("      \"bler\": {:.10},\n", result.bler()));
        if let Some(upper) = result.zero_error_upper_95() {
            out.push_str(&format!("      \"zero_error_upper_95\": {:.10},\n", upper));
        }
        out.push_str(&format!("      \"seed\": {}\n", result.seed));
        out.push_str("    }");
        if i + 1 != results.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

pub fn importance_results_to_json(
    experiment: &str,
    decoder: &str,
    results: &[ImportanceSamplingResult],
) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!(
        "  \"experiment\": \"{}\",\n",
        escape_json(experiment)
));
    out.push_str(&format!("  \"decoder\": \"{}\",\n", escape_json(decoder)));
    out.push_str("  \"sampling\": \"tilted_bsc_proposal_reweighted_to_target_bsc\",\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"artifact_scope\": \"public audit artifact only\",\n");
    out.push_str("  \"frozen_set\": \"natural_order_bhattacharyya_z2i_bad_z2i1_good\",\n");
    out.push_str(
        "  \"adjudication\": \"P1b importance-sampling pilot; evidence, not proof; no reduction claim, no security claim, OPEN = LSN\",\n",
);
    out.push_str("  \"results\": [\n");
    for (i, result) in results.iter().enumerate() {
        out.push_str("    {\n");
        out.push_str(&format!("      \"N\": {},\n", result.n));
        out.push_str(&format!("      \"K\": {},\n", result.k));
        out.push_str(&format!("      \"target_p\": {:.10},\n", result.target_p));
        out.push_str(&format!(
            "      \"proposal_p\": {:.10},\n",
            result.proposal_p
));
        out.push_str(&format!("      \"trials\": {},\n", result.trials));
        out.push_str(&format!(
            "      \"proposal_errors\": {},\n",
            result.proposal_errors
));
        out.push_str(&format!(
            "      \"proposal_error_rate\": {:.10},\n",
            result.proposal_error_rate()
));
        out.push_str(&format!(
            "      \"weighted_bler_estimate\": {:.12e},\n",
            result.weighted_bler_estimate
));
        out.push_str(&format!(
            "      \"mean_likelihood_ratio\": {:.12e},\n",
            result.mean_likelihood_ratio
));
        out.push_str(&format!(
            "      \"effective_sample_size\": {:.6},\n",
            result.effective_sample_size
));
        out.push_str(&format!("      \"seed\": {}\n", result.seed));
        out.push_str("    }");
        if i + 1 != results.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

pub fn polar_rate_rows_to_json(
    experiment: &str,
    target_log2_half_sum_bound: f64,
    rows: &[PolarRateRow],
) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!(
        "  \"experiment\": \"{}\",\n",
        escape_json(experiment)
));
    out.push_str("  \"bound_convention\": \"SC block-error bound uses 0.5 * sum selected Bhattacharyya Z_i; raw sum is also recorded\",\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"artifact_scope\": \"public audit artifact only\",\n");
    out.push_str(&format!(
        "  \"target_log2_half_sum_bound\": {:.6},\n",
        target_log2_half_sum_bound
));
    out.push_str("  \"adjudication\": \"engineering polar-rate sweep only; no reduction claim, no security claim, no closure, no break, OPEN = LSN\",\n");
    out.push_str("  \"rows\": [\n");
    for (i, row) in rows.iter().enumerate() {
        out.push_str("    {\n");
        out.push_str(&format!("      \"N\": {},\n", row.n));
        out.push_str(&format!("      \"K\": {},\n", row.k));
        out.push_str(&format!("      \"p\": {:.10},\n", row.p));
        out.push_str(&format!("      \"rate\": {:.12},\n", row.rate));
        out.push_str(&format!(
            "      \"bhattacharyya_sum_bound\": {:.12e},\n",
            row.bhattacharyya_sum_bound
));
        out.push_str(&format!(
            "      \"half_sum_bound\": {:.12e},\n",
            row.half_sum_bound
));
        out.push_str(&format!(
            "      \"log2_bhattacharyya_sum_bound\": {:.6},\n",
            row.log2_bhattacharyya_sum_bound
));
        out.push_str(&format!(
            "      \"log2_half_sum_bound\": {:.6},\n",
            row.log2_half_sum_bound
));
        out.push_str(&format!(
            "      \"target_log2_half_sum_bound\": {:.6},\n",
            row.target_log2_half_sum_bound
));
        out.push_str(&format!(
            "      \"passes_half_sum_target\": {}\n",
            row.passes_half_sum_target
));
        out.push_str("    }");
        if i + 1 != rows.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

fn escape_json(value: &str) -> String {
    value
.chars()
.flat_map(|c| match c {
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            _ => vec![c],
        })
.collect()
}

impl PolarCode {
    pub fn new(n: usize, k: usize, p: f64) -> Self {
        assert!(n.is_power_of_two(), "N must be a power of two");
        assert!(k <= n, "K must be <= N");
        assert!((0.0..=0.5).contains(&p), "p must be in [0, 0.5]");

        let frozen = build_frozen_natural(n, k, p);
        let frozen_set = frozen.iter().copied().collect::<HashSet<_>>();
        let info_set = (0..n)
.filter(|idx| !frozen_set.contains(idx))
.collect::<Vec<_>>();
        let mut frozen_mask = vec![false; n];
        for &idx in &frozen {
            frozen_mask[idx] = true;
        }

        Self {
            n,
            k,
            p,
            frozen,
            info_set,
            frozen_mask,
        }
    }
}

pub fn build_frozen_natural(n: usize, k: usize, p: f64) -> Vec<usize> {
    assert!(n.is_power_of_two(), "N must be a power of two");
    assert!(k <= n, "K must be <= N");
    assert!((0.0..=0.5).contains(&p), "p must be in [0, 0.5]");

    let z = bhattacharyya_reliabilities(n, p);
    let mut order = (0..n).collect::<Vec<_>>();
    order.sort_by(|&a, &b| z[a].total_cmp(&z[b]).then_with(|| a.cmp(&b)));
    order[k..].to_vec()
}

pub fn bhattacharyya_reliabilities(n: usize, p: f64) -> Vec<f64> {
    assert!(n.is_power_of_two(), "N must be a power of two");
    assert!((0.0..=0.5).contains(&p), "p must be in [0, 0.5]");

    let z0 = 2.0 * (p * (1.0 - p)).sqrt();
    let mut z = vec![z0];
    while z.len() < n {
        let mut next = vec![0.0; z.len() * 2];
        for (i, value) in z.iter().copied().enumerate() {
            next[2 * i] = 2.0 * value - value * value;
            next[2 * i + 1] = value * value;
        }
        z = next;
    }

    z
}

pub fn polar_rate_row(n: usize, k: usize, p: f64, target_log2_half_sum_bound: f64) -> PolarRateRow {
    assert!(k <= n, "K must be <= N");
    let z = bhattacharyya_reliabilities(n, p);
    let mut order = (0..n).collect::<Vec<_>>();
    order.sort_by(|&a, &b| z[a].total_cmp(&z[b]).then_with(|| a.cmp(&b)));
    let bhattacharyya_sum_bound = order[..k].iter().map(|&idx| z[idx]).sum::<f64>();
    polar_rate_row_from_sum(n, k, p, target_log2_half_sum_bound, bhattacharyya_sum_bound)
}

pub fn sweep_polar_rate(
    n: usize,
    p_values: &[f64],
    k_start: usize,
    k_end: usize,
    k_step: usize,
    target_log2_half_sum_bound: f64,
) -> Vec<PolarRateRow> {
    assert!(k_step > 0, "K step must be positive");
    assert!(k_start <= k_end, "K start must be <= K end");
    assert!(k_end <= n, "K end must be <= N");

    let mut rows = Vec::new();
    for &p in p_values {
        let z = bhattacharyya_reliabilities(n, p);
        let mut order = (0..n).collect::<Vec<_>>();
        order.sort_by(|&a, &b| z[a].total_cmp(&z[b]).then_with(|| a.cmp(&b)));

        let mut prefix = vec![0.0; n + 1];
        for (i, &idx) in order.iter().enumerate() {
            prefix[i + 1] = prefix[i] + z[idx];
        }

        let mut k = k_start;
        while k <= k_end {
            rows.push(polar_rate_row_from_sum(
                n,
                k,
                p,
                target_log2_half_sum_bound,
                prefix[k],
));
            match k.checked_add(k_step) {
                Some(next) => k = next,
                None => break,
            }
        }
    }
    rows
}

fn polar_rate_row_from_sum(
    n: usize,
    k: usize,
    p: f64,
    target_log2_half_sum_bound: f64,
    bhattacharyya_sum_bound: f64,
) -> PolarRateRow {
    let half_sum_bound = 0.5 * bhattacharyya_sum_bound;
    let log2_bhattacharyya_sum_bound = bhattacharyya_sum_bound.log2();
    let log2_half_sum_bound = half_sum_bound.log2();

    PolarRateRow {
        n,
        k,
        p,
        rate: k as f64 / n as f64,
        bhattacharyya_sum_bound,
        half_sum_bound,
        log2_bhattacharyya_sum_bound,
        log2_half_sum_bound,
        target_log2_half_sum_bound,
        passes_half_sum_target: log2_half_sum_bound <= target_log2_half_sum_bound,
    }
}

pub fn encode(code: &PolarCode, message: &[u8]) -> Vec<u8> {
    assert_eq!(message.len(), code.k, "message length must equal K");
    let mut u = vec![0u8; code.n];
    for (&idx, &bit) in code.info_set.iter().zip(message.iter()) {
        u[idx] = bit & 1;
    }
    polar_transform(&mut u);
    u
}

pub fn decode_successive_cancellation(code: &PolarCode, llr: &[f64]) -> Vec<u8> {
    assert_eq!(llr.len(), code.n, "LLR length must equal N");
    let u_hat = sc_decode_node(llr, 0, &code.frozen_mask);
    code.info_set.iter().map(|&idx| u_hat[idx]).collect()
}

pub fn decode_scl(code: &PolarCode, llr: &[f64], list_size: usize) -> Vec<u8> {
    assert_eq!(llr.len(), code.n, "LLR length must equal N");
    assert!(list_size > 0, "list size must be positive");
    let paths = scl_decode_node(llr, 0, &code.frozen_mask, list_size);
    let best = paths
.first()
.expect("SCL decoder must produce at least one path");
    code.info_set.iter().map(|&idx| best.bits[idx]).collect()
}

pub fn decode_scl_fast(code: &PolarCode, llr: &[f64], list_size: usize) -> Vec<u8> {
    assert_eq!(llr.len(), code.n, "LLR length must equal N");
    assert!(list_size > 0, "list size must be positive");
    let mut paths = vec![SclPath {
        bits: vec![0; code.n],
        metric: 0.0,
    }];

    for phi in 0..code.n {
        let mut next_paths = Vec::with_capacity(paths.len() * 2);
        for path in &paths {
            let bit_llr = sc_bit_llr_minsum(llr, 0, phi, &path.bits);
            if code.frozen_mask[phi] {
                let mut bits = path.bits.clone();
                bits[phi] = 0;
                next_paths.push(SclPath {
                    bits,
                    metric: path.metric + bit_metric_minsum(bit_llr, 0),
                });
            } else {
                for bit in [0u8, 1u8] {
                    let mut bits = path.bits.clone();
                    bits[phi] = bit;
                    next_paths.push(SclPath {
                        bits,
                        metric: path.metric + bit_metric_minsum(bit_llr, bit),
                    });
                }
            }
        }
        prune_paths(&mut next_paths, list_size);
        paths = next_paths;
    }

    let best = paths.first().expect("fast SCL must keep at least one path");
    code.info_set.iter().map(|&idx| best.bits[idx]).collect()
}

pub fn decode_scl_fixed_i64<const N: usize, const L: usize, const CHILD_CAP: usize>(
    code: &PolarCode,
    llr: &[f64],
    metric_scale: f64,
) -> Vec<u8> {
    assert_eq!(code.n, N, "fixed SCL const N must match PolarCode N");
    assert_eq!(llr.len(), N, "LLR length must equal fixed SCL N");
    assert!(L > 0, "fixed SCL list size must be positive");
    assert!(
        CHILD_CAP >= L.saturating_mul(2),
        "fixed SCL child capacity must hold two children per path"
);
    assert!(
        metric_scale.is_finite() && metric_scale > 0.0,
        "fixed SCL metric scale must be positive and finite"
);

    let quantized_llr = quantize_llrs_i64::<N>(llr, metric_scale);
    let mut paths = FixedSclPathBuffer::<L, N>::new();
    paths.set_candidate(0, 0, [0; N]);
    let mut llr_scratch = [0i64; N];
    let mut partial_scratch = [0u8; N];

    for phi in 0..N {
        let mut children = FixedSclPathBuffer::<CHILD_CAP, N>::new();
        for parent_slot in 0..L {
            let bits = paths.bits(parent_slot);
            let bit_llr = sc_bit_llr_minsum_i64(
                &quantized_llr,
                0,
                phi,
                &bits,
                &mut llr_scratch,
                &mut partial_scratch,
);
            let hard_bit = i64_negative_flag(bit_llr);
            let magnitude = llr_i64_metric_magnitude(bit_llr);
            let deltas =
                fixed_scl_integer_metric_deltas(code.frozen_mask[phi], hard_bit, magnitude);
            children.write_binary_children_from(
                &paths,
                parent_slot,
                parent_slot * 2,
                phi,
                deltas.bit0_metric_delta,
                deltas.bit1_metric_delta,
);
        }
        let top = children.top_l_entries::<L>();
        paths = FixedSclPathBuffer::<L, N>::from_top_entries(&children, top);
    }

    let best_bits = paths.bits(0);
    code.info_set.iter().map(|&idx| best_bits[idx]).collect()
}

pub fn decode_scl_fixed_i64_l8_validation(code: &PolarCode, llr: &[f64]) -> Vec<u8> {
    match code.n {
        128 => decode_scl_fixed_i64::<128, 8, 16>(code, llr, FIXED_I64_VALIDATION_METRIC_SCALE),
        256 => decode_scl_fixed_i64::<256, 8, 16>(code, llr, FIXED_I64_VALIDATION_METRIC_SCALE),
        512 => decode_scl_fixed_i64::<512, 8, 16>(code, llr, FIXED_I64_VALIDATION_METRIC_SCALE),
        2048 => decode_scl_fixed_i64::<2048, 8, 16>(code, llr, FIXED_I64_VALIDATION_METRIC_SCALE),
        other => panic!("fixed-i64 L8 validation decode does not support N={other}"),
    }
}

fn i64_negative_flag(value: i64) -> u8 {
    ((value as u64) >> 63) as u8
}

fn llr_metric_magnitude_i64(llr: f64, metric_scale: f64) -> i64 {
    let cap = i64::MAX / 4;
    let scaled = (llr.abs() * metric_scale).round();
    let nonfinite = (!scaled.is_finite()) as i64;
    let too_large = (scaled >= cap as f64) as i64;
    let out_of_range = nonfinite | too_large;
    let range_mask = 0i64.wrapping_sub(out_of_range);
    let candidate = scaled as i64;

    select_i64(range_mask, candidate, cap)
}

fn quantize_llrs_i64<const N: usize>(llr: &[f64], metric_scale: f64) -> [i64; N] {
    assert_eq!(llr.len(), N, "fixed SCL LLR length must equal N");
    let mut quantized = [0i64; N];
    for (dst, &value) in quantized.iter_mut().zip(llr.iter()) {
        *dst = quantize_llr_i64(value, metric_scale);
    }
    quantized
}

fn quantize_llr_i64(llr: f64, metric_scale: f64) -> i64 {
    let magnitude = llr_metric_magnitude_i64(llr, metric_scale);
    let negative = (llr < 0.0) as i64;
    let sign_mask = 0i64.wrapping_sub(negative);

    select_i64(sign_mask, magnitude, magnitude.saturating_neg())
}

fn llr_i64_metric_magnitude(llr: i64) -> i64 {
    i64_abs_saturating(llr).min(i64::MAX / 4)
}

fn i64_abs_saturating(value: i64) -> i64 {
    value.saturating_abs()
}

pub fn simulate_bsc_sc(n: usize, k: usize, p: f64, trials: usize, seed: u64) -> SimulationResult {
    let code = PolarCode::new(n, k, p);
    let mut rng = Lcg64::new(seed);
    let llr0 = ((1.0 - p) / p).ln();
    let llr1 = -llr0;
    let mut errors = 0usize;

    for _ in 0..trials {
        let message = (0..k)
.map(|_| if rng.next_bool() { 1 } else { 0 })
.collect::<Vec<_>>();
        let x = encode(&code, &message);
        let llr = x
.iter()
.map(|&bit| {
                let flipped = rng.next_f64() < p;
                let y = bit ^ u8::from(flipped);
                if y == 0 {
                    llr0
                } else {
                    llr1
                }
            })
.collect::<Vec<_>>();
        let decoded = decode_successive_cancellation(&code, &llr);
        if decoded != message {
            errors += 1;
        }
    }

    SimulationResult {
        n,
        k,
        p,
        trials,
        errors,
        seed,
    }
}

pub fn simulate_bsc_scl(
    n: usize,
    k: usize,
    p: f64,
    trials: usize,
    seed: u64,
    list_size: usize,
) -> SimulationResult {
    let code = PolarCode::new(n, k, p);
    let mut rng = Lcg64::new(seed);
    let llr0 = ((1.0 - p) / p).ln();
    let llr1 = -llr0;
    let mut errors = 0usize;

    for _ in 0..trials {
        let message = (0..k)
.map(|_| if rng.next_bool() { 1 } else { 0 })
.collect::<Vec<_>>();
        let x = encode(&code, &message);
        let llr = x
.iter()
.map(|&bit| {
                let flipped = rng.next_f64() < p;
                let y = bit ^ u8::from(flipped);
                if y == 0 {
                    llr0
                } else {
                    llr1
                }
            })
.collect::<Vec<_>>();
        let decoded = decode_scl(&code, &llr, list_size);
        if decoded != message {
            errors += 1;
        }
    }

    SimulationResult {
        n,
        k,
        p,
        trials,
        errors,
        seed,
    }
}

pub fn simulate_bsc_scl_fast(
    n: usize,
    k: usize,
    p: f64,
    trials: usize,
    seed: u64,
    list_size: usize,
) -> SimulationResult {
    let code = PolarCode::new(n, k, p);
    let mut rng = Lcg64::new(seed);
    let llr0 = ((1.0 - p) / p).ln();
    let llr1 = -llr0;
    let mut errors = 0usize;

    for _ in 0..trials {
        let message = (0..k)
.map(|_| if rng.next_bool() { 1 } else { 0 })
.collect::<Vec<_>>();
        let x = encode(&code, &message);
        let llr = x
.iter()
.map(|&bit| {
                let flipped = rng.next_f64() < p;
                let y = bit ^ u8::from(flipped);
                if y == 0 {
                    llr0
                } else {
                    llr1
                }
            })
.collect::<Vec<_>>();
        let decoded = decode_scl_fast(&code, &llr, list_size);
        if decoded != message {
            errors += 1;
        }
    }

    SimulationResult {
        n,
        k,
        p,
        trials,
        errors,
        seed,
    }
}

pub fn simulate_bsc_scl_fixed_i64<const N: usize, const L: usize, const CHILD_CAP: usize>(
    k: usize,
    p: f64,
    trials: usize,
    seed: u64,
    metric_scale: f64,
) -> SimulationResult {
    let code = PolarCode::new(N, k, p);
    let mut rng = Lcg64::new(seed);
    let llr0 = ((1.0 - p) / p).ln();
    let llr1 = -llr0;
    let mut errors = 0usize;

    for _ in 0..trials {
        let message = (0..k)
.map(|_| if rng.next_bool() { 1 } else { 0 })
.collect::<Vec<_>>();
        let x = encode(&code, &message);
        let llr = x
.iter()
.map(|&bit| {
                let flipped = rng.next_f64() < p;
                let y = bit ^ u8::from(flipped);
                if y == 0 {
                    llr0
                } else {
                    llr1
                }
            })
.collect::<Vec<_>>();
        let decoded = decode_scl_fixed_i64::<N, L, CHILD_CAP>(&code, &llr, metric_scale);
        if decoded != message {
            errors += 1;
        }
    }

    SimulationResult {
        n: N,
        k,
        p,
        trials,
        errors,
        seed,
    }
}

pub fn simulate_bsc_scl_fixed_i64_l8_validation(
    n: usize,
    k: usize,
    p: f64,
    trials: usize,
    seed: u64,
) -> SimulationResult {
    let code = PolarCode::new(n, k, p);
    let mut rng = Lcg64::new(seed);
    let llr0 = ((1.0 - p) / p).ln();
    let llr1 = -llr0;
    let mut errors = 0usize;

    for _ in 0..trials {
        let message = (0..k)
.map(|_| if rng.next_bool() { 1 } else { 0 })
.collect::<Vec<_>>();
        let x = encode(&code, &message);
        let llr = x
.iter()
.map(|&bit| {
                let flipped = rng.next_f64() < p;
                let y = bit ^ u8::from(flipped);
                if y == 0 {
                    llr0
                } else {
                    llr1
                }
            })
.collect::<Vec<_>>();
        let decoded = decode_scl_fixed_i64_l8_validation(&code, &llr);
        if decoded != message {
            errors += 1;
        }
    }

    SimulationResult {
        n,
        k,
        p,
        trials,
        errors,
        seed,
    }
}

pub fn compare_scl_fast_fixed_i64_decoded_bits<
    const N: usize,
    const L: usize,
    const CHILD_CAP: usize,
>(
    k: usize,
    p: f64,
    trials: usize,
    seed: u64,
    metric_scale: f64,
) -> FixedI64DecoderAgreement {
    let code = PolarCode::new(N, k, p);
    let mut rng = Lcg64::new(seed);
    let llr0 = ((1.0 - p) / p).ln();
    let llr1 = -llr0;
    let mut decoded_mismatches = 0usize;
    let mut fast_errors = 0usize;
    let mut fixed_errors = 0usize;

    for _ in 0..trials {
        let message = (0..k)
.map(|_| if rng.next_bool() { 1 } else { 0 })
.collect::<Vec<_>>();
        let x = encode(&code, &message);
        let llr = x
.iter()
.map(|&bit| {
                let flipped = rng.next_f64() < p;
                let y = bit ^ u8::from(flipped);
                if y == 0 {
                    llr0
                } else {
                    llr1
                }
            })
.collect::<Vec<_>>();
        let fast_decoded = decode_scl_fast(&code, &llr, L);
        let fixed_decoded = decode_scl_fixed_i64::<N, L, CHILD_CAP>(&code, &llr, metric_scale);
        if fast_decoded != fixed_decoded {
            decoded_mismatches += 1;
        }
        if fast_decoded != message {
            fast_errors += 1;
        }
        if fixed_decoded != message {
            fixed_errors += 1;
        }
    }

    FixedI64DecoderAgreement {
        n: N,
        k,
        p,
        trials,
        decoded_mismatches,
        fast_errors,
        fixed_errors,
        seed,
    }
}

pub fn simulate_bsc_scl_fast_importance(
    n: usize,
    k: usize,
    target_p: f64,
    proposal_p: f64,
    trials: usize,
    seed: u64,
    list_size: usize,
) -> ImportanceSamplingResult {
    assert!(
        (0.0..0.5).contains(&target_p),
        "target_p must be in (0, 0.5)"
);
    assert!(
        (0.0..0.5).contains(&proposal_p),
        "proposal_p must be in (0, 0.5)"
);
    assert!(trials > 0, "trials must be positive");

    let code = PolarCode::new(n, k, target_p);
    let mut rng = Lcg64::new(seed);
    let llr0 = ((1.0 - target_p) / target_p).ln();
    let llr1 = -llr0;
    let log_flip_weight = (target_p / proposal_p).ln();
    let log_clean_weight = ((1.0 - target_p) / (1.0 - proposal_p)).ln();
    let mut proposal_errors = 0usize;
    let mut weighted_error_sum = 0.0;
    let mut weight_sum = 0.0;
    let mut weight_square_sum = 0.0;

    for _ in 0..trials {
        let message = (0..k)
.map(|_| if rng.next_bool() { 1 } else { 0 })
.collect::<Vec<_>>();
        let x = encode(&code, &message);
        let mut flip_count = 0usize;
        let llr = x
.iter()
.map(|&bit| {
                let flipped = rng.next_f64() < proposal_p;
                flip_count += usize::from(flipped);
                let y = bit ^ u8::from(flipped);
                if y == 0 {
                    llr0
                } else {
                    llr1
                }
            })
.collect::<Vec<_>>();

        let log_weight =
            flip_count as f64 * log_flip_weight + (n - flip_count) as f64 * log_clean_weight;
        let weight = log_weight.exp();
        weight_sum += weight;
        weight_square_sum += weight * weight;

        let decoded = decode_scl_fast(&code, &llr, list_size);
        if decoded != message {
            proposal_errors += 1;
            weighted_error_sum += weight;
        }
    }

    let effective_sample_size = if weight_square_sum == 0.0 {
        0.0
    } else {
        weight_sum * weight_sum / weight_square_sum
    };

    ImportanceSamplingResult {
        n,
        k,
        target_p,
        proposal_p,
        trials,
        proposal_errors,
        weighted_bler_estimate: weighted_error_sum / trials as f64,
        mean_likelihood_ratio: weight_sum / trials as f64,
        effective_sample_size,
        seed,
    }
}

fn polar_transform(bits: &mut [u8]) {
    let n = bits.len();
    let mut half = 1usize;
    while half < n {
        let step = half * 2;
        for block in (0..n).step_by(step) {
            for i in 0..half {
                bits[block + i] ^= bits[block + half + i];
            }
        }
        half = step;
    }
}

fn sc_decode_node(llr: &[f64], offset: usize, frozen_mask: &[bool]) -> Vec<u8> {
    if llr.len() == 1 {
        let bit = if frozen_mask[offset] || llr[0] >= 0.0 {
            0
        } else {
            1
        };
        return vec![bit];
    }

    let half = llr.len() / 2;
    let mut left_llr = vec![0.0; half];
    for i in 0..half {
        left_llr[i] = f_llr(llr[i], llr[half + i]);
    }
    let left = sc_decode_node(&left_llr, offset, frozen_mask);
    let mut left_partial = left.clone();
    polar_transform(&mut left_partial);

    let mut right_llr = vec![0.0; half];
    for i in 0..half {
        right_llr[i] = g_llr(llr[i], llr[half + i], left_partial[i]);
    }
    let right = sc_decode_node(&right_llr, offset + half, frozen_mask);

    [left, right].concat()
}

#[derive(Clone, Debug)]
struct SclPath {
    bits: Vec<u8>,
    metric: f64,
}

fn scl_decode_node(
    llr: &[f64],
    offset: usize,
    frozen_mask: &[bool],
    list_size: usize,
) -> Vec<SclPath> {
    if llr.len() == 1 {
        if frozen_mask[offset] {
            return vec![SclPath {
                bits: vec![0],
                metric: bit_metric(llr[0], 0),
            }];
        }

        let mut paths = vec![
            SclPath {
                bits: vec![0],
                metric: bit_metric(llr[0], 0),
            },
            SclPath {
                bits: vec![1],
                metric: bit_metric(llr[0], 1),
            },
        ];
        prune_paths(&mut paths, list_size);
        return paths;
    }

    let half = llr.len() / 2;
    let mut left_llr = vec![0.0; half];
    for i in 0..half {
        left_llr[i] = f_llr(llr[i], llr[half + i]);
    }

    let left_paths = scl_decode_node(&left_llr, offset, frozen_mask, list_size);
    let mut combined = Vec::new();
    for left in left_paths {
        let mut left_partial = left.bits.clone();
        polar_transform(&mut left_partial);

        let mut right_llr = vec![0.0; half];
        for i in 0..half {
            right_llr[i] = g_llr(llr[i], llr[half + i], left_partial[i]);
        }

        let right_paths = scl_decode_node(&right_llr, offset + half, frozen_mask, list_size);
        for right in right_paths {
            let mut bits = Vec::with_capacity(llr.len());
            bits.extend_from_slice(&left.bits);
            bits.extend_from_slice(&right.bits);
            combined.push(SclPath {
                bits,
                metric: left.metric + right.metric,
            });
        }
    }

    prune_paths(&mut combined, list_size);
    combined
}

fn prune_paths(paths: &mut Vec<SclPath>, list_size: usize) {
    paths.sort_by(|a, b| a.metric.total_cmp(&b.metric));
    paths.truncate(list_size);
}

fn bit_metric(llr: f64, bit: u8) -> f64 {
    let signed = if bit == 0 { llr } else { -llr };
    if signed >= 0.0 {
        (1.0 + (-signed).exp()).ln()
    } else {
        -signed + (1.0 + signed.exp()).ln()
    }
}

fn bit_metric_minsum(llr: f64, bit: u8) -> f64 {
    let agrees_with_hard_decision = (llr >= 0.0 && bit == 0) || (llr < 0.0 && bit == 1);
    if agrees_with_hard_decision {
        0.0
    } else {
        llr.abs()
    }
}

fn sc_bit_llr_minsum(llr: &[f64], offset: usize, phi: usize, decisions: &[u8]) -> f64 {
    if llr.len() == 1 {
        return llr[0];
    }

    let half = llr.len() / 2;
    if phi < offset + half {
        let mut left_llr = vec![0.0; half];
        for i in 0..half {
            left_llr[i] = f_llr_minsum(llr[i], llr[half + i]);
        }
        sc_bit_llr_minsum(&left_llr, offset, phi, decisions)
    } else {
        let mut left_partial = decisions[offset..offset + half].to_vec();
        polar_transform(&mut left_partial);

        let mut right_llr = vec![0.0; half];
        for i in 0..half {
            right_llr[i] = g_llr(llr[i], llr[half + i], left_partial[i]);
        }
        sc_bit_llr_minsum(&right_llr, offset + half, phi, decisions)
    }
}

fn sc_bit_llr_minsum_i64(
    llr: &[i64],
    offset: usize,
    phi: usize,
    decisions: &[u8],
    llr_scratch: &mut [i64],
    partial_scratch: &mut [u8],
) -> i64 {
    if llr.len() == 1 {
        return llr[0];
    }

    let half = llr.len() / 2;
    if phi < offset + half {
        let (left_llr, rest_llr_scratch) = llr_scratch.split_at_mut(half);
        for i in 0..half {
            left_llr[i] = f_llr_minsum_i64(llr[i], llr[half + i]);
        }
        sc_bit_llr_minsum_i64(
            left_llr,
            offset,
            phi,
            decisions,
            rest_llr_scratch,
            partial_scratch,
)
    } else {
        let (right_llr, rest_llr_scratch) = llr_scratch.split_at_mut(half);
        let (left_partial, rest_partial_scratch) = partial_scratch.split_at_mut(half);
        left_partial.copy_from_slice(&decisions[offset..offset + half]);
        polar_transform(left_partial);

        for i in 0..half {
            right_llr[i] = g_llr_i64(llr[i], llr[half + i], left_partial[i]);
        }
        sc_bit_llr_minsum_i64(
            right_llr,
            offset + half,
            phi,
            decisions,
            rest_llr_scratch,
            rest_partial_scratch,
)
    }
}

fn f_llr(a: f64, b: f64) -> f64 {
    let sign = if (a < 0.0) ^ (b < 0.0) { -1.0 } else { 1.0 };
    let min_abs = a.abs().min(b.abs());
    let correction = (1.0 + (-(a + b).abs()).exp()).ln() - (1.0 + (-(a - b).abs()).exp()).ln();
    sign * min_abs + correction
}

fn f_llr_minsum(a: f64, b: f64) -> f64 {
    let sign = if (a < 0.0) ^ (b < 0.0) { -1.0 } else { 1.0 };
    sign * a.abs().min(b.abs())
}

fn f_llr_minsum_i64(a: i64, b: i64) -> i64 {
    let min_abs = i64_abs_saturating(a).min(i64_abs_saturating(b));
    let sign = i64_negative_flag(a) ^ i64_negative_flag(b);
    let sign_mask = 0i64.wrapping_sub(i64::from(sign));

    select_i64(sign_mask, min_abs, min_abs.saturating_neg())
}

fn g_llr(a: f64, b: f64, u: u8) -> f64 {
    if u == 0 {
        b + a
    } else {
        b - a
    }
}

fn g_llr_i64(a: i64, b: i64, u: u8) -> i64 {
    let add = b.saturating_add(a);
    let sub = b.saturating_sub(a);
    let bit = u & 1;
    let bit_mask = 0i64.wrapping_sub(i64::from(bit));

    select_i64(bit_mask, add, sub)
}

#[derive(Clone, Debug)]
struct Lcg64 {
    state: u64,
}

impl Lcg64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
.state
.wrapping_mul(6364136223846793005)
.wrapping_add(1442695040888963407);
        self.state
    }

    fn next_bool(&mut self) -> bool {
        (self.next_u64() >> 63) != 0
    }

    fn next_f64(&mut self) -> f64 {
        const SCALE: f64 = 1.0 / ((1u64 << 53) as f64);
        ((self.next_u64() >> 11) as f64) * SCALE
    }
}
