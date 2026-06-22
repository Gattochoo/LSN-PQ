// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0
//
// Rust-only sanity coverage for the lem:m2 public-invariant smoke axis.
// This keeps the implementation/test tree from depending on the Python
// research harness while preserving the core algebraic guard:
//
//   B = section_A(C) + K,  K A = 0,  so B A = C.
//
// This is an audit/screen artifact only. It is not a reduction and not a
// security claim.

use polar_validation::{
    marginal_invariant_smoke_claim_posture_json, ofa_evidence_audit_posture_json_fields,
    ofa_evidence_audit_posture_json_lines, ofa_evidence_entry_audit_posture_json_fields,
    ofa_evidence_entry_audit_posture_json_lines, ofa_evidence_entry_json_fields,
    ofa_evidence_entry_paper_claim_dependency_json_field,
    ofa_evidence_entry_runtime_dependency_json_field, ofa_evidence_entry_scope_json_field,
    ofa_evidence_entry_use_in_impl_json_field, ofa_evidence_identity_json_fields,
    ofa_evidence_paper_claim_dependency_json_field, ofa_evidence_runtime_dependency_json_field,
    ofa_evidence_scope_json_field, MARGINAL_INVARIANT_OFA_EVIDENCE,
    MARGINAL_INVARIANT_OFA_EVIDENCE_IDS, OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY,
    OFA_EVIDENCE_RUNTIME_DEPENDENCY, OFA_EVIDENCE_SCOPE, OFA_EVIDENCE_USE_IN_IMPL,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum KernelMode {
    Zero,
    Shared,
    BlockShared,
    IidLowdim,
    IidFull,
}

fn dot_f2(a: usize, b: usize) -> usize {
    ((a & b).count_ones() as usize) & 1
}

fn independent_basis(rows: &[usize], n_cols: usize) -> Vec<usize> {
    let mut basis = vec![0usize; n_cols];
    let mut seen = vec![false; n_cols];
    let mask = (1usize << n_cols) - 1;

    for row in rows {
        let mut x = row & mask;
        while x != 0 {
            let pivot = usize::BITS as usize - 1 - x.leading_zeros() as usize;
            if !seen[pivot] {
                basis[pivot] = x;
                seen[pivot] = true;
                break;
            }
            x ^= basis[pivot];
        }
    }

    (0..n_cols)
.rev()
.filter(|&pivot| seen[pivot])
.map(|pivot| basis[pivot])
.collect()
}

fn parity_check_rows(cols: &[usize], m: usize) -> Vec<usize> {
    let candidates = (0..(1usize << m))
.filter(|&h| cols.iter().all(|&col| dot_f2(h, col) == 0))
.collect::<Vec<_>>();
    independent_basis(&candidates, m)
}

fn rows_to_columns(rows: &[usize], n_cols: usize) -> Vec<usize> {
    let mut cols = vec![0usize; n_cols];
    for (i, row) in rows.iter().enumerate() {
        for (j, col) in cols.iter_mut().enumerate() {
            if ((row >> j) & 1) == 1 {
                *col |= 1usize << i;
            }
        }
    }
    cols
}

fn apply_row_combination(rows: &[usize], coeffs: usize) -> usize {
    rows.iter().enumerate().fold(0usize, |acc, (i, row)| {
        acc ^ ((((coeffs >> i) & 1) != 0) as usize * row)
    })
}

fn apply_rows(rows: &[usize], vector: usize) -> usize {
    rows.iter()
.enumerate()
.fold(0usize, |acc, (i, row)| acc | (dot_f2(*row, vector) << i))
}

fn dual_rows_for_columns(cols: &[usize], m: usize) -> Vec<usize> {
    let mut duals = Vec::with_capacity(cols.len());
    for j in 0..cols.len() {
        let dual = (0..(1usize << m))
.find(|&candidate| {
                cols.iter()
.enumerate()
.all(|(k, col)| dot_f2(candidate, *col) == usize::from(j == k))
            })
.expect("test columns must be independent");
        duals.push(dual);
    }
    duals
}

fn kernel_rows_for_mode(kernel_basis: &[usize], row_count: usize, mode: KernelMode) -> Vec<usize> {
    match mode {
        KernelMode::Zero => vec![0usize; row_count],
        KernelMode::Shared => vec![kernel_basis[0]; row_count],
        KernelMode::BlockShared => {
            let split = row_count / 2;
            (0..row_count)
.map(|i| {
                    if i < split {
                        kernel_basis[0]
                    } else {
                        kernel_basis[1 % kernel_basis.len()]
                    }
                })
.collect()
        }
        KernelMode::IidLowdim => {
            let low_dim = usize::max(1, kernel_basis.len() / 2);
            (0..row_count)
.map(|i| {
                    let coeffs = ((i + 1) * 3) % (1usize << low_dim);
                    apply_row_combination(&kernel_basis[..low_dim], coeffs)
                })
.collect()
        }
        KernelMode::IidFull => (0..row_count)
.map(|i| {
                let coeffs = ((i + 1) * 5 + 1) % (1usize << kernel_basis.len());
                apply_row_combination(kernel_basis, coeffs)
            })
.collect(),
    }
}

fn min_coset_weight(cols: &[usize], y: usize) -> usize {
    let mut span = vec![0usize];
    for col in cols {
        let current = span.clone();
        span.extend(current.into_iter().map(|s| s ^ col));
    }
    span.into_iter()
.map(|s| (y ^ s).count_ones() as usize)
.min()
.expect("span is non-empty")
}

#[test]
fn marginal_invariant_smoke_records_claim_posture_in_rust() {
    let json = marginal_invariant_smoke_claim_posture_json();

    assert_eq!(
        OFA_EVIDENCE_SCOPE,
        "historical OFA Rust research, audit metadata only"
);
    assert!(!OFA_EVIDENCE_RUNTIME_DEPENDENCY);
    assert!(!OFA_EVIDENCE_PAPER_CLAIM_DEPENDENCY);
    assert_eq!(
        MARGINAL_INVARIANT_OFA_EVIDENCE_IDS,
        ["OFA-359", "OFA-360", "OFA-398", "OFA-399"]
);
    assert_eq!(
        json.matches("\"ofa\": \"OFA-").count(),
        MARGINAL_INVARIANT_OFA_EVIDENCE_IDS.len()
);
    for ofa in MARGINAL_INVARIANT_OFA_EVIDENCE_IDS {
        assert!(json.contains(&format!("\"ofa\": \"{ofa}\"")));
    }
    assert_eq!(
        MARGINAL_INVARIANT_OFA_EVIDENCE.len(),
        MARGINAL_INVARIANT_OFA_EVIDENCE_IDS.len()
);
    for entry in MARGINAL_INVARIANT_OFA_EVIDENCE {
        assert!(MARGINAL_INVARIANT_OFA_EVIDENCE_IDS.contains(&entry.ofa));
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
        assert_eq!(json.matches(field).count(), 4);
    }
    assert_eq!(
        json.matches(ofa_evidence_entry_audit_posture_json_lines())
.count(),
        4
);

    assert!(json.contains("\"experiment\": \"-lem-m2-marginal-invariant-smoke\""));
    assert!(json.contains("\"artifact_scope\": \"public audit artifact only\""));
    assert!(json.contains("\"source_track\": \"impl/polar_validation Rust\""));
    assert!(json.contains("\"python_research_harness_dependency\": false"));
    assert!(json.contains("\"status\": \"OPEN\""));
    assert!(json.contains("\"production_constant_time_claim\": false"));
    assert!(json.contains("\"reduction_claim\": false"));
    assert!(json.contains("\"security_claim\": false"));
    assert!(json.contains("\"reduction\": \"NOT A REDUCTION\""));
    assert!(json.contains("\"security\": \"NOT A SECURITY CLAIM\""));
    assert!(json.contains("\"checked_invariant\": \"B A equals sampled C\""));
    assert!(json.contains(
        "\"out_of_scope\": \"statistical indistinguishability, reduction, and security claims\""
));
    assert!(json.contains(&ofa_evidence_scope_json_field()));
    assert!(json.contains(&ofa_evidence_runtime_dependency_json_field()));
    assert!(json.contains(&ofa_evidence_paper_claim_dependency_json_field()));
    assert!(json.contains("\"related_ofa_evidence\""));
    assert!(json.contains("\"ofa\": \"OFA-359\""));
    assert!(json.contains("\"finding\": \"low_degree_cube_sparsity_wall\""));
    assert!(json.contains("\"ofa\": \"OFA-360\""));
    assert!(json.contains("\"finding\": \"full_noise_law_not_preserved_by_nonlocal_sp\""));
    assert!(json.contains("\"ofa\": \"OFA-398\""));
    assert!(json.contains("\"finding\": \"omega_stress_margin_live_signal_wall_bound\""));
    assert!(json.contains("\"ofa\": \"OFA-399\""));
    assert!(json.contains("\"finding\": \"constant_rate_wall_n7\""));
    assert!(json.contains(&ofa_evidence_entry_runtime_dependency_json_field()));
    assert!(json.contains(&ofa_evidence_entry_use_in_impl_json_field()));
    assert!(json.contains(&ofa_evidence_entry_scope_json_field()));
    assert!(json.contains(&ofa_evidence_entry_paper_claim_dependency_json_field()));
    assert_eq!(
        json.matches(&ofa_evidence_entry_runtime_dependency_json_field())
.count(),
        4
);
    assert_eq!(
        json.matches(&ofa_evidence_entry_scope_json_field()).count(),
        4
);
    assert_eq!(
        json.matches(&ofa_evidence_entry_paper_claim_dependency_json_field())
.count(),
        4
);
    assert!(json.contains("\"use_in_impl\": \"audit ledger only; no runtime dependency\""));
    assert!(json.contains("Rust-side smoke guard"));
    assert!(json.contains("not production constant-time evidence"));
}

#[test]
fn kernel_perturbed_lift_preserves_public_product_in_rust() {
    let modes = [
        KernelMode::Zero,
        KernelMode::Shared,
        KernelMode::BlockShared,
        KernelMode::IidLowdim,
        KernelMode::IidFull,
    ];

    for &(n, m) in &[(2usize, 4usize), (3, 6), (4, 8)] {
        let ambient = 2 * n;
        let basis = (0..n).map(|j| 1usize << j).collect::<Vec<_>>();
        let dual_rows = dual_rows_for_columns(&basis, ambient);
        let kernel_basis = parity_check_rows(&basis, ambient);

        assert_eq!(basis.len(), n);
        assert_eq!(dual_rows.len(), n);
        assert_eq!(kernel_basis.len(), n);

        for mode in modes {
            let target_c_rows = (0..m)
.map(|i| ((i + 1) * (n + 1) + i * i) & ((1usize << n) - 1))
.collect::<Vec<_>>();
            let section_rows = target_c_rows
.iter()
.map(|&row| apply_row_combination(&dual_rows, row))
.collect::<Vec<_>>();
            let kernel_rows = kernel_rows_for_mode(&kernel_basis, m, mode);
            let b_rows = section_rows
.iter()
.zip(kernel_rows.iter())
.map(|(section, kernel)| section ^ kernel)
.collect::<Vec<_>>();

            let cols = basis
.iter()
.map(|&basis_col| apply_rows(&b_rows, basis_col))
.collect::<Vec<_>>();
            let target_cols = rows_to_columns(&target_c_rows, n);

            assert_eq!(b_rows.len(), m);
            assert_eq!(kernel_rows.len(), m);
            assert_eq!(cols, target_cols);
            for kernel_row in &kernel_rows {
                assert!(basis
.iter()
.all(|&basis_col| dot_f2(*kernel_row, basis_col) == 0));
            }

            let x = (1usize << n) - 1;
            let ax = apply_row_combination(&basis, x);
            let public_error = (0..ambient)
.filter(|i| i % 2 == 0)
.fold(0usize, |acc, i| acc | (1usize << i));
            let y = apply_rows(&b_rows, ax ^ public_error);
            let bucket = min_coset_weight(&cols, y);

            assert!(bucket <= m);
            if mode == KernelMode::Zero {
                assert_eq!(bucket, 0);
            }
        }
    }
}

#[test]
fn marginal_invariant_ofa_evidence_count_is_fixed_shape() {
    assert_eq!(
        polar_validation::MARGINAL_INVARIANT_OFA_EVIDENCE_ENTRY_COUNT,
        4
);
    assert_eq!(
        polar_validation::marginal_invariant_ofa_evidence_entry_count(),
        polar_validation::MARGINAL_INVARIANT_OFA_EVIDENCE_ENTRY_COUNT
);
    assert_eq!(
        polar_validation::marginal_invariant_ofa_evidence_entry_count(),
        polar_validation::MARGINAL_INVARIANT_OFA_EVIDENCE.len()
);
    assert_eq!(
        polar_validation::marginal_invariant_ofa_evidence_entry_count(),
        polar_validation::MARGINAL_INVARIANT_OFA_EVIDENCE_IDS.len()
);
}

#[test]
fn marginal_invariant_json_records_ofa_evidence_entry_count() {
    let json = polar_validation::marginal_invariant_smoke_claim_posture_json();
    assert!(
        json.contains(polar_validation::marginal_invariant_ofa_evidence_entry_count_json_field())
);
    assert_eq!(
        json.matches("\"related_ofa_evidence_entry_count\"").count(),
        1
);
}

#[test]
fn marginal_invariant_json_records_ofa_evidence_ids() {
    let json = polar_validation::marginal_invariant_smoke_claim_posture_json();
    let field = polar_validation::marginal_invariant_ofa_evidence_ids_json_field();
    assert!(json.contains(field));
    assert_eq!(json.matches("\"related_ofa_evidence_ids\"").count(), 1);
    for ofa in polar_validation::MARGINAL_INVARIANT_OFA_EVIDENCE_IDS {
        assert!(field.contains(ofa));
    }
}

#[test]
fn marginal_invariant_entry_posture_line_block_includes_use_in_impl() {
    let block = polar_validation::ofa_evidence_entry_audit_posture_json_lines();
    assert!(block.contains(polar_validation::ofa_evidence_entry_use_in_impl_json_field()));
    let json = polar_validation::marginal_invariant_smoke_claim_posture_json();
    assert_eq!(
        json.matches(block).count(),
        polar_validation::marginal_invariant_ofa_evidence_entry_count()
);
}

#[test]
fn marginal_invariant_ofa_summary_line_block_records_fixed_fields() {
    let block = polar_validation::marginal_invariant_ofa_evidence_summary_json_lines();
    assert!(block.contains(polar_validation::ofa_evidence_scope_json_field()));
    assert!(block.contains(polar_validation::ofa_evidence_runtime_dependency_json_field()));
    assert!(block.contains(polar_validation::ofa_evidence_paper_claim_dependency_json_field()));
    assert!(
        block.contains(polar_validation::marginal_invariant_ofa_evidence_entry_count_json_field())
);
    assert!(block.contains(polar_validation::marginal_invariant_ofa_evidence_ids_json_field()));
    let json = polar_validation::marginal_invariant_smoke_claim_posture_json();
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn marginal_invariant_ofa_summary_field_list_matches_line_block() {
    let block = polar_validation::marginal_invariant_ofa_evidence_summary_json_lines();
    let fields = polar_validation::marginal_invariant_ofa_evidence_summary_json_fields();
    assert_eq!(fields.len(), 5);
    let json = polar_validation::marginal_invariant_smoke_claim_posture_json();
    for field in fields {
        assert!(block.contains(field));
        assert!(json.contains(field));
    }
}

#[test]
fn marginal_invariant_claim_posture_field_list_matches_json() {
    let fields = polar_validation::marginal_invariant_claim_posture_json_fields();
    assert_eq!(fields.len(), 6);
    let json = polar_validation::marginal_invariant_smoke_claim_posture_json();
    for field in fields {
        assert!(json.contains(field));
    }
}

#[test]
fn marginal_invariant_claim_posture_line_block_matches_field_list() {
    let block = polar_validation::marginal_invariant_claim_posture_json_lines();
    let fields = polar_validation::marginal_invariant_claim_posture_json_fields();
    let json = polar_validation::marginal_invariant_smoke_claim_posture_json();
    assert_eq!(fields.len(), 6);
    assert_eq!(json.matches(block).count(), 1);
    for field in fields {
        assert!(block.contains(field));
        assert!(json.contains(field));
    }
}

#[test]
fn marginal_invariant_public_artifact_fields_record_rust_only_boundary() {
    let fields = polar_validation::marginal_invariant_public_artifact_json_fields();
    let json = polar_validation::marginal_invariant_smoke_claim_posture_json();
    assert_eq!(fields.len(), 8);
    assert!(fields.contains(&"\"source_track\": \"impl/polar_validation Rust\""));
    assert!(fields.contains(&"\"python_research_harness_dependency\": false"));
    assert!(fields.contains(&"\"artifact_scope\": \"public audit artifact only\""));
    for field in fields {
        assert!(json.contains(field));
    }
}

#[test]
fn marginal_invariant_public_artifact_line_block_matches_field_list() {
    let block = polar_validation::marginal_invariant_public_artifact_json_lines();
    let fields = polar_validation::marginal_invariant_public_artifact_json_fields();
    let json = polar_validation::marginal_invariant_smoke_claim_posture_json();
    assert_eq!(fields.len(), 8);
    assert_eq!(json.matches(block).count(), 1);
    for field in fields {
        assert!(block.contains(field));
        assert!(json.contains(field));
    }
}

#[test]
fn marginal_invariant_audit_boundary_line_block_matches_components() {
    let block = polar_validation::marginal_invariant_audit_boundary_json_lines();
    let json = polar_validation::marginal_invariant_smoke_claim_posture_json();
    assert!(block.contains(polar_validation::marginal_invariant_public_artifact_json_lines()));
    assert!(block.contains(polar_validation::marginal_invariant_claim_posture_json_lines()));
    assert!(block.contains(polar_validation::marginal_invariant_ofa_evidence_summary_json_lines()));
    assert_eq!(json.matches(block).count(), 1);
}

#[test]
fn marginal_invariant_adjudication_field_records_non_claim_boundary() {
    let field = polar_validation::marginal_invariant_adjudication_json_field();
    let json = polar_validation::marginal_invariant_smoke_claim_posture_json();
    assert!(field.contains("\"adjudication\":"));
    assert!(field.contains("no reduction claim"));
    assert!(field.contains("no security claim"));
    assert!(field.contains("OPEN = LSN"));
    assert_eq!(json.matches(field).count(), 1);
}
