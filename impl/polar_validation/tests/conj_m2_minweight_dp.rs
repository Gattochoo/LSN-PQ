// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0

use polar_validation::conj_m2_minweight_dp::{
    conj_m2_affine_coset_permutation_count, conj_m2_agy_pushforward_fullrun_report,
    conj_m2_agy_pushforward_fullrun_report_json, conj_m2_agy_pushforward_report,
    conj_m2_agy_pushforward_report_json, conj_m2_heavy_left_tail_profile_c4_report,
    conj_m2_heavy_left_tail_profile_c4_report_json, conj_m2_heavy_left_tail_report_json,
    conj_m2_lemm2_usable_lpn_sd_report, conj_m2_lemm2_usable_lpn_sd_report_json,
    conj_m2_lpn_preserving_affine_coset_symmetry_count, conj_m2_n2_minweight_gate_report,
    conj_m2_n2_overdispersion_report, conj_m2_n2_overdispersion_report_json,
    conj_m2_n3_minweight_probe_report, conj_m2_n3_minweight_probe_report_json,
    conj_m2_n3_overdispersion_report, conj_m2_n3_overdispersion_report_json,
    conj_m2_n4_overdispersion_report_json, conj_m2_samec_sd_scaling_comparisons,
    conj_m2_samec_sd_scaling_report, conj_m2_samec_sd_scaling_report_json,
    conj_m2_samec_sd_scaling_single_c_report_json, conj_m2_var_retention_report,
    conj_m2_var_retention_report_json,
};

#[test]
fn conj_m2_minweight_dp_reproduces_n2_solvable_gate() {
    let report = conj_m2_n2_minweight_gate_report();

    assert_eq!(report.len(), 3);
    assert_eq!(report[0].m, 11);
    assert_eq!(report[1].m, 12);
    assert_eq!(report[2].m, 13);

    assert!(
        (report[0].best_advantage - 0.108).abs() < 0.001,
        "{:?}",
        report[0]
);
    assert!(
        (report[1].best_advantage - 0.122).abs() < 0.001,
        "{:?}",
        report[1]
);
    assert!(
        (report[2].best_advantage - 0.138).abs() < 0.001,
        "{:?}",
        report[2]
);

    assert!(report[0].best_advantage < report[1].best_advantage);
    assert!(report[1].best_advantage < report[2].best_advantage);
    for row in report {
        assert_eq!(row.n, 2);
        assert!(row.reachable_state_count > 0);
        assert_eq!(row.status, "OPEN");
        assert!(!row.reduction_claim);
        assert!(!row.security_claim);
    }
}

#[test]
fn conj_m2_n2_overdispersion_report_reproduces_gate_and_records_shape() {
    let report = conj_m2_n2_overdispersion_report();

    assert_eq!(report.len(), 3);
    assert!((report[0].sd1d - 0.108).abs() < 0.001);
    assert!((report[1].sd1d - 0.122).abs() < 0.001);
    assert!((report[2].sd1d - 0.138).abs() < 0.001);
    for row in report {
        assert_eq!(row.n, 2);
        assert_eq!(row.claim_level, "EVIDENCE");
        assert_eq!(row.status, "OPEN");
        assert!(!row.reduction_claim);
        assert!(!row.security_claim);
        assert_eq!(row.distribution.len(), row.m + 1);
        assert!(row.structured_metrics.mean >= 0.0);
        assert!(row.lpn_metrics.mean >= 0.0);
        assert!(row.structured_metrics.variance >= 0.0);
        assert!(row.lpn_metrics.variance >= 0.0);
        assert!(row.structured_metrics.tail_ge_2_mass >= 0.0);
        assert!(row.lpn_metrics.tail_ge_2_mass >= 0.0);
    }
}

#[test]
fn conj_m2_minweight_dp_runs_n3_sparse_probe_smoke() {
    let report = conj_m2_n3_minweight_probe_report(4);

    assert_eq!(report.len(), 1);
    assert_eq!(report[0].n, 3);
    assert_eq!(report[0].m, 4);
    assert!(report[0].best_advantage >= 0.0);
    assert!(report[0].best_advantage <= 1.0);
    assert!(report[0].reachable_state_count > 0);
    assert_eq!(report[0].status, "OPEN");
    assert!(!report[0].reduction_claim);
    assert!(!report[0].security_claim);
}

#[test]
fn conj_m2_n3_overdispersion_report_records_m4_shape() {
    let report = conj_m2_n3_overdispersion_report(4);

    assert_eq!(report.len(), 1);
    assert_eq!(report[0].n, 3);
    assert_eq!(report[0].m, 4);
    assert!((report[0].sd1d - 0.011537575911).abs() < 0.000_000_001);
    assert_eq!(report[0].claim_level, "EVIDENCE");
    assert_eq!(report[0].status, "OPEN");
    assert!(!report[0].reduction_claim);
    assert!(!report[0].security_claim);
    assert_eq!(report[0].distribution.len(), report[0].m + 1);
    assert!(report[0].structured_metrics.variance >= 0.0);
    assert!(report[0].lpn_metrics.variance >= 0.0);
}

#[test]
fn conj_m2_affine_coset_group_has_expected_n3_size_and_preserves_lpn_law() {
    assert_eq!(conj_m2_affine_coset_permutation_count(3), 1344);
    assert_eq!(
        conj_m2_lpn_preserving_affine_coset_symmetry_count(3, 0.25),
        168
);
}

#[test]
fn conj_m2_symmetry_reduced_dp_preserves_pr8_n3_gate_values() {
    let report = conj_m2_n3_minweight_probe_report(6);

    assert_eq!(report.len(), 3);
    assert!((report[0].best_advantage - 0.011537575911).abs() < 0.000_000_001);
    assert!((report[1].best_advantage - 0.013109787778).abs() < 0.000_000_001);
    assert!((report[2].best_advantage - 0.018049799025).abs() < 0.000_000_001);

    assert!(report[0].reachable_state_count <= 2945);
    assert!(report[1].reachable_state_count <= 10128);
    assert!(report[2].reachable_state_count <= 29953);
}

#[test]
fn conj_m2_minweight_dp_n3_json_records_posture_and_boundary() {
    let json = conj_m2_n3_minweight_probe_report_json(4);

    assert!(json.contains("\"experiment\": \"-conj-m2-n3-minweight-symmetry-dp\""));
    assert!(json.contains("\"gate\": \"n=2 reproduced before n=3 trust\""));
    assert!(json.contains("\"target\": \"conj:m2 n=3 min-syndrome-weight symmetry-reduced DP\""));
    assert!(json.contains("\"method\": \"law-preserving affine-coset profile canonicalization\""));
    assert!(json.contains("\"max_m_requested\": 4"));
    assert!(json.contains("\"status\": \"OPEN\""));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
    assert!(json.contains("\"rows\""));
    assert!(json.contains("\"reachable_state_count\""));
}

#[test]
fn conj_m2_n2_overdispersion_json_records_tail_metrics() {
    let json = conj_m2_n2_overdispersion_report_json();

    assert!(json.contains("\"experiment\": \"-conj-m2-n2-overdispersion-char\""));
    assert!(json.contains("\"claim_level\": \"EVIDENCE\""));
    assert!(json.contains("\"status\": \"OPEN\""));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
    assert!(json.contains("\"structured_metrics\""));
    assert!(json.contains("\"lpn_metrics\""));
    assert!(json.contains("\"upper_tail_mass\""));
    assert!(json.contains("\"tail_ge_2_mass\""));
    assert!(json.contains("\"saturation_mass\""));
    assert!(json.contains("\"sd_contribution\""));
}

#[test]
fn conj_m2_n3_overdispersion_json_records_tail_metrics() {
    let json = conj_m2_n3_overdispersion_report_json(4);

    assert!(json.contains("\"experiment\": \"-conj-m2-n3-overdispersion-char\""));
    assert!(json.contains("\"claim_level\": \"EVIDENCE\""));
    assert!(json.contains("\"status\": \"OPEN\""));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
    assert!(json.contains("\"structured_metrics\""));
    assert!(json.contains("\"lpn_metrics\""));
    assert!(json.contains("\"upper_tail_mass\""));
    assert!(json.contains("\"tail_ge_2_mass\""));
    assert!(json.contains("\"saturation_mass\""));
    assert!(json.contains("\"sd_contribution\""));
}

#[test]
fn conj_m2_n4_overdispersion_json_records_gate_and_feasible_scope() {
    let json = conj_m2_n4_overdispersion_report_json(0);

    assert!(json.contains("\"experiment\": \"-conj-m2-n4-overdispersion-char\""));
    assert!(json.contains("\"claim_level\": \"EVIDENCE\""));
    assert!(json.contains("\"gate\": \"n=3 over-dispersion metrics reproduced before n=4 trust\""));
    assert!(json.contains("\"max_m_requested\": 0"));
    assert!(json.contains("no threshold is inferred beyond generated rows"));
    assert!(json.contains("\"n3_gate_reference_rows\""));
    assert!(json.contains("\"structured_variance\": 0.4557"));
    assert!(json.contains("\"n4_rows\""));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
}

#[test]
fn conj_m2_samec_sd_scaling_empty_report_has_no_inferred_comparison() {
    let report = conj_m2_samec_sd_scaling_report(0);
    let comparisons = conj_m2_samec_sd_scaling_comparisons(&report);

    assert!(report.is_empty());
    assert!(comparisons.is_empty());
}

#[test]
fn conj_m2_samec_sd_scaling_json_records_grid_and_posture() {
    let json = conj_m2_samec_sd_scaling_report_json(0);

    assert!(json.contains("\"experiment\": \"-conj-m2-samec-sd-scaling\""));
    assert!(json.contains("\"claim_level\": \"EVIDENCE\""));
    assert!(json.contains("\"planned_grid\""));
    assert!(json.contains("\"c\": 2"));
    assert!(json.contains("\"n2_m\": 8"));
    assert!(json.contains("\"n3_m\": 12"));
    assert!(json.contains("missing c-cells carry no inferred trend"));
    assert!(json.contains("\"completed_cells\""));
    assert!(json.contains("\"comparisons\""));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
}

#[test]
fn conj_m2_samec_sd_scaling_single_c_json_keeps_noninferential_contract() {
    let json = conj_m2_samec_sd_scaling_single_c_report_json(0);

    assert!(json.contains("\"experiment\": \"-conj-m2-samec-sd-scaling\""));
    assert!(json.contains("\"max_c_requested\": 0"));
    assert!(json.contains("\"planned_grid\""));
    assert!(json.contains("\"completed_cells\""));
    assert!(json.contains("\"comparisons\""));
    assert!(json.contains("missing c-cells carry no inferred trend"));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
}

#[test]
fn conj_m2_heavy_left_tail_json_records_corrected_regime_and_posture() {
    let json = conj_m2_heavy_left_tail_report_json(3);

    assert!(json.contains("\"experiment\": \"-conj-m2-heavy-left-tail-asymmetry\""));
    assert!(json.contains("\"claim_level\": \"EVIDENCE\""));
    assert!(json.contains("\"heavy weight-2 B rows with q=3/8"));
    assert!(json.contains("\"r_cov_definition\""));
    assert!(json.contains("\"max_c_requested\": 3"));
    assert!(json.contains("\"rows\""));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
}

#[test]
fn conj_m2_heavy_left_tail_profile_method_records_n2_gate_drift() {
    let rows = conj_m2_heavy_left_tail_profile_c4_report(2);

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].n, 2);
    assert_eq!(rows[0].m, 8);
    assert!(rows[0].realized_q > rows[0].delta_gv);
    assert_eq!(rows[0].structured.mode_t, 0);
    assert!((rows[0].mean_lpn_minus_structured - 0.015728731198).abs() < 0.000_000_001);
    assert!((rows[0].variance_gap_structured_minus_lpn - 2.383709269950).abs() < 0.000_000_001);
    assert_eq!(rows[0].status, "OPEN");
    assert_eq!(rows[0].claim_level, "EVIDENCE");
    assert!(!rows[0].reduction_claim);
    assert!(!rows[0].security_claim);
}

#[test]
fn conj_m2_heavy_left_tail_profile_json_flags_method_drift_before_n4() {
    let json = conj_m2_heavy_left_tail_profile_c4_report_json(0);

    assert!(json.contains("\"experiment\": \"-conj-m2-heavy-n4-thirdpoint-profile\""));
    assert!(json.contains("\"claim_level\": \"EVIDENCE\""));
    assert!(json.contains("\"profile_method\""));
    assert!(json.contains("profile mismatch must be treated as evidence-method drift"));
    assert!(json.contains("profile rows do not exactly reproduce PR #19 row-law"));
    assert!(json.contains("\"max_n_requested\": 0"));
    assert!(json.contains("\"rows\""));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
}

#[test]
fn conj_m2_var_retention_fixed_b_reproduces_track_a_n3_anchor() {
    let rows = conj_m2_var_retention_report(3);
    let fixed_n3 = rows
.iter()
.find(|row| row.b_model == "fixed_weight_two_rows_profile" && row.n == 3)
.expect("fixed n=3 row must be present");

    assert!(fixed_n3.exact);
    assert!((fixed_n3.var_t_ratio - 3.278).abs() < 0.01, "{fixed_n3:?}");
    assert!(
        (fixed_n3.raw_weight_ratio - 3.199).abs() < 0.01,
        "{fixed_n3:?}"
);
    assert!(
        (fixed_n3.ratio_attenuation - 1.036).abs() < 0.02,
        "{fixed_n3:?}"
);
    assert_eq!(fixed_n3.status, "OPEN");
    assert_eq!(fixed_n3.claim_level, "EVIDENCE");
    assert!(!fixed_n3.reduction_claim);
    assert!(!fixed_n3.security_claim);
}

#[test]
fn conj_m2_var_retention_json_records_n4_debug_boundary() {
    let json = conj_m2_var_retention_report_json(0);

    assert!(json.contains("\"experiment\": \"-conj-m2-overdispersion-retention\""));
    assert!(json.contains("\"claim_level\": \"EVIDENCE\""));
    assert!(json.contains("\"retention_definition\""));
    assert!(json.contains("\"ratio_attenuation_definition\""));
    assert!(json.contains("fixed weight-2 rows use integer-sorted order"));
    assert!(json.contains("random-B averaged n=4 is not extrapolated"));
    assert!(json.contains("\"rows\""));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
}

#[test]
fn conj_m2_agy_pushforward_lags0_first_m_reproduces_exp941_anchor() {
    let rows = conj_m2_agy_pushforward_report(2, 1);
    let anchor = rows
.iter()
.find(|row| {
            row.b_model == "first_m_integer_sorted"
                && row.n == 2
                && row.lagrangian_scope == "first_lagrangian"
        })
.expect("n=2 Lags[0] first-m integer-sorted row must be present");

    assert_eq!(anchor.m, 8);
    assert_eq!(anchor.rank_b, 3);
    assert_eq!(anchor.col_b_size, 8);
    assert_eq!(anchor.lagrangians_evaluated, 1);
    assert!(anchor.lagrangian_total > anchor.lagrangians_evaluated);
    assert!(!anchor.all_lagrangians_exact);
    assert!(
        (anchor.var_full - 0.7724609375).abs() < 0.000_000_001,
        "{anchor:?}"
);
    assert!(
        (anchor.var_col_b - 2.25).abs() < 0.000_000_001,
        "{anchor:?}"
);
    assert!(
        (anchor.pushforward_ratio - 2.912768647282).abs() < 0.000_000_001,
        "{anchor:?}"
);
    assert!(anchor.bernoulli_ratio > 1.0, "{anchor:?}");
    assert!(anchor.push_over_bernoulli.is_finite(), "{anchor:?}");
    assert_eq!(anchor.status, "OPEN");
    assert_eq!(anchor.claim_level, "EVIDENCE");
    assert!(!anchor.reduction_claim);
    assert!(!anchor.security_claim);
}

#[test]
fn conj_m2_agy_pushforward_json_records_b_dependent_pivot() {
    let json = conj_m2_agy_pushforward_report_json(2, 1);

    assert!(json.contains("\"experiment\": \"-conj-m2--pushforward-ratio\""));
    assert!(json.contains("\"target\": \"same-B Bernoulli ratio versus uniform-pushforward ratio"));
    assert!(json.contains("\"b_models\""));
    assert!(json.contains("\"circulant_gaps_1_2\""));
    assert!(json.contains("\"circulant_gaps_1_3\""));
    assert!(json.contains("\"first_m_integer_sorted\""));
    assert!(json.contains("B-dependent trend caveat"));
    assert!(json.contains("no horn-closing lean is inferred from a single B construction"));
    assert!(json.contains("\"push_over_bernoulli\""));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
}

#[test]
fn conj_m2_agy_pushforward_fullrun_schedule_keeps_n2_n3_exact() {
    let rows = conj_m2_agy_pushforward_fullrun_report(4, 2, 1, 0);
    let n2_circ = rows
.iter()
.find(|row| row.b_model == "circulant_gaps_1_2" && row.n == 2)
.expect("fullrun n=2 circulant row must be present");
    let n3_circ = rows
.iter()
.find(|row| row.b_model == "circulant_gaps_1_2" && row.n == 3)
.expect("fullrun n=3 circulant row must be present");
    let n4_circ = rows
.iter()
.find(|row| row.b_model == "circulant_gaps_1_2" && row.n == 4)
.expect("fullrun n=4 circulant row must be present");

    assert_eq!(n2_circ.lagrangian_scope, "all_lagrangians");
    assert_eq!(n2_circ.lagrangians_evaluated, 15);
    assert_eq!(n2_circ.lagrangian_total, 15);
    assert!(n2_circ.all_lagrangians_exact);
    assert_eq!(n3_circ.lagrangian_scope, "all_lagrangians");
    assert_eq!(n3_circ.lagrangians_evaluated, 135);
    assert_eq!(n3_circ.lagrangian_total, 135);
    assert!(n3_circ.all_lagrangians_exact);
    assert_eq!(n4_circ.lagrangian_scope, "even_spaced_sample");
    assert_eq!(n4_circ.lagrangians_evaluated, 2);
    assert_eq!(n4_circ.lagrangian_total, 2295);
    assert!(!n4_circ.all_lagrangians_exact);
}

#[test]
fn conj_m2_agy_pushforward_fullrun_json_records_n5_sample_and_n6_ceiling() {
    let json = conj_m2_agy_pushforward_fullrun_report_json(6, 1, 1, 0);

    assert!(json.contains("\"experiment\": \"-conj-m2--pushforward-fullrun\""));
    assert!(
        json.contains("\"n2_n3_policy\": \"exact all Lagrangians regardless of sample limits\"")
);
    assert!(json.contains("\"n4_policy\": \"exact if n4_lagrangian_limit is 0 or >= total; otherwise evenly spaced sample\""));
    assert!(json.contains("\"n5_policy\": \"deterministic isotropic-basis sample\""));
    assert!(
        json.contains("\"n6_policy\": \"profile ceiling unless explicitly made feasible later\"")
);
    assert!(json.contains("\"individual_variance_fields\": [\"var_full\", \"var_colB\"]"));
    assert!(json.contains("\"lagrangian_total\": 75735"));
    assert!(json.contains("\"lagrangian_total\": 4922775"));
    assert!(json.contains("\"lagrangian_scope\": \"deterministic_basis_sample\""));
    assert!(json.contains("\"lagrangian_scope\": \"profile_ceiling\""));
    assert!(json.contains("\"feasibility_note\""));
    assert!(json.contains("\"bernoulli_ratio\": null"));
    assert!(json.contains("\"pushforward_ratio\": null"));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
}

#[test]
fn conj_m2_agy_pushforward_fullrun_json_records_canonical_exact_trend_contract() {
    let rows = conj_m2_agy_pushforward_fullrun_report(5, 1, 2, 0);
    let n4_circ = rows
.iter()
.find(|row| row.b_model == "circulant_gaps_1_2" && row.n == 4)
.expect("canonical fullrun n=4 circulant row must be present");
    let n5_circ = rows
.iter()
.find(|row| row.b_model == "circulant_gaps_1_2" && row.n == 5)
.expect("canonical fullrun n=5 circulant row must be present");

    assert_eq!(n4_circ.lagrangian_scope, "even_spaced_sample");
    assert_eq!(n4_circ.lagrangians_evaluated, 1);
    assert!(!n4_circ.all_lagrangians_exact);
    assert_eq!(n5_circ.lagrangian_scope, "deterministic_basis_sample");
    assert_eq!(n5_circ.lagrangians_evaluated, 2);
    assert!(!n5_circ.all_lagrangians_exact);

    let json = conj_m2_agy_pushforward_fullrun_report_json(6, 1, 2, 0);

    assert!(json.contains("\"fixture\": \"947-conj-m2-pushforward-exact-trend\""));
    assert!(json.contains("canonical instruction supersedes earlier appended handoffs"));
    assert!(json.contains("both prior analytic predictions are retracted or falsified"));
    assert!(json.contains("\"recommended_n4_lagrangian_limit\": 0"));
    assert!(json.contains("\"recommended_n5_lagrangian_limit\": 200"));
    assert!(json.contains("\"n5_lagrangian_limit\": 2"));
    assert!(json.contains("\"lagrangian_scope\": \"deterministic_basis_sample\""));
    assert!(json.contains("\"lagrangians_evaluated\": 2"));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
}

#[test]
fn conj_m2_lemm2_usable_lpn_sd_reproduces_confined_track_a_gate() {
    let rows = conj_m2_lemm2_usable_lpn_sd_report(3, 0, 0);
    let n2_circ = rows
.iter()
.find(|row| {
            row.model == "confined_fixed_b" && row.b_model == "circulant_gaps_1_2" && row.n == 2
        })
.expect("n=2 confined circulant SD row must be present");
    let n3_circ = rows
.iter()
.find(|row| {
            row.model == "confined_fixed_b" && row.b_model == "circulant_gaps_1_2" && row.n == 3
        })
.expect("n=3 confined circulant SD row must be present");

    assert!(n2_circ.exact);
    assert_eq!(n2_circ.lagrangians_evaluated, 15);
    assert!((n2_circ.sd_mean - 0.850).abs() < 0.01, "{n2_circ:?}");
    assert!(n2_circ.q_exceeds_delta_gv);
    assert!(n3_circ.exact);
    assert_eq!(n3_circ.lagrangians_evaluated, 135);
    assert!((n3_circ.sd_mean - 0.943).abs() < 0.01, "{n3_circ:?}");
    assert!(n3_circ.sd_mean > n2_circ.sd_mean);
    assert_eq!(n3_circ.status, "OPEN");
    assert_eq!(n3_circ.claim_level, "EVIDENCE");
    assert!(!n3_circ.reduction_claim);
    assert!(!n3_circ.security_claim);
}

#[test]
fn conj_m2_lemm2_usable_lpn_sd_json_records_truth_test_posture() {
    let json = conj_m2_lemm2_usable_lpn_sd_report_json(3, 0, 0);

    assert!(json.contains("\"experiment\": \"-lemm2-usable-lpn-sd-scale\""));
    assert!(json.contains("\"fixture\": \"957-lemm2-diffuse-SD-scale\""));
    assert!(json.contains("usable LPN target delta_GV, not matched-rate LPN"));
    assert!(json.contains("Independent gates: confined n=2 SD approx 0.850 and n=3 SD approx 0.943"));
    assert!(json.contains("\"model\": \"confined_fixed_b\""));
    assert!(json.contains("\"model\": \"uniform_b_sanity_pending\""));
    assert!(json.contains("\"model\": \"random_weight_two_average_pending\""));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
}
