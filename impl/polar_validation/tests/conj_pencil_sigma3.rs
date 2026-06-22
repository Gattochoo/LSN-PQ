// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0

use polar_validation::conj_pencil_sigma3::{
    conj_pencil_sigma3_full_pencil_gate, conj_pencil_sigma3_incidence_report,
    conj_pencil_sigma3_light_report_json, conj_pencil_sigma3_max_search_report,
    conj_pencil_sigma3_max_search_report_json, conj_pencil_sigma3_max_search_v2_report,
    conj_pencil_sigma3_max_search_v2_report_json, conj_pencil_sigma3_n3_sanity_report,
    conj_pencil_sigma3_pencil_report, conj_pencil_sigma3_v2_special_config_report,
    lagrangian_count_formula, nearest_conj_pencil_scale,
};

#[test]
fn conj_pencil_sigma3_n3_is_degenerate_sanity_gate() {
    let report = conj_pencil_sigma3_n3_sanity_report(11);

    assert_eq!(report.n, 3);
    assert_eq!(report.subset_size, 11);
    assert_eq!(report.sigma3, 11);
    assert!((report.sigma3_over_m_squared - (1.0 / 11.0)).abs() < 1e-12);
    assert_eq!(report.status, "OPEN");
    assert!(!report.reduction_claim);
    assert!(!report.security_claim);
}

#[test]
fn conj_pencil_sigma3_n4_incidence_matches_hand_count() {
    let report = conj_pencil_sigma3_incidence_report(4);

    assert_eq!(report.lagrangian_count, 2295);
    assert_eq!(report.isotropic_three_space_count, 11475);
    assert_eq!(report.three_spaces_per_lagrangian, 15);
    assert_eq!(report.lagrangians_per_three_space, 3);
}

#[test]
fn conj_pencil_sigma3_n4_scale_pencil_seed_is_exactly_computable() {
    let scale = nearest_conj_pencil_scale(4);
    let report = conj_pencil_sigma3_pencil_report(4, 2, Some(scale));

    assert_eq!(scale, 9);
    assert_eq!(report.subset_size, 9);
    assert!(report.sigma3 > 0);
    assert!(report.sigma3_over_m_squared > 0.0);
    assert_eq!(report.construction, "k=2 pencil seed");
}

#[test]
fn conj_pencil_sigma3_n6_scale_pencil_seed_is_exactly_computable() {
    let scale = nearest_conj_pencil_scale(6);
    let report = conj_pencil_sigma3_pencil_report(6, 2, Some(scale));

    assert_eq!(lagrangian_count_formula(6), 4_922_775);
    assert_eq!(scale, 1202);
    assert_eq!(report.subset_size, 1202);
    assert!(report.sigma3_over_m_squared > 4.146092037984);
    assert_eq!(report.construction, "k=2 pencil seed");
}

#[test]
fn conj_pencil_sigma3_full_k2_pencil_closed_form_gate_matches_track_a_values() {
    let n4 = conj_pencil_sigma3_full_pencil_gate(4);
    let n5 = conj_pencil_sigma3_full_pencil_gate(5);
    let n6 = conj_pencil_sigma3_full_pencil_gate(6);

    assert!((n4.sigma3_over_pencil_size_squared - 7.0 / 5.0).abs() < 0.01);
    assert!((n5.sigma3_over_pencil_size_squared - 421.0 / 135.0).abs() < 0.01);
    assert!((n6.sigma3_over_pencil_size_squared - 83.0 / 17.0).abs() < 0.01);
    assert!(n4.gate_passed);
    assert!(n5.gate_passed);
    assert!(n6.gate_passed);
}

#[test]
fn conj_pencil_sigma3_max_search_report_records_bounded_scope() {
    let report = conj_pencil_sigma3_max_search_report();

    assert_eq!(report.len(), 3);
    for row in &report {
        assert!(row.inspected_candidate_count >= row.subset_size);
        assert!(row.best_ratio >= row.full_pencil_ratio);
        assert_eq!(row.claim_level, "EVIDENCE");
        assert_eq!(row.status, "OPEN");
        assert!(!row.reduction_claim);
        assert!(!row.security_claim);
    }
    assert!(report[0].search_kind.contains("bounded greedy"));
    assert!(report[1].search_kind.contains("bounded greedy"));
    assert!(report[2].search_kind.contains("gate only"));
}

#[test]
fn conj_pencil_sigma3_json_records_audit_posture() {
    let json = conj_pencil_sigma3_light_report_json();

    assert!(json.contains("\"experiment\": \"-conj-pencil-sigma3-search-n6\""));
    assert!(json.contains("\"claim_level\": \"EVIDENCE\""));
    assert!(json.contains("not a conj:pencil break"));
    assert!(json.contains("\"status\": \"OPEN\""));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
    assert!(json.contains("\"n3_sanity\""));
    assert!(json.contains("\"n4_incidence\""));
}

#[test]
fn conj_pencil_sigma3_max_search_json_records_gate_and_scope() {
    let json = conj_pencil_sigma3_max_search_report_json();

    assert!(json.contains("\"experiment\": \"-sigma3-max-search-n456\""));
    assert!(json.contains("\"claim_level\": \"EVIDENCE\""));
    assert!(json.contains("\"full_k2_pencil_gates\""));
    assert!(json.contains("\"search_rows\""));
    assert!(json.contains("\"bounded deterministic lower-bound search"));
    assert!(json.contains("\"gate_passed\": true"));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
}

#[test]
fn conj_pencil_sigma3_v2_report_records_real_search_scope() {
    let report = conj_pencil_sigma3_max_search_v2_report();

    assert_eq!(report.len(), 3);
    for row in &report {
        assert!(row.inspected_candidate_count >= row.subset_size);
        assert!(row.restart_count > 0);
        assert!(row.best_ratio >= row.full_pencil_ratio);
        assert_eq!(row.claim_level, "EVIDENCE");
        assert_eq!(row.status, "OPEN");
        assert!(!row.reduction_claim);
        assert!(!row.security_claim);
    }
    assert!(report[0].search_kind.contains("random-restart"));
    assert!(report[1].search_kind.contains("random-restart"));
    assert!(report[2].search_kind.contains("quotient-lift"));
    assert!(report[2].inspected_candidate_count > report[2].subset_size);
}

#[test]
fn conj_pencil_sigma3_v2_special_configs_are_reported_with_posture() {
    let report = conj_pencil_sigma3_v2_special_config_report();

    assert_eq!(report.len(), 12);
    assert!(report
.iter()
.any(|row| row.construction.contains("near-2-pencil")));
    assert!(report
.iter()
.any(|row| row.construction.contains("three coordinate")));
    assert!(report
.iter()
.any(|row| row.construction.contains("heavy-pair")));
    for row in &report {
        assert!(row.config_ratio > 0.0);
        assert_eq!(row.claim_level, "EVIDENCE");
        assert_eq!(row.status, "OPEN");
        assert!(!row.reduction_claim);
        assert!(!row.security_claim);
    }
}

#[test]
fn conj_pencil_sigma3_v2_json_records_handoff_contract() {
    let json = conj_pencil_sigma3_max_search_v2_report_json();

    assert!(json.contains("\"experiment\": \"-sigma3-max-search-v2-n456\""));
    assert!(json.contains("\"claim_level\": \"EVIDENCE\""));
    assert!(json.contains("\"search_scope\": \"bounded randomized-restart evidence search"));
    assert!(json.contains("\"special_config_rows\""));
    assert!(json.contains("near-2-pencil"));
    assert!(json.contains("heavy-pair"));
    assert!(json.contains("quotient-lift"));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
}
