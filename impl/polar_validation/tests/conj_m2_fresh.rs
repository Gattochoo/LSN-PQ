// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0

use polar_validation::conj_m2_fresh::{
    conj_m2_fresh_evidence_lagrangian_count, conj_m2_fresh_evidence_lagrangian_count_json,
    conj_m2_fresh_evidence_target_ns, conj_m2_lagrangian_span_masks,
    conj_m2_span_is_totally_isotropic, conj_m2_span_mask_contains, conj_m2_span_mask_weight,
    conj_m2_symplectic_pairing,
};

#[test]
fn conj_m2_symplectic_pairing_uses_standard_split_coordinates() {
    assert_eq!(conj_m2_symplectic_pairing(0b01, 0b10, 1), 1);
    assert_eq!(conj_m2_symplectic_pairing(0b10, 0b01, 1), 1);
    assert_eq!(conj_m2_symplectic_pairing(0b01, 0b01, 1), 0);
    assert_eq!(conj_m2_symplectic_pairing(0b001, 0b1000, 3), 1);
    assert_eq!(conj_m2_symplectic_pairing(0b010, 0b1000, 3), 0);
}

#[test]
fn conj_m2_fresh_evidence_targets_n3_and_n4() {
    assert_eq!(conj_m2_fresh_evidence_target_ns(), [3, 4]);
}

#[test]
fn conj_m2_fresh_lagrangian_counts_are_fresh_enumerated_for_n3_n4() {
    assert_eq!(conj_m2_fresh_evidence_lagrangian_count(3), 135);
    assert_eq!(conj_m2_fresh_evidence_lagrangian_count(4), 2295);
}

#[test]
fn conj_m2_fresh_lagrangian_spans_are_fixed_size_and_isotropic() {
    for n in conj_m2_fresh_evidence_target_ns() {
        for span in conj_m2_lagrangian_span_masks(n) {
            assert_eq!(conj_m2_span_mask_weight(&span), 1usize << n);
            assert!(conj_m2_span_mask_contains(&span, 0));
            assert!(conj_m2_span_is_totally_isotropic(&span, n));
        }
    }
}

#[test]
fn conj_m2_fresh_lagrangian_count_json_records_posture_and_counts() {
    let json = conj_m2_fresh_evidence_lagrangian_count_json();

    assert!(json.contains("\"experiment\": \"-conj-m2-fresh-lagrangian-counts\""));
    assert!(json.contains("\"target\": \"conj:m2 n=3,4 fresh evidence\""));
    assert!(json.contains("\"n3_lagrangian_count\": 135"));
    assert!(json.contains("\"n4_lagrangian_count\": 2295"));
    assert!(json.contains(
        "\"archive_use\": \"method reference only; no archived numeric outputs reused\""
));
    assert!(json.contains("\"status\": \"OPEN\""));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"production_constant_time_claim\": false"));
}
