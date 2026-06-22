// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

use crate::conj_m2_fresh::{
    conj_m2_lagrangian_span_masks, conj_m2_span_mask_contains, conj_m2_span_mask_vectors,
    conj_m2_symplectic_pairing,
};

#[derive(Clone, Debug)]
pub struct ConjM2MinWeightGateRow {
    pub n: usize,
    pub m: usize,
    pub best_advantage: f64,
    pub best_p_b: f64,
    pub best_p_prime: f64,
    pub reachable_state_count: usize,
    pub status: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

#[derive(Clone, Debug)]
pub struct ConjM2TDistributionMetrics {
    pub mean: f64,
    pub variance: f64,
    pub upper_tail_threshold: usize,
    pub upper_tail_mass: f64,
    pub tail_ge_2_mass: f64,
    pub saturation_t: usize,
    pub saturation_mass: f64,
}

#[derive(Clone, Debug)]
pub struct ConjM2TDistributionPoint {
    pub t: usize,
    pub structured_probability: f64,
    pub lpn_probability: f64,
    pub signed_delta: f64,
    pub sd_contribution: f64,
}

#[derive(Clone, Debug)]
pub struct ConjM2OverdispersionRow {
    pub n: usize,
    pub m: usize,
    pub p_b: f64,
    pub p_prime: f64,
    pub sd1d: f64,
    pub structured_metrics: ConjM2TDistributionMetrics,
    pub lpn_metrics: ConjM2TDistributionMetrics,
    pub distribution: Vec<ConjM2TDistributionPoint>,
    pub status: &'static str,
    pub claim_level: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

#[derive(Clone, Debug)]
pub struct ConjM2SameCScalingComparison {
    pub c: usize,
    pub n2_sd1d: f64,
    pub n3_sd1d: f64,
    pub sd_direction_n2_to_n3: &'static str,
    pub n2_variance_gap: f64,
    pub n3_variance_gap: f64,
    pub variance_gap_direction_n2_to_n3: &'static str,
    pub status: &'static str,
    pub claim_level: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

#[derive(Clone, Debug)]
pub struct ConjM2LeftTailSide {
    pub mean: f64,
    pub variance: f64,
    pub mode_t: usize,
    pub max_support_t: usize,
    pub left_tail_mass_below_r_cov: f64,
    pub atom_at_r_cov: f64,
}

#[derive(Clone, Debug)]
pub struct ConjM2HeavyLeftTailRow {
    pub c: usize,
    pub n: usize,
    pub m: usize,
    pub delta_gv: f64,
    pub realized_q: f64,
    pub r_cov_t: usize,
    pub sd1d: f64,
    pub mean_lpn_minus_structured: f64,
    pub variance_gap_structured_minus_lpn: f64,
    pub structured: ConjM2LeftTailSide,
    pub lpn: ConjM2LeftTailSide,
    pub structured_reachable_state_count: usize,
    pub lpn_reachable_state_count: usize,
    pub row_model: &'static str,
    pub status: &'static str,
    pub claim_level: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

#[derive(Clone, Debug)]
pub struct ConjM2VarRetentionRow {
    pub b_model: &'static str,
    pub c: usize,
    pub n: usize,
    pub m: usize,
    pub exact: bool,
    pub feasibility_note: &'static str,
    pub var_t_struct: f64,
    pub var_t_lpn: f64,
    pub var_t_ratio: f64,
    pub var_wt_be: f64,
    pub var_wt_eprime: f64,
    pub raw_weight_ratio: f64,
    pub retention: f64,
    pub ratio_attenuation: f64,
    pub status: &'static str,
    pub claim_level: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

#[derive(Clone, Debug)]
pub struct ConjM2AgyPushforwardRow {
    pub b_model: &'static str,
    pub b_description: &'static str,
    pub c: usize,
    pub n: usize,
    pub m: usize,
    pub max_lagrangians_per_n: usize,
    pub lagrangian_scope: &'static str,
    pub lagrangians_evaluated: usize,
    pub lagrangian_total: usize,
    pub all_lagrangians_exact: bool,
    pub feasibility_note: &'static str,
    pub rank_b: usize,
    pub col_b_size: usize,
    pub var_t_struct: f64,
    pub var_t_lpn: f64,
    pub bernoulli_ratio: f64,
    pub var_full: f64,
    pub var_col_b: f64,
    pub pushforward_ratio: f64,
    pub push_over_bernoulli: f64,
    pub status: &'static str,
    pub claim_level: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

#[derive(Clone, Debug)]
pub struct ConjM2UsableLpnSdRow {
    pub model: &'static str,
    pub b_model: &'static str,
    pub c: usize,
    pub n: usize,
    pub m: usize,
    pub delta_gv: f64,
    pub realized_q: f64,
    pub q_exceeds_delta_gv: bool,
    pub lagrangian_scope: &'static str,
    pub lagrangians_evaluated: usize,
    pub lagrangian_total: usize,
    pub exact: bool,
    pub feasibility_note: &'static str,
    pub support_counting_quantity: f64,
    pub sd_mean: f64,
    pub sd_min: f64,
    pub sd_max: f64,
    pub status: &'static str,
    pub claim_level: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

pub fn conj_m2_n2_minweight_gate_report() -> Vec<ConjM2MinWeightGateRow> {
    [11usize, 12, 13]
.into_iter()
.map(|m| conj_m2_minweight_best_case_row(2, m))
.collect()
}

pub fn conj_m2_n2_overdispersion_report() -> Vec<ConjM2OverdispersionRow> {
    conj_m2_n2_minweight_gate_report()
.into_iter()
.map(|gate| conj_m2_overdispersion_row(gate.n, gate.m, gate.best_p_b, gate.best_p_prime))
.collect()
}

pub fn conj_m2_n2_overdispersion_report_json() -> String {
    let rows = conj_m2_n2_overdispersion_report();
    overdispersion_report_json(
        "-conj-m2-n2-overdispersion-char",
        "n=2 exact min-syndrome-weight T-distribution over-dispersion mechanism",
        &rows,
)
}

pub fn conj_m2_n3_overdispersion_report(max_m: usize) -> Vec<ConjM2OverdispersionRow> {
    conj_m2_n3_minweight_probe_report(max_m)
.into_iter()
.map(|gate| conj_m2_overdispersion_row(gate.n, gate.m, gate.best_p_b, gate.best_p_prime))
.collect()
}

pub fn conj_m2_n3_overdispersion_report_json(max_m: usize) -> String {
    let rows = conj_m2_n3_overdispersion_report(max_m);
    overdispersion_report_json(
        "-conj-m2-n3-overdispersion-char",
        "n=3 symmetry-DP min-syndrome-weight T-distribution over-dispersion mechanism",
        &rows,
)
}

pub fn conj_m2_n4_overdispersion_report(max_m: usize) -> Vec<ConjM2OverdispersionRow> {
    (1..=max_m)
.map(|m| conj_m2_minweight_best_case_row(4, m))
.map(|gate| conj_m2_overdispersion_row(gate.n, gate.m, gate.best_p_b, gate.best_p_prime))
.collect()
}

pub fn conj_m2_n4_overdispersion_report_json(max_m: usize) -> String {
    let n4_rows = conj_m2_n4_overdispersion_report(max_m);
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("  \"experiment\": \"-conj-m2-n4-overdispersion-char\",\n");
    out.push_str("  \"claim_level\": \"EVIDENCE\",\n");
    out.push_str("  \"status\": \"OPEN\",\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str("  \"target\": \"conj:m2 n=4 min-syndrome-weight T-distribution over-dispersion mechanism\",\n");
    out.push_str("  \"gate\": \"n=3 over-dispersion metrics reproduced before n=4 trust\",\n");
    out.push_str(&format!("  \"max_m_requested\": {max_m},\n"));
    out.push_str("  \"feasibility_note\": \"n=4 is reported only through the completed exact DP m range; m=3 and generic full-grid m=4 exceeded the bounded run budget and are deferred; no threshold is inferred beyond generated rows\",\n");
    out.push_str("  \"tail_definition\": \"upper_tail_mass is Pr[T >= ceil(mean + stddev)]; tail_ge_2_mass is Pr[T >= 2] for n=2/3/4 comparability\",\n");
    out.push_str("  \"n3_gate_reference_rows\": [\n");
    out.push_str("    {\"n\": 3, \"m\": 4, \"structured_variance\": 0.1415, \"gate_source\": \"PR #13 Rust n=3 over-dispersion bundle\"},\n");
    out.push_str("    {\"n\": 3, \"m\": 5, \"structured_variance\": 0.2064, \"gate_source\": \"PR #13 Rust n=3 over-dispersion bundle\"},\n");
    out.push_str("    {\"n\": 3, \"m\": 6, \"structured_variance\": 0.2806, \"gate_source\": \"PR #13 Rust n=3 over-dispersion bundle\"},\n");
    out.push_str("    {\"n\": 3, \"m\": 7, \"structured_variance\": 0.3637, \"gate_source\": \"PR #13 Rust n=3 over-dispersion bundle\"},\n");
    out.push_str("    {\"n\": 3, \"m\": 8, \"structured_variance\": 0.4557, \"gate_source\": \"PR #13 Rust n=3 over-dispersion bundle\"}\n");
    out.push_str("  ],\n");
    out.push_str("  \"n4_rows\": [\n");
    for (idx, row) in n4_rows.iter().enumerate() {
        let comma = if idx + 1 == n4_rows.len() { "" } else { "," };
        out.push_str(&overdispersion_row_json(row));
        out.push_str(comma);
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

pub fn conj_m2_samec_sd_scaling_report(max_c: usize) -> Vec<ConjM2OverdispersionRow> {
    [2usize, 3, 4]
.into_iter()
.filter(|c| *c <= max_c)
.flat_map(conj_m2_samec_sd_scaling_report_for_c)
.collect()
}

pub fn conj_m2_samec_sd_scaling_report_for_c(c: usize) -> Vec<ConjM2OverdispersionRow> {
    [
        conj_m2_minweight_best_case_row(2, 2 * c),
        conj_m2_minweight_best_case_row(3, 3 * c),
    ]
.into_iter()
.map(|gate| conj_m2_overdispersion_row(gate.n, gate.m, gate.best_p_b, gate.best_p_prime))
.collect()
}

pub fn conj_m2_samec_sd_scaling_comparisons(
    rows: &[ConjM2OverdispersionRow],
) -> Vec<ConjM2SameCScalingComparison> {
    [2usize, 3, 4]
.into_iter()
.filter_map(|c| {
            let n2 = rows.iter().find(|row| row.n == 2 && row.m == 2 * c)?;
            let n3 = rows.iter().find(|row| row.n == 3 && row.m == 3 * c)?;
            let n2_gap = variance_gap(n2);
            let n3_gap = variance_gap(n3);
            Some(ConjM2SameCScalingComparison {
                c,
                n2_sd1d: n2.sd1d,
                n3_sd1d: n3.sd1d,
                sd_direction_n2_to_n3: direction(n2.sd1d, n3.sd1d),
                n2_variance_gap: n2_gap,
                n3_variance_gap: n3_gap,
                variance_gap_direction_n2_to_n3: direction(n2_gap, n3_gap),
                status: "OPEN",
                claim_level: "EVIDENCE",
                reduction_claim: false,
                security_claim: false,
            })
        })
.collect()
}

pub fn conj_m2_samec_sd_scaling_report_json(max_c: usize) -> String {
    let rows = conj_m2_samec_sd_scaling_report(max_c);
    samec_sd_scaling_json(max_c, &rows)
}

pub fn conj_m2_samec_sd_scaling_single_c_report_json(c: usize) -> String {
    let rows = conj_m2_samec_sd_scaling_report_for_c(c);
    samec_sd_scaling_json(c, &rows)
}

pub fn conj_m2_heavy_left_tail_report(max_c: usize) -> Vec<ConjM2HeavyLeftTailRow> {
    (4..=max_c)
.flat_map(conj_m2_heavy_left_tail_report_for_c)
.collect()
}

pub fn conj_m2_heavy_left_tail_report_for_c(c: usize) -> Vec<ConjM2HeavyLeftTailRow> {
    [2usize, 3]
.into_iter()
.map(|n| conj_m2_heavy_left_tail_row(n, c))
.collect()
}

pub fn conj_m2_heavy_left_tail_report_json(max_c: usize) -> String {
    let rows = conj_m2_heavy_left_tail_report(max_c);
    heavy_left_tail_json(max_c, &rows)
}

pub fn conj_m2_heavy_left_tail_single_c_report_json(c: usize) -> String {
    let rows = conj_m2_heavy_left_tail_report_for_c(c);
    heavy_left_tail_json(c, &rows)
}

pub fn conj_m2_heavy_left_tail_profile_c4_report(max_n: usize) -> Vec<ConjM2HeavyLeftTailRow> {
    [2usize, 3, 4]
.into_iter()
.filter(|n| *n <= max_n)
.map(conj_m2_heavy_left_tail_profile_c4_row)
.collect()
}

pub fn conj_m2_heavy_left_tail_profile_c4_report_json(max_n: usize) -> String {
    let rows = conj_m2_heavy_left_tail_profile_c4_report(max_n);
    heavy_left_tail_profile_json(max_n, &rows)
}

pub fn conj_m2_var_retention_report(max_n: usize) -> Vec<ConjM2VarRetentionRow> {
    let mut rows = Vec::new();
    for n in [2usize, 3, 4] {
        if n > max_n {
            continue;
        }
        rows.push(var_retention_row_from_heavy_row(
            "fixed_weight_two_rows_profile",
            conj_m2_heavy_left_tail_profile_c4_row(n),
            fixed_b_raw_weight_variance(n, 4 * n),
            "exact fixed integer-sorted weight-2 B profile cell",
            true,
));
        if n <= 3 {
            rows.push(var_retention_row_from_heavy_row(
                "random_weight_two_rows_average",
                conj_m2_heavy_left_tail_row(n, 4),
                averaged_b_raw_weight_variance(n, 4 * n),
                "exact random weight-2 B row-law average cell",
                true,
));
        } else {
            rows.push(var_retention_infeasible_row(
                "random_weight_two_rows_average",
                n,
                "random-B averaged n=4 exact row-law cell is not extrapolated; existing exact DP machinery is bounded to n=2,3 for this report",
));
        }
    }
    rows
}

pub fn conj_m2_var_retention_report_json(max_n: usize) -> String {
    let rows = conj_m2_var_retention_report(max_n);
    var_retention_json(max_n, &rows)
}

pub fn conj_m2_agy_pushforward_report(
    max_n: usize,
    max_lagrangians_per_n: usize,
) -> Vec<ConjM2AgyPushforwardRow> {
    let mut rows = Vec::new();
    for n in 2..=max_n {
        let spans = conj_m2_lagrangian_span_masks(n);
        let lagrangian_total = spans.len();
        let lagrangians_evaluated = if max_lagrangians_per_n == 0 {
            lagrangian_total
        } else {
            lagrangian_total.min(max_lagrangians_per_n)
        };
        let selected_spans = &spans[..lagrangians_evaluated];
        let lagrangian_scope = if lagrangians_evaluated == lagrangian_total {
            "all_lagrangians"
        } else if lagrangians_evaluated == 1 {
            "first_lagrangian"
        } else {
            "prefix_sample"
        };
        let selected_bases: Vec<_> = selected_spans
.iter()
.map(|span| canonical_basis(span, n))
.collect();
        for (b_model, b_description, b_rows) in agy_pushforward_b_models(n) {
            rows.push(agy_pushforward_row(
                n,
                b_model,
                b_description,
                &b_rows,
                &selected_bases,
                lagrangian_total,
                max_lagrangians_per_n,
                lagrangian_scope,
                "computed by the generic bounded  pushforward report",
));
        }
    }
    rows
}

pub fn conj_m2_agy_pushforward_report_json(max_n: usize, max_lagrangians_per_n: usize) -> String {
    let rows = conj_m2_agy_pushforward_report(max_n, max_lagrangians_per_n);
    agy_pushforward_json(max_n, max_lagrangians_per_n, &rows)
}

pub fn conj_m2_agy_pushforward_fullrun_report(
    max_n: usize,
    n4_lagrangian_limit: usize,
    n5_lagrangian_limit: usize,
    n6_lagrangian_limit: usize,
) -> Vec<ConjM2AgyPushforwardRow> {
    let mut rows = Vec::new();
    for n in 2..=max_n {
        let schedule = agy_fullrun_lagrangian_schedule(
            n,
            n4_lagrangian_limit,
            n5_lagrangian_limit,
            n6_lagrangian_limit,
);
        for (b_model, b_description, b_rows) in agy_pushforward_b_models(n) {
            if schedule.profile_ceiling {
                rows.push(agy_pushforward_ceiling_row(
                    n,
                    b_model,
                    b_description,
                    &b_rows,
                    schedule.lagrangian_total,
                    schedule.requested_limit,
                    schedule.lagrangian_scope,
                    schedule.feasibility_note,
));
            } else {
                rows.push(agy_pushforward_row(
                    n,
                    b_model,
                    b_description,
                    &b_rows,
                    &schedule.bases,
                    schedule.lagrangian_total,
                    schedule.requested_limit,
                    schedule.lagrangian_scope,
                    schedule.feasibility_note,
));
            }
        }
    }
    rows
}

pub fn conj_m2_agy_pushforward_fullrun_report_json(
    max_n: usize,
    n4_lagrangian_limit: usize,
    n5_lagrangian_limit: usize,
    n6_lagrangian_limit: usize,
) -> String {
    let rows = conj_m2_agy_pushforward_fullrun_report(
        max_n,
        n4_lagrangian_limit,
        n5_lagrangian_limit,
        n6_lagrangian_limit,
);
    agy_pushforward_fullrun_json(
        max_n,
        n4_lagrangian_limit,
        n5_lagrangian_limit,
        n6_lagrangian_limit,
        &rows,
)
}

pub fn conj_m2_lemm2_usable_lpn_sd_report(
    max_n: usize,
    n4_lagrangian_limit: usize,
    n5_lagrangian_limit: usize,
) -> Vec<ConjM2UsableLpnSdRow> {
    let mut rows = Vec::new();
    for n in 2..=max_n {
        let c = 4usize;
        let m = c * n;
        let delta_gv = delta_gv_for_rate(1.0 / c as f64);
        let (bases, lagrangian_total, lagrangian_scope, exact, feasibility_note) =
            lemm2_sd_lagrangian_schedule(n, n4_lagrangian_limit, n5_lagrangian_limit);
        for (b_model, _, b_rows) in lemm2_sd_confined_b_models(n) {
            rows.push(lemm2_usable_lpn_sd_confined_row(
                n,
                b_model,
                &b_rows,
                &bases,
                lagrangian_total,
                lagrangian_scope,
                exact,
                feasibility_note,
                delta_gv,
));
        }
        rows.push(lemm2_usable_lpn_sd_pending_row(
            "random_weight_two_average_pending",
            n,
            m,
            delta_gv,
            "random weight-2 B ensemble SD is deferred; this row prevents mistaking confined-B evidence for diffuse proof",
));
        rows.push(lemm2_usable_lpn_sd_pending_row(
            "uniform_b_sanity_pending",
            n,
            m,
            delta_gv,
            "uniform-B sanity anchor is deferred; expected small SD is not inferred by this Rust fixture",
));
    }
    rows
}

pub fn conj_m2_lemm2_usable_lpn_sd_report_json(
    max_n: usize,
    n4_lagrangian_limit: usize,
    n5_lagrangian_limit: usize,
) -> String {
    let rows = conj_m2_lemm2_usable_lpn_sd_report(max_n, n4_lagrangian_limit, n5_lagrangian_limit);
    lemm2_usable_lpn_sd_json(max_n, n4_lagrangian_limit, n5_lagrangian_limit, &rows)
}

fn conj_m2_heavy_left_tail_row(n: usize, c: usize) -> ConjM2HeavyLeftTailRow {
    let m = n * c;
    let delta_gv = delta_gv_for_rate(1.0 / c as f64);
    let (structured, structured_states) = structured_t_dist_weight2(n, m);
    let (lpn, lpn_states) = t_dist_from_w(&w_lpn(n, delta_gv), n, m);
    let structured_metrics = t_distribution_metrics(&structured);
    let lpn_metrics = t_distribution_metrics(&lpn);
    let r_cov_t = lpn_metrics.saturation_t;
    let structured_side = left_tail_side(&structured, r_cov_t);
    let lpn_side = left_tail_side(&lpn, r_cov_t);
    ConjM2HeavyLeftTailRow {
        c,
        n,
        m,
        delta_gv,
        realized_q: 3.0 / 8.0,
        r_cov_t,
        sd1d: sd1d(&structured, &lpn),
        mean_lpn_minus_structured: lpn_metrics.mean - structured_metrics.mean,
        variance_gap_structured_minus_lpn: structured_metrics.variance - lpn_metrics.variance,
        structured: structured_side,
        lpn: lpn_side,
        structured_reachable_state_count: structured_states,
        lpn_reachable_state_count: lpn_states,
        row_model: "uniform weight-2 B rows (q=3/8)",
        status: "OPEN",
        claim_level: "EVIDENCE",
        reduction_claim: false,
        security_claim: false,
    }
}

fn conj_m2_heavy_left_tail_profile_c4_row(n: usize) -> ConjM2HeavyLeftTailRow {
    let c = 4usize;
    let m = n * c;
    let delta_gv = delta_gv_for_rate(1.0 / c as f64);
    let (structured, lpn, structured_states, lpn_states) =
        heavy_left_tail_profile_t_dists(n, m, delta_gv);
    let structured_metrics = t_distribution_metrics(&structured);
    let lpn_metrics = t_distribution_metrics(&lpn);
    let r_cov_t = lpn_metrics.saturation_t;
    let structured_side = left_tail_side(&structured, r_cov_t);
    let lpn_side = left_tail_side(&lpn, r_cov_t);
    ConjM2HeavyLeftTailRow {
        c,
        n,
        m,
        delta_gv,
        realized_q: 3.0 / 8.0,
        r_cov_t,
        sd1d: sd1d(&structured, &lpn),
        mean_lpn_minus_structured: lpn_metrics.mean - structured_metrics.mean,
        variance_gap_structured_minus_lpn: structured_metrics.variance - lpn_metrics.variance,
        structured: structured_side,
        lpn: lpn_side,
        structured_reachable_state_count: structured_states,
        lpn_reachable_state_count: lpn_states,
        row_model: "fixed integer-sorted weight-2 B-row coset-leader profile (q=3/8)",
        status: "OPEN",
        claim_level: "EVIDENCE",
        reduction_claim: false,
        security_claim: false,
    }
}

fn var_retention_row_from_heavy_row(
    b_model: &'static str,
    row: ConjM2HeavyLeftTailRow,
    var_wt_be: f64,
    feasibility_note: &'static str,
    exact: bool,
) -> ConjM2VarRetentionRow {
    let var_t_struct = row.structured.variance;
    let var_t_lpn = row.lpn.variance;
    let var_wt_eprime = raw_lpn_weight_variance(row.m, row.delta_gv);
    let var_t_ratio = var_t_struct / var_t_lpn;
    let raw_weight_ratio = var_wt_be / var_wt_eprime;
    let retention = (var_t_struct - var_t_lpn) / (var_wt_be - var_wt_eprime);
    let ratio_attenuation = (var_t_ratio - 1.0) / (raw_weight_ratio - 1.0);
    ConjM2VarRetentionRow {
        b_model,
        c: row.c,
        n: row.n,
        m: row.m,
        exact,
        feasibility_note,
        var_t_struct,
        var_t_lpn,
        var_t_ratio,
        var_wt_be,
        var_wt_eprime,
        raw_weight_ratio,
        retention,
        ratio_attenuation,
        status: "OPEN",
        claim_level: "EVIDENCE",
        reduction_claim: false,
        security_claim: false,
    }
}

fn var_retention_infeasible_row(
    b_model: &'static str,
    n: usize,
    feasibility_note: &'static str,
) -> ConjM2VarRetentionRow {
    ConjM2VarRetentionRow {
        b_model,
        c: 4,
        n,
        m: 4 * n,
        exact: false,
        feasibility_note,
        var_t_struct: f64::NAN,
        var_t_lpn: f64::NAN,
        var_t_ratio: f64::NAN,
        var_wt_be: f64::NAN,
        var_wt_eprime: f64::NAN,
        raw_weight_ratio: f64::NAN,
        retention: f64::NAN,
        ratio_attenuation: f64::NAN,
        status: "OPEN",
        claim_level: "EVIDENCE",
        reduction_claim: false,
        security_claim: false,
    }
}

fn samec_sd_scaling_json(max_c_requested: usize, rows: &[ConjM2OverdispersionRow]) -> String {
    let comparisons = conj_m2_samec_sd_scaling_comparisons(&rows);
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("  \"experiment\": \"-conj-m2-samec-sd-scaling\",\n");
    out.push_str("  \"claim_level\": \"EVIDENCE\",\n");
    out.push_str("  \"status\": \"OPEN\",\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str("  \"target\": \"same-c structured-vs-matched-rate-LPN SD scaling for conj:m2 n=2 vs n=3\",\n");
    out.push_str("  \"gate\": \"n=3 m=6 structured variance reference is 0.2806 from PR #13; completed rows recompute exact distributions\",\n");
    out.push_str(&format!("  \"max_c_requested\": {max_c_requested},\n"));
    out.push_str("  \"planned_grid\": [\n");
    out.push_str("    {\"c\": 2, \"n2_m\": 4, \"n3_m\": 6},\n");
    out.push_str("    {\"c\": 3, \"n2_m\": 6, \"n3_m\": 9},\n");
    out.push_str("    {\"c\": 4, \"n2_m\": 8, \"n3_m\": 12}\n");
    out.push_str("  ],\n");
    out.push_str("  \"feasibility_note\": \"only completed exact cells are compared; missing c-cells carry no inferred trend\",\n");
    out.push_str("  \"tail_definition\": \"upper_tail_mass is Pr[T >= ceil(mean + stddev)]; tail_ge_2_mass is Pr[T >= 2]\",\n");
    out.push_str("  \"completed_cells\": [\n");
    for (idx, row) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        out.push_str(&samec_cell_json(row));
        out.push_str(comma);
        out.push('\n');
    }
    out.push_str("  ],\n");
    out.push_str("  \"comparisons\": [\n");
    for (idx, comparison) in comparisons.iter().enumerate() {
        let comma = if idx + 1 == comparisons.len() {
            ""
        } else {
            ","
        };
        out.push_str(&samec_comparison_json(comparison));
        out.push_str(comma);
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

fn heavy_left_tail_json(max_c_requested: usize, rows: &[ConjM2HeavyLeftTailRow]) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("  \"experiment\": \"-conj-m2-heavy-left-tail-asymmetry\",\n");
    out.push_str("  \"claim_level\": \"EVIDENCE\",\n");
    out.push_str("  \"status\": \"OPEN\",\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str("  \"target\": \"heavy-rate q=3/8 left-tail asymmetry for conj:m2 min-syndrome-weight T\",\n");
    out.push_str("  \"regime_correction\": \"heavy weight-2 B rows with q=3/8; this is the corrected residual-band regime, unlike the older light-rate p_B=0.125 same-c ladder\",\n");
    out.push_str("  \"r_cov_definition\": \"operational proxy: LPN max support t with probability > 1e-12; mode and max support are reported separately for both laws\",\n");
    out.push_str(&format!("  \"max_c_requested\": {max_c_requested},\n"));
    out.push_str("  \"rows\": [\n");
    for (idx, row) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        out.push_str(&heavy_left_tail_row_json(row));
        out.push_str(comma);
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

fn heavy_left_tail_profile_json(max_n_requested: usize, rows: &[ConjM2HeavyLeftTailRow]) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("  \"experiment\": \"-conj-m2-heavy-n4-thirdpoint-profile\",\n");
    out.push_str("  \"claim_level\": \"EVIDENCE\",\n");
    out.push_str("  \"status\": \"OPEN\",\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str(
        "  \"target\": \"c=4 heavy-rate q=3/8 left-tail n=2,3 gate plus n=4 third point\",\n",
);
    out.push_str("  \"profile_method\": \"for each Lagrangian A, build a cyclic weight-2 B-row matrix, compute the C=BA coset-leader-weight profile once, then weight the same profile by LPN Bernoulli(delta_GV)^m and structured Be with e~Bernoulli(1/4)^(2n)\",\n");
    out.push_str("  \"gate\": \"n=2 and n=3 profile rows are reported beside the n=4 third point; any profile mismatch must be treated as evidence-method drift, not as an asymptotic claim\",\n");
    out.push_str("  \"gate_result\": \"profile rows do not exactly reproduce PR #19 row-law n=2/n=3 metrics; n=4 is profile-method evidence only, not a trusted extension of the row-law fixture\",\n");
    out.push_str("  \"posture\": \"OPEN / EVIDENCE only / NOT A REDUCTION / NOT A SECURITY CLAIM / no production constant-time claim\",\n");
    out.push_str(&format!("  \"max_n_requested\": {max_n_requested},\n"));
    out.push_str("  \"rows\": [\n");
    for (idx, row) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        out.push_str(&heavy_left_tail_row_json(row));
        out.push_str(comma);
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

fn var_retention_json(max_n_requested: usize, rows: &[ConjM2VarRetentionRow]) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("  \"experiment\": \"-conj-m2-overdispersion-retention\",\n");
    out.push_str("  \"claim_level\": \"EVIDENCE\",\n");
    out.push_str("  \"status\": \"OPEN\",\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str("  \"target\": \"c=4 q=3/8 over-dispersion ratio and raw-weight retention for random-B averaged and fixed-B profile models\",\n");
    out.push_str("  \"retention_definition\": \"(var_T_struct - var_T_lpn) / (var_wt_Be - var_wt_eprime)\",\n");
    out.push_str("  \"ratio_attenuation_definition\": \"(var_T_struct/var_T_lpn - 1) / (var_wt_Be/var_wt_eprime - 1)\",\n");
    out.push_str("  \"n4_debug_fix\": \"fixed weight-2 rows use integer-sorted order matching Independent exp938; the previous pair-loop order changed the n=4 fixed-B model and produced the wrong attenuation\",\n");
    out.push_str("  \"scope_boundary\": \"random-B averaged n=4 is not extrapolated; missing cells are explicit null-valued infeasible rows\",\n");
    out.push_str("  \"posture\": \"OPEN / EVIDENCE only / NOT A REDUCTION / NOT A SECURITY CLAIM / no production constant-time claim\",\n");
    out.push_str(&format!("  \"max_n_requested\": {max_n_requested},\n"));
    out.push_str("  \"rows\": [\n");
    for (idx, row) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        out.push_str(&var_retention_row_json(row));
        out.push_str(comma);
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

fn var_retention_row_json(row: &ConjM2VarRetentionRow) -> String {
    format!(
        concat!(
            "    {{\"b_model\": \"{}\", \"c\": {}, \"n\": {}, \"m\": {}, ",
            "\"exact\": {}, \"feasibility_note\": \"{}\", ",
            "\"var_T_struct\": {}, \"var_T_lpn\": {}, \"var_T_ratio\": {}, ",
            "\"var_wt_Be\": {}, \"var_wt_eprime\": {}, \"raw_weight_ratio\": {}, ",
            "\"retention\": {}, \"ratio_attenuation\": {}, ",
            "\"status\": \"{}\", \"claim_level\": \"{}\", ",
            "\"reduction_claim\": {}, \"security_claim\": {}}}"
),
        row.b_model,
        row.c,
        row.n,
        row.m,
        row.exact,
        row.feasibility_note,
        json_number(row.var_t_struct),
        json_number(row.var_t_lpn),
        json_number(row.var_t_ratio),
        json_number(row.var_wt_be),
        json_number(row.var_wt_eprime),
        json_number(row.raw_weight_ratio),
        json_number(row.retention),
        json_number(row.ratio_attenuation),
        row.status,
        row.claim_level,
        row.reduction_claim,
        row.security_claim
)
}

fn agy_pushforward_json(
    max_n_requested: usize,
    max_lagrangians_per_n: usize,
    rows: &[ConjM2AgyPushforwardRow],
) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("  \"experiment\": \"-conj-m2--pushforward-ratio\",\n");
    out.push_str("  \"claim_level\": \"EVIDENCE\",\n");
    out.push_str("  \"status\": \"OPEN\",\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str("  \"target\": \"same-B Bernoulli ratio versus uniform-pushforward ratio\",\n");
    out.push_str("  \"b_models\": [\"circulant_gaps_1_2\", \"circulant_gaps_1_3\", \"first_m_integer_sorted\"],\n");
    out.push_str("  \"bernoulli_ratio_definition\": \"Var(T_C(Be), e~Bernoulli(1/4)^(2n)) / Var(T_C(eprime), eprime~Bernoulli(delta_GV)^m)\",\n");
    out.push_str(
        "  \"pushforward_ratio_definition\": \"Var_{U(Col B)}(T_C) / Var_{U(F_2^m)}(T_C)\",\n",
);
    out.push_str("  \"scope_boundary\": \"B-dependent trend caveat: report same-B columns side by side; no horn-closing lean is inferred from a single B construction or finite sampled prefix\",\n");
    out.push_str("  \"posture\": \"OPEN / EVIDENCE only / NOT A REDUCTION / NOT A SECURITY CLAIM / no production constant-time claim\",\n");
    out.push_str(&format!("  \"max_n_requested\": {max_n_requested},\n"));
    out.push_str(&format!(
        "  \"max_lagrangians_per_n\": {max_lagrangians_per_n},\n"
));
    out.push_str("  \"rows\": [\n");
    for (idx, row) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        out.push_str(&agy_pushforward_row_json(row));
        out.push_str(comma);
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

fn agy_pushforward_fullrun_json(
    max_n_requested: usize,
    n4_lagrangian_limit: usize,
    n5_lagrangian_limit: usize,
    n6_lagrangian_limit: usize,
    rows: &[ConjM2AgyPushforwardRow],
) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("  \"experiment\": \"-conj-m2--pushforward-fullrun\",\n");
    out.push_str("  \"fixture\": \"947-conj-m2-pushforward-exact-trend\",\n");
    out.push_str("  \"claim_level\": \"EVIDENCE\",\n");
    out.push_str("  \"status\": \"OPEN\",\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str("  \"target\": \"decisive full-run schedule for same-B Bernoulli ratio versus uniform-pushforward ratio\",\n");
    out.push_str("  \"prediction_boundary\": \"both prior analytic predictions are retracted or falsified; this fixture reports the finite exact/sample trend without targeting R->1 or R->2.01\",\n");
    out.push_str("  \"n2_n3_policy\": \"exact all Lagrangians regardless of sample limits\",\n");
    out.push_str("  \"n4_policy\": \"exact if n4_lagrangian_limit is 0 or >= total; otherwise evenly spaced sample\",\n");
    out.push_str("  \"n5_policy\": \"deterministic isotropic-basis sample\",\n");
    out.push_str("  \"n6_policy\": \"profile ceiling unless explicitly made feasible later\",\n");
    out.push_str("  \"individual_variance_fields\": [\"var_full\", \"var_colB\"],\n");
    out.push_str("  \"exp945_boundary\": \"use the verified combined-form identity only as audit context; this JSON does not claim the earlier split-form R_n reduction\",\n");
    out.push_str("  \"posture\": \"OPEN / EVIDENCE only / NOT A REDUCTION / NOT A SECURITY CLAIM / no production constant-time claim\",\n");
    out.push_str(&format!("  \"max_n_requested\": {max_n_requested},\n"));
    out.push_str("  \"recommended_n4_lagrangian_limit\": 0,\n");
    out.push_str("  \"recommended_n5_lagrangian_limit\": 200,\n");
    out.push_str(&format!(
        "  \"n4_lagrangian_limit\": {n4_lagrangian_limit},\n"
));
    out.push_str(&format!(
        "  \"n5_lagrangian_limit\": {n5_lagrangian_limit},\n"
));
    out.push_str(&format!(
        "  \"n6_lagrangian_limit\": {n6_lagrangian_limit},\n"
));
    out.push_str("  \"rows\": [\n");
    for (idx, row) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        out.push_str(&agy_pushforward_row_json(row));
        out.push_str(comma);
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

fn agy_pushforward_row_json(row: &ConjM2AgyPushforwardRow) -> String {
    format!(
        concat!(
            "    {{\"b_model\": \"{}\", \"b_description\": \"{}\", ",
            "\"c\": {}, \"n\": {}, \"m\": {}, ",
            "\"max_lagrangians_per_n\": {}, \"lagrangian_scope\": \"{}\", ",
            "\"lagrangians_evaluated\": {}, \"lagrangian_total\": {}, ",
            "\"all_lagrangians_exact\": {}, \"feasibility_note\": \"{}\", ",
            "\"rank_b\": {}, \"col_b_size\": {}, ",
            "\"var_T_struct\": {}, \"var_T_lpn\": {}, \"bernoulli_ratio\": {}, ",
            "\"var_full\": {}, \"var_colB\": {}, \"pushforward_ratio\": {}, ",
            "\"push_over_bernoulli\": {}, \"status\": \"{}\", \"claim_level\": \"{}\", ",
            "\"reduction_claim\": {}, \"security_claim\": {}}}"
),
        row.b_model,
        row.b_description,
        row.c,
        row.n,
        row.m,
        row.max_lagrangians_per_n,
        row.lagrangian_scope,
        row.lagrangians_evaluated,
        row.lagrangian_total,
        row.all_lagrangians_exact,
        row.feasibility_note,
        row.rank_b,
        row.col_b_size,
        json_number(row.var_t_struct),
        json_number(row.var_t_lpn),
        json_number(row.bernoulli_ratio),
        json_number(row.var_full),
        json_number(row.var_col_b),
        json_number(row.pushforward_ratio),
        json_number(row.push_over_bernoulli),
        row.status,
        row.claim_level,
        row.reduction_claim,
        row.security_claim
)
}

fn lemm2_usable_lpn_sd_json(
    max_n_requested: usize,
    n4_lagrangian_limit: usize,
    n5_lagrangian_limit: usize,
    rows: &[ConjM2UsableLpnSdRow],
) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("  \"experiment\": \"-lemm2-usable-lpn-sd-scale\",\n");
    out.push_str("  \"fixture\": \"957-lemm2-diffuse-SD-scale\",\n");
    out.push_str("  \"claim_level\": \"EVIDENCE\",\n");
    out.push_str("  \"status\": \"OPEN\",\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str("  \"target\": \"usable LPN target delta_GV, not matched-rate LPN; SD(nu mod Col C, mu_delta_GV mod Col C)\",\n");
    out.push_str("  \"gate\": \"Independent gates: confined n=2 SD approx 0.850 and n=3 SD approx 0.943 before trusting scale rows\",\n");
    out.push_str("  \"diffuse_boundary\": \"confined fixed-B rows are not a diffuse proof; random-weight-2 and uniform-B rows remain explicit pending cells until implemented\",\n");
    out.push_str("  \"posture\": \"OPEN / EVIDENCE only / NOT A REDUCTION / NOT A SECURITY CLAIM / no production constant-time claim\",\n");
    out.push_str(&format!("  \"max_n_requested\": {max_n_requested},\n"));
    out.push_str(&format!(
        "  \"n4_lagrangian_limit\": {n4_lagrangian_limit},\n"
));
    out.push_str(&format!(
        "  \"n5_lagrangian_limit\": {n5_lagrangian_limit},\n"
));
    out.push_str("  \"rows\": [\n");
    for (idx, row) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        out.push_str(&lemm2_usable_lpn_sd_row_json(row));
        out.push_str(comma);
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

fn lemm2_usable_lpn_sd_row_json(row: &ConjM2UsableLpnSdRow) -> String {
    format!(
        concat!(
            "    {{\"model\": \"{}\", \"b_model\": \"{}\", ",
            "\"c\": {}, \"n\": {}, \"m\": {}, \"delta_gv\": {}, ",
            "\"realized_q\": {}, \"q_exceeds_delta_gv\": {}, ",
            "\"lagrangian_scope\": \"{}\", \"lagrangians_evaluated\": {}, ",
            "\"lagrangian_total\": {}, \"exact\": {}, \"feasibility_note\": \"{}\", ",
            "\"support_counting_quantity\": {}, \"sd_mean\": {}, \"sd_min\": {}, \"sd_max\": {}, ",
            "\"status\": \"{}\", \"claim_level\": \"{}\", ",
            "\"reduction_claim\": {}, \"security_claim\": {}}}"
),
        row.model,
        row.b_model,
        row.c,
        row.n,
        row.m,
        json_number(row.delta_gv),
        json_number(row.realized_q),
        row.q_exceeds_delta_gv,
        row.lagrangian_scope,
        row.lagrangians_evaluated,
        row.lagrangian_total,
        row.exact,
        row.feasibility_note,
        json_number(row.support_counting_quantity),
        json_number(row.sd_mean),
        json_number(row.sd_min),
        json_number(row.sd_max),
        row.status,
        row.claim_level,
        row.reduction_claim,
        row.security_claim
)
}

fn json_number(value: f64) -> String {
    if value.is_finite() {
        format!("{value:.12}")
    } else {
        "null".to_string()
    }
}

fn overdispersion_report_json(
    experiment: &str,
    target: &str,
    rows: &[ConjM2OverdispersionRow],
) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("  \"experiment\": \"{}\",\n", experiment));
    out.push_str("  \"claim_level\": \"EVIDENCE\",\n");
    out.push_str("  \"status\": \"OPEN\",\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str(&format!("  \"target\": \"{}\",\n", target));
    out.push_str("  \"tail_definition\": \"upper_tail_mass is Pr[T >= ceil(mean + stddev)]; saturation_mass is mass at largest T with non-negligible probability\",\n");
    out.push_str("  \"rows\": [\n");
    for (idx, row) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        out.push_str(&overdispersion_row_json(row));
        out.push_str(comma);
        out.push('\n');
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

fn heavy_left_tail_row_json(row: &ConjM2HeavyLeftTailRow) -> String {
    format!(
        concat!(
            "    {{\"c\": {}, \"n\": {}, \"m\": {}, ",
            "\"delta_gv\": {:.12}, \"realized_q\": {:.12}, \"q_exceeds_delta_gv\": {}, ",
            "\"r_cov_t\": {}, \"sd1d\": {:.12}, ",
            "\"mean_lpn_minus_structured\": {:.12}, ",
            "\"variance_gap_structured_minus_lpn\": {:.12}, ",
            "\"structured\": {}, \"lpn\": {}, ",
            "\"structured_reachable_state_count\": {}, ",
            "\"lpn_reachable_state_count\": {}, ",
            "\"row_model\": \"{}\", \"status\": \"{}\", \"claim_level\": \"{}\", ",
            "\"reduction_claim\": {}, \"security_claim\": {}}}"
),
        row.c,
        row.n,
        row.m,
        row.delta_gv,
        row.realized_q,
        row.realized_q > row.delta_gv,
        row.r_cov_t,
        row.sd1d,
        row.mean_lpn_minus_structured,
        row.variance_gap_structured_minus_lpn,
        left_tail_side_json(&row.structured),
        left_tail_side_json(&row.lpn),
        row.structured_reachable_state_count,
        row.lpn_reachable_state_count,
        row.row_model,
        row.status,
        row.claim_level,
        row.reduction_claim,
        row.security_claim
)
}

fn left_tail_side_json(side: &ConjM2LeftTailSide) -> String {
    format!(
        concat!(
            "{{\"mean\": {:.12}, \"variance\": {:.12}, ",
            "\"mode_t\": {}, \"max_support_t\": {}, ",
            "\"left_tail_mass_below_r_cov\": {:.12}, ",
            "\"atom_at_r_cov\": {:.12}}}"
),
        side.mean,
        side.variance,
        side.mode_t,
        side.max_support_t,
        side.left_tail_mass_below_r_cov,
        side.atom_at_r_cov
)
}

fn samec_cell_json(row: &ConjM2OverdispersionRow) -> String {
    format!(
        concat!(
            "    {{\"c\": {}, \"n\": {}, \"m\": {}, ",
            "\"p_b\": {:.12}, \"p_prime\": {:.12}, \"sd1d\": {:.12}, ",
            "\"variance_gap_structured_minus_lpn\": {:.12}, ",
            "\"structured_metrics\": {}, \"lpn_metrics\": {}, ",
            "\"status\": \"{}\", \"claim_level\": \"{}\", ",
            "\"reduction_claim\": {}, \"security_claim\": {}}}"
),
        row.m / row.n,
        row.n,
        row.m,
        row.p_b,
        row.p_prime,
        row.sd1d,
        variance_gap(row),
        t_metrics_json(&row.structured_metrics),
        t_metrics_json(&row.lpn_metrics),
        row.status,
        row.claim_level,
        row.reduction_claim,
        row.security_claim
)
}

fn samec_comparison_json(comparison: &ConjM2SameCScalingComparison) -> String {
    format!(
        concat!(
            "    {{\"c\": {}, \"n2_sd1d\": {:.12}, \"n3_sd1d\": {:.12}, ",
            "\"sd_direction_n2_to_n3\": \"{}\", ",
            "\"n2_variance_gap\": {:.12}, \"n3_variance_gap\": {:.12}, ",
            "\"variance_gap_direction_n2_to_n3\": \"{}\", ",
            "\"status\": \"{}\", \"claim_level\": \"{}\", ",
            "\"reduction_claim\": {}, \"security_claim\": {}}}"
),
        comparison.c,
        comparison.n2_sd1d,
        comparison.n3_sd1d,
        comparison.sd_direction_n2_to_n3,
        comparison.n2_variance_gap,
        comparison.n3_variance_gap,
        comparison.variance_gap_direction_n2_to_n3,
        comparison.status,
        comparison.claim_level,
        comparison.reduction_claim,
        comparison.security_claim
)
}

fn variance_gap(row: &ConjM2OverdispersionRow) -> f64 {
    row.structured_metrics.variance - row.lpn_metrics.variance
}

fn direction(left: f64, right: f64) -> &'static str {
    if right > left + 1e-12 {
        "grows"
    } else if right + 1e-12 < left {
        "shrinks"
    } else {
        "flat"
    }
}

fn left_tail_side(dist: &[f64], r_cov_t: usize) -> ConjM2LeftTailSide {
    let metrics = t_distribution_metrics(dist);
    let mode_t = dist
.iter()
.enumerate()
.max_by(|(_, left), (_, right)| {
            left.partial_cmp(right)
.expect("distribution probabilities must be finite")
        })
.map(|(t, _)| t)
.unwrap_or(0);
    ConjM2LeftTailSide {
        mean: metrics.mean,
        variance: metrics.variance,
        mode_t,
        max_support_t: metrics.saturation_t,
        left_tail_mass_below_r_cov: dist.iter().take(r_cov_t).sum(),
        atom_at_r_cov: dist.get(r_cov_t).copied().unwrap_or(0.0),
    }
}

fn delta_gv_for_rate(rate: f64) -> f64 {
    let target = 1.0 - rate;
    let mut lo = 0.0f64;
    let mut hi = 0.5f64;
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if binary_entropy(mid) < target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

fn binary_entropy(p: f64) -> f64 {
    if p <= 0.0 || p >= 1.0 {
        return 0.0;
    }
    -p * p.log2() - (1.0 - p) * (1.0 - p).log2()
}

fn structured_t_dist_weight2(n: usize, m: usize) -> (Vec<f64>, usize) {
    let prior = structured_prior(n, 0.25);
    let mut cache: HashMap<Vec<u64>, (Vec<f64>, usize)> = HashMap::new();
    let mut mix = vec![0.0; m + 1];
    let mut max_states = 0usize;

    for (basis, v, pr) in prior {
        let w = w_structured_weight2(n, &basis, v);
        let key = canonical_w_key(&w, n);
        let (tdist, states) = cache
.entry(key.clone())
.or_insert_with(|| t_dist_from_w(&w, n, m));
        max_states = max_states.max(*states);
        for (dst, src) in mix.iter_mut().zip(tdist.iter()) {
            *dst += pr * src;
        }
    }

    (mix, max_states)
}

fn heavy_left_tail_profile_t_dists(
    n: usize,
    m: usize,
    p_lpn: f64,
) -> (Vec<f64>, Vec<f64>, usize, usize) {
    assert!(m <= 16, "profile method stores syndromes in u32");
    let spans = conj_m2_lagrangian_span_masks(n);
    let lagrangian_probability = 1.0 / spans.len() as f64;
    let b_rows = fixed_weight_two_rows(2 * n, m);
    let hamming_weights = hamming_weight_table(m);
    let mut structured = vec![0.0; m + 1];
    let mut lpn = vec![0.0; m + 1];
    let mut max_structured_syndromes = 0usize;
    let mut max_lpn_cosets = 0usize;

    for span in spans {
        let basis = canonical_basis(&span, n);
        let codewords = b_times_lagrangian_codewords(n, &b_rows, &basis);
        let profile = coset_leader_weight_profile(m, &codewords, &hamming_weights);
        max_lpn_cosets = max_lpn_cosets.max(1usize << (m - n));

        for z in 0..(1usize << m) {
            let t = profile[z] as usize;
            let probability = bernoulli_vector_probability_usize(z, m, p_lpn);
            lpn[t] += lagrangian_probability * probability;
        }

        let mut structured_syndromes = HashSet::new();
        for e in 0..(1usize << (2 * n)) {
            let syndrome = b_times_vector_syndrome(n, &b_rows, e as u16) as usize;
            structured_syndromes.insert(syndrome);
            let t = profile[syndrome] as usize;
            let probability = bernoulli_vector_probability_usize(e, 2 * n, 0.25);
            structured[t] += lagrangian_probability * probability;
        }
        max_structured_syndromes = max_structured_syndromes.max(structured_syndromes.len());
    }

    (structured, lpn, max_structured_syndromes, max_lpn_cosets)
}

fn fixed_b_raw_weight_variance(n: usize, m: usize) -> f64 {
    let b_rows = fixed_weight_two_rows(2 * n, m);
    let mut mean = 0.0f64;
    let mut second_moment = 0.0f64;
    for e in 0..(1usize << (2 * n)) {
        let probability = bernoulli_vector_probability_usize(e, 2 * n, 0.25);
        let weight = b_times_vector_syndrome(n, &b_rows, e as u16).count_ones() as f64;
        mean += probability * weight;
        second_moment += probability * weight * weight;
    }
    second_moment - mean * mean
}

fn averaged_b_raw_weight_variance(n: usize, m: usize) -> f64 {
    let rows = weight_two_rows(2 * n);
    let row_count = rows.len() as f64;
    let mut expected_conditional_variance = 0.0f64;
    let mut mean = 0.0f64;
    let mut second_conditional_mean = 0.0f64;
    for e in 0..(1u16 << (2 * n)) {
        let probability = bernoulli_vector_probability(e, 2 * n, 0.25);
        let hit_probability = rows
.iter()
.filter(|&&row| conj_m2_symplectic_pairing(row, e, n) != 0)
.count() as f64
            / row_count;
        let conditional_mean = m as f64 * hit_probability;
        let conditional_variance = m as f64 * hit_probability * (1.0 - hit_probability);
        expected_conditional_variance += probability * conditional_variance;
        mean += probability * conditional_mean;
        second_conditional_mean += probability * conditional_mean * conditional_mean;
    }
    expected_conditional_variance + second_conditional_mean - mean * mean
}

fn raw_lpn_weight_variance(m: usize, p_prime: f64) -> f64 {
    m as f64 * p_prime * (1.0 - p_prime)
}

fn fixed_weight_two_rows(width: usize, m: usize) -> Vec<u16> {
    let mut rows = weight_two_rows(width);
    rows.sort_unstable();
    (0..m).map(|idx| rows[idx % rows.len()]).collect()
}

fn agy_pushforward_b_models(n: usize) -> Vec<(&'static str, &'static str, Vec<u16>)> {
    let m = 4 * n;
    vec![
        (
            "circulant_gaps_1_2",
            "consistent circulant weight-2 B rows with gaps {1,2}; Independent exp943 worrying decreasing trend",
            circulant_weight_two_rows(2 * n, &[1, 2]),
),
        (
            "circulant_gaps_1_3",
            "consistent circulant weight-2 B rows with gaps {1,3}; second deterministic B-construction check",
            circulant_weight_two_rows(2 * n, &[1, 3]),
),
        (
            "first_m_integer_sorted",
            "first m integer-sorted weight-2 rows; included as the PR #22 fixed-profile comparator",
            fixed_weight_two_rows(2 * n, m),
),
    ]
}

fn lemm2_sd_confined_b_models(n: usize) -> Vec<(&'static str, &'static str, Vec<u16>)> {
    let m = 4 * n;
    vec![
        (
            "first_m_integer_sorted",
            "first m integer-sorted weight-2 rows",
            fixed_weight_two_rows(2 * n, m),
),
        (
            "circulant_gaps_1_2",
            "consistent circulant weight-2 rows with gaps {1,2}",
            circulant_weight_two_rows(2 * n, &[1, 2]),
),
    ]
}

fn lemm2_sd_lagrangian_schedule(
    n: usize,
    n4_lagrangian_limit: usize,
    n5_lagrangian_limit: usize,
) -> (Vec<Vec<u16>>, usize, &'static str, bool, &'static str) {
    let lagrangian_total = lagrangian_count_formula(n);
    match n {
        2 | 3 => (
            exact_lagrangian_bases(n),
            lagrangian_total,
            "all_lagrangians",
            true,
            "Independent confined gate cell, exact all-Lagrangians",
),
        4 => {
            let bases = exact_lagrangian_bases(n);
            if n4_lagrangian_limit == 0 || n4_lagrangian_limit >= bases.len() {
                (
                    bases,
                    lagrangian_total,
                    "all_lagrangians",
                    true,
                    "exact n=4 all-Lagrangians confined SD cell",
)
            } else {
                (
                    evenly_spaced_bases(&bases, n4_lagrangian_limit.max(1)),
                    lagrangian_total,
                    "even_spaced_sample",
                    false,
                    "deterministic evenly spaced n=4 sample",
)
            }
        }
        5 => (
            deterministic_lagrangian_basis_sample(n, n5_lagrangian_limit.max(1)),
            lagrangian_total,
            "deterministic_basis_sample",
            false,
            "deterministic n=5 isotropic-basis sample",
),
        _ => (
            Vec::new(),
            lagrangian_total,
            "profile_ceiling",
            false,
            "profile ceiling: SD row not computed for this n in the current Rust fixture",
),
    }
}

fn lemm2_usable_lpn_sd_confined_row(
    n: usize,
    b_model: &'static str,
    b_rows: &[u16],
    bases: &[Vec<u16>],
    lagrangian_total: usize,
    lagrangian_scope: &'static str,
    exact: bool,
    feasibility_note: &'static str,
    delta_gv: f64,
) -> ConjM2UsableLpnSdRow {
    let c = 4usize;
    let m = c * n;
    if bases.is_empty() {
        return lemm2_usable_lpn_sd_pending_row(
            "confined_fixed_b",
            n,
            m,
            delta_gv,
            feasibility_note,
);
    }
    let mut sd_values = Vec::with_capacity(bases.len());
    for basis in bases {
        let codewords = b_times_lagrangian_codewords(n, b_rows, basis);
        sd_values.push(sd_usable_lpn_for_code(n, m, b_rows, &codewords, delta_gv));
    }
    let sd_mean = sd_values.iter().sum::<f64>() / sd_values.len() as f64;
    let sd_min = sd_values
.iter()
.copied()
.fold(f64::INFINITY, |acc, item| acc.min(item));
    let sd_max = sd_values
.iter()
.copied()
.fold(f64::NEG_INFINITY, |acc, item| acc.max(item));
    let realized_q = realized_q_for_rows(b_rows);
    ConjM2UsableLpnSdRow {
        model: "confined_fixed_b",
        b_model,
        c,
        n,
        m,
        delta_gv,
        realized_q,
        q_exceeds_delta_gv: realized_q > delta_gv,
        lagrangian_scope,
        lagrangians_evaluated: bases.len(),
        lagrangian_total,
        exact,
        feasibility_note,
        support_counting_quantity: (1usize << n) as f64 * (1.0 - delta_gv).powi(m as i32),
        sd_mean,
        sd_min,
        sd_max,
        status: "OPEN",
        claim_level: "EVIDENCE",
        reduction_claim: false,
        security_claim: false,
    }
}

fn lemm2_usable_lpn_sd_pending_row(
    model: &'static str,
    n: usize,
    m: usize,
    delta_gv: f64,
    feasibility_note: &'static str,
) -> ConjM2UsableLpnSdRow {
    ConjM2UsableLpnSdRow {
        model,
        b_model: "not_computed",
        c: 4,
        n,
        m,
        delta_gv,
        realized_q: f64::NAN,
        q_exceeds_delta_gv: false,
        lagrangian_scope: "pending",
        lagrangians_evaluated: 0,
        lagrangian_total: lagrangian_count_formula(n),
        exact: false,
        feasibility_note,
        support_counting_quantity: f64::NAN,
        sd_mean: f64::NAN,
        sd_min: f64::NAN,
        sd_max: f64::NAN,
        status: "OPEN",
        claim_level: "EVIDENCE",
        reduction_claim: false,
        security_claim: false,
    }
}

fn sd_usable_lpn_for_code(
    n: usize,
    m: usize,
    b_rows: &[u16],
    codewords: &[u32],
    delta_gv: f64,
) -> f64 {
    let mut nu = HashMap::<u32, f64>::new();
    let mut mu = HashMap::<u32, f64>::new();
    for e in 0..(1usize << (2 * n)) {
        let z = b_times_vector_syndrome(n, b_rows, e as u16);
        let key = canonical_coset_rep(z, codewords);
        *nu.entry(key).or_insert(0.0) += bernoulli_vector_probability_usize(e, 2 * n, 0.25);
    }
    for z in 0..(1usize << m) {
        let key = canonical_coset_rep(z as u32, codewords);
        *mu.entry(key).or_insert(0.0) += bernoulli_vector_probability_usize(z, m, delta_gv);
    }
    let mut keys = HashSet::new();
    keys.extend(nu.keys().copied());
    keys.extend(mu.keys().copied());
    0.5 * keys
.into_iter()
.map(|key| {
            (nu.get(&key).copied().unwrap_or(0.0) - mu.get(&key).copied().unwrap_or(0.0)).abs()
        })
.sum::<f64>()
}

fn canonical_coset_rep(z: u32, codewords: &[u32]) -> u32 {
    codewords
.iter()
.map(|&codeword| z ^ codeword)
.min()
.expect("codeword list must be nonempty")
}

fn realized_q_for_rows(rows: &[u16]) -> f64 {
    rows.iter()
.map(|row| 0.5 * (1.0 - 0.5f64.powi(row.count_ones() as i32)))
.sum::<f64>()
        / rows.len() as f64
}

struct AgyFullrunLagrangianSchedule {
    bases: Vec<Vec<u16>>,
    lagrangian_total: usize,
    requested_limit: usize,
    lagrangian_scope: &'static str,
    feasibility_note: &'static str,
    profile_ceiling: bool,
}

fn agy_fullrun_lagrangian_schedule(
    n: usize,
    n4_lagrangian_limit: usize,
    n5_lagrangian_limit: usize,
    n6_lagrangian_limit: usize,
) -> AgyFullrunLagrangianSchedule {
    let lagrangian_total = lagrangian_count_formula(n);
    match n {
        2 | 3 => {
            let bases = exact_lagrangian_bases(n);
            AgyFullrunLagrangianSchedule {
                bases,
                lagrangian_total,
                requested_limit: 0,
                lagrangian_scope: "all_lagrangians",
                feasibility_note: "exact all-Lagrangians cell required by the full-run handoff",
                profile_ceiling: false,
            }
        }
        4 => {
            let all_bases = exact_lagrangian_bases(n);
            let selected = if n4_lagrangian_limit == 0 || n4_lagrangian_limit >= all_bases.len() {
                all_bases
            } else {
                evenly_spaced_bases(&all_bases, n4_lagrangian_limit)
            };
            let exact = selected.len() == lagrangian_total;
            AgyFullrunLagrangianSchedule {
                bases: selected,
                lagrangian_total,
                requested_limit: n4_lagrangian_limit,
                lagrangian_scope: if exact {
                    "all_lagrangians"
                } else {
                    "even_spaced_sample"
                },
                feasibility_note: if exact {
                    "exact n=4 all-Lagrangians cell"
                } else {
                    "deterministic evenly spaced n=4 sample from the exact Lagrangian list"
                },
                profile_ceiling: false,
            }
        }
        5 => {
            let sample_count = n5_lagrangian_limit.max(1);
            AgyFullrunLagrangianSchedule {
                bases: deterministic_lagrangian_basis_sample(n, sample_count),
                lagrangian_total,
                requested_limit: n5_lagrangian_limit,
                lagrangian_scope: "deterministic_basis_sample",
                feasibility_note: "deterministic n=5 isotropic-basis sample; full exact enumeration is not attempted",
                profile_ceiling: false,
            }
        }
        6 => AgyFullrunLagrangianSchedule {
            bases: Vec::new(),
            lagrangian_total,
            requested_limit: n6_lagrangian_limit,
            lagrangian_scope: "profile_ceiling",
            feasibility_note: "profile ceiling: n=6 has m=24 and a 2^24 ambient profile per B/Lagrangian, so this Rust audit row reports the ceiling instead of extrapolating",
            profile_ceiling: true,
        },
        _ => AgyFullrunLagrangianSchedule {
            bases: Vec::new(),
            lagrangian_total,
            requested_limit: 0,
            lagrangian_scope: "unsupported_n",
            feasibility_note: "unsupported n for this full-run audit schedule",
            profile_ceiling: true,
        },
    }
}

fn exact_lagrangian_bases(n: usize) -> Vec<Vec<u16>> {
    conj_m2_lagrangian_span_masks(n)
.iter()
.map(|span| canonical_basis(span, n))
.collect()
}

fn evenly_spaced_bases(all_bases: &[Vec<u16>], limit: usize) -> Vec<Vec<u16>> {
    if limit >= all_bases.len() {
        return all_bases.to_vec();
    }
    if limit <= 1 {
        return vec![all_bases[0].clone()];
    }
    let last = all_bases.len() - 1;
    (0..limit)
.map(|idx| all_bases[(idx * last) / (limit - 1)].clone())
.collect()
}

fn lagrangian_count_formula(n: usize) -> usize {
    (1..=n).map(|idx| (1usize << idx) + 1).product()
}

fn deterministic_lagrangian_basis_sample(n: usize, count: usize) -> Vec<Vec<u16>> {
    let mut state = 0x9e37_79b9_7f4a_7c15u64 ^ ((n as u64) << 32) ^ count as u64;
    let mut samples = Vec::new();
    let mut seen = HashSet::new();
    let mut attempts = 0usize;
    while samples.len() < count && attempts < count * 10_000 {
        attempts += 1;
        if let Some(basis) = deterministic_lagrangian_basis(n, &mut state) {
            let key = lagrangian_basis_span_key(&basis, n);
            if seen.insert(key) {
                samples.push(basis);
            }
        }
    }
    assert_eq!(
        samples.len(),
        count,
        "deterministic Lagrangian sampler did not find enough samples"
);
    samples
}

fn deterministic_lagrangian_basis(n: usize, state: &mut u64) -> Option<Vec<u16>> {
    let ambient = 1u16 << (2 * n);
    let mut basis = Vec::with_capacity(n);
    let mut span = HashSet::new();
    span.insert(0u16);
    for _ in 0..n {
        let mut chosen = None;
        for _attempt in 0..(ambient as usize * 8) {
            let candidate = 1 + (next_lcg(state) % (ambient as u64 - 1)) as u16;
            if span.contains(&candidate) {
                continue;
            }
            if basis
.iter()
.any(|&basis_vec| symplectic_pairing_bits(candidate, basis_vec, n) != 0)
            {
                continue;
            }
            chosen = Some(candidate);
            break;
        }
        let candidate = chosen?;
        let current: Vec<_> = span.iter().copied().collect();
        for item in current {
            span.insert(item ^ candidate);
        }
        basis.push(candidate);
    }
    Some(basis)
}

fn lagrangian_basis_span_key(basis: &[u16], n: usize) -> Vec<u16> {
    let mut span = Vec::with_capacity(1usize << n);
    for coeffs in 0..(1u16 << n) {
        span.push(linear_combination(basis, coeffs));
    }
    span.sort_unstable();
    span
}

fn next_lcg(state: &mut u64) -> u64 {
    *state = state
.wrapping_mul(6364136223846793005)
.wrapping_add(1442695040888963407);
    *state
}

fn symplectic_pairing_bits(u: u16, v: u16, n: usize) -> u8 {
    let mut acc = 0u8;
    for idx in 0..n {
        let ux = ((u >> idx) & 1) as u8;
        let vx = ((v >> idx) & 1) as u8;
        let uz = ((u >> (idx + n)) & 1) as u8;
        let vz = ((v >> (idx + n)) & 1) as u8;
        acc ^= (ux & vz) ^ (uz & vx);
    }
    acc
}

fn agy_pushforward_row(
    n: usize,
    b_model: &'static str,
    b_description: &'static str,
    b_rows: &[u16],
    selected_bases: &[Vec<u16>],
    lagrangian_total: usize,
    max_lagrangians_per_n: usize,
    lagrangian_scope: &'static str,
    feasibility_note: &'static str,
) -> ConjM2AgyPushforwardRow {
    let c = 4usize;
    let m = c * n;
    assert_eq!(b_rows.len(), m, " pushforward B must have m rows");
    assert!(m <= 24, " pushforward profile stores syndromes in u32");
    let p_lpn = delta_gv_for_rate(1.0 / c as f64);
    let hamming_weights = hamming_weight_table(m);
    let columns = b_column_vectors(2 * n, b_rows);
    let rank_b = gf2_rank_u32(&columns);
    let col_b = b_column_space(n, b_rows);
    let lagrangians_evaluated = selected_bases.len();
    let lagrangian_probability = 1.0 / lagrangians_evaluated as f64;
    let full_probability = 1.0 / (1usize << m) as f64;
    let col_b_probability = 1.0 / col_b.len() as f64;
    let mut structured = vec![0.0; m + 1];
    let mut lpn = vec![0.0; m + 1];
    let mut full = vec![0.0; m + 1];
    let mut col = vec![0.0; m + 1];

    for basis in selected_bases {
        let codewords = b_times_lagrangian_codewords(n, b_rows, &basis);
        let profile = coset_leader_weight_profile(m, &codewords, &hamming_weights);

        for z in 0..(1usize << m) {
            let t = profile[z] as usize;
            full[t] += lagrangian_probability * full_probability;
            lpn[t] += lagrangian_probability * bernoulli_vector_probability_usize(z, m, p_lpn);
        }
        for &z in &col_b {
            let t = profile[z as usize] as usize;
            col[t] += lagrangian_probability * col_b_probability;
        }
        for e in 0..(1usize << (2 * n)) {
            let syndrome = b_times_vector_syndrome(n, b_rows, e as u16) as usize;
            let t = profile[syndrome] as usize;
            structured[t] +=
                lagrangian_probability * bernoulli_vector_probability_usize(e, 2 * n, 0.25);
        }
    }

    let structured_metrics = t_distribution_metrics(&structured);
    let lpn_metrics = t_distribution_metrics(&lpn);
    let full_metrics = t_distribution_metrics(&full);
    let col_metrics = t_distribution_metrics(&col);
    let bernoulli_ratio = structured_metrics.variance / lpn_metrics.variance;
    let pushforward_ratio = col_metrics.variance / full_metrics.variance;
    ConjM2AgyPushforwardRow {
        b_model,
        b_description,
        c,
        n,
        m,
        max_lagrangians_per_n,
        lagrangian_scope,
        lagrangians_evaluated,
        lagrangian_total,
        all_lagrangians_exact: lagrangians_evaluated == lagrangian_total,
        feasibility_note,
        rank_b,
        col_b_size: col_b.len(),
        var_t_struct: structured_metrics.variance,
        var_t_lpn: lpn_metrics.variance,
        bernoulli_ratio,
        var_full: full_metrics.variance,
        var_col_b: col_metrics.variance,
        pushforward_ratio,
        push_over_bernoulli: pushforward_ratio / bernoulli_ratio,
        status: "OPEN",
        claim_level: "EVIDENCE",
        reduction_claim: false,
        security_claim: false,
    }
}

fn agy_pushforward_ceiling_row(
    n: usize,
    b_model: &'static str,
    b_description: &'static str,
    b_rows: &[u16],
    lagrangian_total: usize,
    max_lagrangians_per_n: usize,
    lagrangian_scope: &'static str,
    feasibility_note: &'static str,
) -> ConjM2AgyPushforwardRow {
    let c = 4usize;
    let columns = b_column_vectors(2 * n, b_rows);
    let rank_b = gf2_rank_u32(&columns);
    let col_b = b_column_space(n, b_rows);
    ConjM2AgyPushforwardRow {
        b_model,
        b_description,
        c,
        n,
        m: c * n,
        max_lagrangians_per_n,
        lagrangian_scope,
        lagrangians_evaluated: 0,
        lagrangian_total,
        all_lagrangians_exact: false,
        feasibility_note,
        rank_b,
        col_b_size: col_b.len(),
        var_t_struct: f64::NAN,
        var_t_lpn: f64::NAN,
        bernoulli_ratio: f64::NAN,
        var_full: f64::NAN,
        var_col_b: f64::NAN,
        pushforward_ratio: f64::NAN,
        push_over_bernoulli: f64::NAN,
        status: "OPEN",
        claim_level: "EVIDENCE",
        reduction_claim: false,
        security_claim: false,
    }
}

fn circulant_weight_two_rows(width: usize, gaps: &[usize]) -> Vec<u16> {
    let mut rows = Vec::with_capacity(width * gaps.len());
    for &gap in gaps {
        let gap = gap % width;
        assert!(gap != 0, "circulant gap must be nonzero modulo width");
        for left in 0..width {
            let right = (left + gap) % width;
            rows.push((1u16 << left) | (1u16 << right));
        }
    }
    rows
}

fn b_column_vectors(width: usize, b_rows: &[u16]) -> Vec<u32> {
    (0..width)
.map(|col| {
            let mut column = 0u32;
            for (row_idx, &row) in b_rows.iter().enumerate() {
                if ((row >> col) & 1) != 0 {
                    column |= 1u32 << row_idx;
                }
            }
            column
        })
.collect()
}

fn b_column_space(n: usize, b_rows: &[u16]) -> Vec<u32> {
    let mut syndromes = Vec::with_capacity(1usize << (2 * n));
    for mask in 0..(1usize << (2 * n)) {
        syndromes.push(b_times_vector_syndrome(n, b_rows, mask as u16));
    }
    syndromes.sort_unstable();
    syndromes.dedup();
    syndromes
}

fn gf2_rank_u32(vectors: &[u32]) -> usize {
    let mut basis = [0u32; 32];
    let mut rank = 0usize;
    for &vector in vectors {
        let mut row = vector;
        while row != 0 {
            let pivot = 31usize - row.leading_zeros() as usize;
            if basis[pivot] == 0 {
                basis[pivot] = row;
                rank += 1;
                break;
            }
            row ^= basis[pivot];
        }
    }
    rank
}

fn b_times_lagrangian_codewords(n: usize, b_rows: &[u16], basis: &[u16]) -> Vec<u32> {
    let mut columns = Vec::with_capacity(basis.len());
    for &basis_vector in basis {
        columns.push(b_times_vector_syndrome(
            n,
            b_rows,
            interleaved_xz_bits(basis_vector, n),
));
    }
    (0..(1usize << basis.len()))
.map(|coeffs| {
            let mut codeword = 0u32;
            for (idx, &column) in columns.iter().enumerate() {
                if ((coeffs >> idx) & 1) == 1 {
                    codeword ^= column;
                }
            }
            codeword
        })
.collect()
}

fn b_times_vector_syndrome(n: usize, b_rows: &[u16], vector: u16) -> u32 {
    let mut syndrome = 0u32;
    for (idx, &row) in b_rows.iter().enumerate() {
        if gf2_dot(row, vector, 2 * n) != 0 {
            syndrome |= 1u32 << idx;
        }
    }
    syndrome
}

fn interleaved_xz_bits(vector: u16, n: usize) -> u16 {
    let mut interleaved = 0u16;
    for idx in 0..n {
        if ((vector >> idx) & 1) != 0 {
            interleaved |= 1u16 << (2 * idx);
        }
        if ((vector >> (idx + n)) & 1) != 0 {
            interleaved |= 1u16 << (2 * idx + 1);
        }
    }
    interleaved
}

fn gf2_dot(left: u16, right: u16, width: usize) -> u8 {
    let mask = (1u16 << width) - 1;
    ((left & right & mask).count_ones() as u8) & 1
}

fn coset_leader_weight_profile(m: usize, codewords: &[u32], hamming_weights: &[u8]) -> Vec<u8> {
    let syndrome_count = 1usize << m;
    let mut profile = vec![m as u8 + 1; syndrome_count];
    for &codeword in codewords {
        let codeword = codeword as usize;
        for syndrome in 0..syndrome_count {
            let candidate = hamming_weights[syndrome ^ codeword];
            if candidate < profile[syndrome] {
                profile[syndrome] = candidate;
            }
        }
    }
    profile
}

fn hamming_weight_table(width: usize) -> Vec<u8> {
    (0..(1usize << width))
.map(|vector| vector.count_ones() as u8)
.collect()
}

fn bernoulli_vector_probability_usize(vector: usize, width: usize, p: f64) -> f64 {
    let mask = (1usize << width) - 1;
    let weight = (vector & mask).count_ones() as i32;
    p.powi(weight) * (1.0 - p).powi(width as i32 - weight)
}

fn w_structured_weight2(n: usize, basis: &[u16], v: u16) -> Vec<f64> {
    let pattern_count = 1usize << (n + 1);
    let mut w = vec![0.0; pattern_count];
    let rows = weight_two_rows(2 * n);
    let row_probability = 1.0 / rows.len() as f64;
    for row in rows {
        let mut pattern = 0usize;
        for (j, &a_j) in basis.iter().enumerate() {
            if conj_m2_symplectic_pairing(row, a_j, n) != 0 {
                pattern |= 1usize << (n - j);
            }
        }
        if conj_m2_symplectic_pairing(row, v, n) != 0 {
            pattern |= 1;
        }
        w[pattern] += row_probability;
    }
    w
}

fn weight_two_rows(width: usize) -> Vec<u16> {
    let mut rows = Vec::new();
    for left in 0..width {
        for right in (left + 1)..width {
            rows.push((1u16 << left) | (1u16 << right));
        }
    }
    rows
}

pub fn conj_m2_n3_minweight_probe_report(max_m: usize) -> Vec<ConjM2MinWeightGateRow> {
    (4..=max_m)
.map(|m| conj_m2_minweight_best_case_row(3, m))
.collect()
}

fn conj_m2_overdispersion_row(
    n: usize,
    m: usize,
    p_b: f64,
    p_prime: f64,
) -> ConjM2OverdispersionRow {
    let (structured, _) = structured_t_dist(n, m, p_b, 0.25);
    let (lpn, _) = t_dist_from_w(&w_lpn(n, p_prime), n, m);
    let distribution: Vec<_> = structured
.iter()
.zip(lpn.iter())
.enumerate()
.map(|(t, (structured_probability, lpn_probability))| {
            let signed_delta = structured_probability - lpn_probability;
            ConjM2TDistributionPoint {
                t,
                structured_probability: *structured_probability,
                lpn_probability: *lpn_probability,
                signed_delta,
                sd_contribution: 0.5 * signed_delta.abs(),
            }
        })
.collect();
    ConjM2OverdispersionRow {
        n,
        m,
        p_b,
        p_prime,
        sd1d: sd1d(&structured, &lpn),
        structured_metrics: t_distribution_metrics(&structured),
        lpn_metrics: t_distribution_metrics(&lpn),
        distribution,
        status: "OPEN",
        claim_level: "EVIDENCE",
        reduction_claim: false,
        security_claim: false,
    }
}

fn t_distribution_metrics(dist: &[f64]) -> ConjM2TDistributionMetrics {
    let mean = dist
.iter()
.enumerate()
.map(|(t, probability)| t as f64 * probability)
.sum::<f64>();
    let variance = dist
.iter()
.enumerate()
.map(|(t, probability)| {
            let centered = t as f64 - mean;
            centered * centered * probability
        })
.sum::<f64>();
    let upper_tail_threshold = (mean + variance.sqrt()).ceil() as usize;
    let upper_tail_mass = dist
.iter()
.enumerate()
.filter(|(t, _)| *t >= upper_tail_threshold)
.map(|(_, probability)| probability)
.sum::<f64>();
    let tail_ge_2_mass = dist.iter().skip(2).sum::<f64>();
    let saturation_t = dist
.iter()
.rposition(|probability| *probability > 1e-12)
.unwrap_or(0);
    ConjM2TDistributionMetrics {
        mean,
        variance,
        upper_tail_threshold,
        upper_tail_mass,
        tail_ge_2_mass,
        saturation_t,
        saturation_mass: dist[saturation_t],
    }
}

fn overdispersion_row_json(row: &ConjM2OverdispersionRow) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        concat!(
            "    {{\"n\": {}, \"m\": {}, \"p_b\": {:.12}, \"p_prime\": {:.12}, ",
            "\"sd1d\": {:.12}, \"status\": \"{}\", \"claim_level\": \"{}\", ",
            "\"reduction_claim\": {}, \"security_claim\": {}, "
),
        row.n,
        row.m,
        row.p_b,
        row.p_prime,
        row.sd1d,
        row.status,
        row.claim_level,
        row.reduction_claim,
        row.security_claim
));
    out.push_str(&format!(
        "\"structured_metrics\": {}, \"lpn_metrics\": {}, \"distribution\": [",
        t_metrics_json(&row.structured_metrics),
        t_metrics_json(&row.lpn_metrics)
));
    for (idx, point) in row.distribution.iter().enumerate() {
        let comma = if idx + 1 == row.distribution.len() {
            ""
        } else {
            ", "
        };
        out.push_str(&format!(
            concat!(
                "{{\"t\": {}, \"structured_probability\": {:.12}, ",
                "\"lpn_probability\": {:.12}, \"signed_delta\": {:.12}, ",
                "\"sd_contribution\": {:.12}}}{}"
),
            point.t,
            point.structured_probability,
            point.lpn_probability,
            point.signed_delta,
            point.sd_contribution,
            comma
));
    }
    out.push_str("]}");
    out
}

fn t_metrics_json(metrics: &ConjM2TDistributionMetrics) -> String {
    format!(
        concat!(
            "{{\"mean\": {:.12}, \"variance\": {:.12}, ",
            "\"upper_tail_threshold\": {}, \"upper_tail_mass\": {:.12}, ",
            "\"tail_ge_2_mass\": {:.12}, ",
            "\"saturation_t\": {}, \"saturation_mass\": {:.12}}}"
),
        metrics.mean,
        metrics.variance,
        metrics.upper_tail_threshold,
        metrics.upper_tail_mass,
        metrics.tail_ge_2_mass,
        metrics.saturation_t,
        metrics.saturation_mass
)
}

pub fn conj_m2_n3_minweight_probe_report_json(max_m: usize) -> String {
    let rows = conj_m2_n3_minweight_probe_report(max_m);
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("  \"experiment\": \"-conj-m2-n3-minweight-symmetry-dp\",\n");
    out.push_str("  \"target\": \"conj:m2 n=3 min-syndrome-weight symmetry-reduced DP\",\n");
    out.push_str("  \"method\": \"law-preserving affine-coset profile canonicalization\",\n");
    out.push_str("  \"gate\": \"n=2 reproduced before n=3 trust\",\n");
    out.push_str(&format!("  \"max_m_requested\": {max_m},\n"));
    out.push_str("  \"status\": \"OPEN\",\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str("  \"rows\": [\n");
    for (idx, row) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        out.push_str(&format!(
            concat!(
                "    {{\"n\": {}, \"m\": {}, \"best_advantage\": {:.12}, ",
                "\"best_p_b\": {:.12}, \"best_p_prime\": {:.12}, ",
                "\"reachable_state_count\": {}}}{}\n"
),
            row.n,
            row.m,
            row.best_advantage,
            row.best_p_b,
            row.best_p_prime,
            row.reachable_state_count,
            comma
));
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

pub fn conj_m2_affine_coset_permutation_count(n: usize) -> usize {
    affine_coset_permutations(n).len()
}

pub fn conj_m2_lpn_preserving_affine_coset_symmetry_count(n: usize, p_prime: f64) -> usize {
    preserving_affine_coset_permutations(&w_lpn(n, p_prime), n).len()
}

pub fn conj_m2_minweight_best_case_row(n: usize, m: usize) -> ConjM2MinWeightGateRow {
    let p_b_grid = [0.125, 1.0 / 6.0, 0.25, 1.0 / 3.0, 0.5];
    let p_prime_grid: Vec<_> = (5..50).step_by(2).map(|i| i as f64 / 100.0).collect();
    let lpn_t: Vec<_> = p_prime_grid
.iter()
.map(|&p_prime| (p_prime, t_dist_from_w(&w_lpn(n, p_prime), n, m).0))
.collect();

    let mut best = ConjM2MinWeightGateRow {
        n,
        m,
        best_advantage: f64::INFINITY,
        best_p_b: 0.0,
        best_p_prime: 0.0,
        reachable_state_count: 0,
        status: "OPEN",
        reduction_claim: false,
        security_claim: false,
    };

    for &p_b in &p_b_grid {
        let (structured, state_count) = structured_t_dist(n, m, p_b, 0.25);
        for (p_prime, lpn) in &lpn_t {
            let adv = sd1d(&structured, lpn);
            if adv < best.best_advantage {
                best.best_advantage = adv;
                best.best_p_b = p_b;
                best.best_p_prime = *p_prime;
                best.reachable_state_count = state_count;
            }
        }
    }

    best
}

fn structured_t_dist(n: usize, m: usize, p_b: f64, p_source: f64) -> (Vec<f64>, usize) {
    let mixture = structured_w_mixture(n, p_b, p_source);
    let mut cache: HashMap<Vec<u64>, (Vec<f64>, usize)> = HashMap::new();
    let mut mix = vec![0.0; m + 1];
    let mut max_states = 0usize;

    for (key, w, pr) in mixture {
        let (tdist, states) = cache
.entry(key.clone())
.or_insert_with(|| t_dist_from_w(&w, n, m));
        max_states = max_states.max(*states);
        for (dst, src) in mix.iter_mut().zip(tdist.iter()) {
            *dst += pr * src;
        }
    }

    (mix, max_states)
}

fn structured_w_mixture(n: usize, p_b: f64, p_source: f64) -> Vec<(Vec<u64>, Vec<f64>, f64)> {
    let prior = structured_prior(n, p_source);
    let mut grouped: HashMap<Vec<u64>, (Vec<f64>, f64)> = HashMap::new();

    for (basis, v, pr) in prior {
        let w = w_structured(n, &basis, v, p_b);
        let key = canonical_w_key(&w, n);
        let entry = grouped.entry(key.clone()).or_insert((w, 0.0));
        entry.1 += pr;
    }

    grouped
.into_iter()
.map(|(key, (w, pr))| (key, w, pr))
.collect()
}

fn structured_prior(n: usize, p_source: f64) -> Vec<(Vec<u16>, u16, f64)> {
    let spans = conj_m2_lagrangian_span_masks(n);
    let lagrangian_count = spans.len() as f64;
    let x_count = (1usize << n) as f64;
    let ambient = 1u16 << (2 * n);
    let mut prior: HashMap<(Vec<u16>, u16), f64> = HashMap::new();

    for span in spans {
        let basis = canonical_basis(&span, n);
        for x in 0..(1u16 << n) {
            let ax = linear_combination(&basis, x);
            for e in 0..ambient {
                let v = ax ^ e;
                let pe = bernoulli_vector_probability(e, 2 * n, p_source);
                *prior.entry((basis.clone(), v)).or_insert(0.0) +=
                    pe / (lagrangian_count * x_count);
            }
        }
    }

    prior
.into_iter()
.map(|((basis, v), pr)| (basis, v, pr))
.collect()
}

fn canonical_basis(span: &[u64; 4], n: usize) -> Vec<u16> {
    let mut basis = Vec::with_capacity(n);
    let mut generated = HashSet::new();
    generated.insert(0u16);

    for v in conj_m2_span_mask_vectors(span) {
        if v == 0 || generated.contains(&v) {
            continue;
        }
        let current: Vec<_> = generated.iter().copied().collect();
        for s in current {
            generated.insert(s ^ v);
        }
        basis.push(v);
        if basis.len() == n {
            break;
        }
    }

    assert_eq!(basis.len(), n);
    assert_eq!(generated.len(), 1usize << n);
    for v in generated {
        assert!(conj_m2_span_mask_contains(span, v));
    }
    basis
}

fn linear_combination(basis: &[u16], coeffs: u16) -> u16 {
    let mut acc = 0u16;
    for (i, &b) in basis.iter().enumerate() {
        if ((coeffs >> i) & 1) == 1 {
            acc ^= b;
        }
    }
    acc
}

fn w_structured(n: usize, basis: &[u16], v: u16, p_b: f64) -> Vec<f64> {
    let pattern_count = 1usize << (n + 1);
    let mut w = vec![0.0; pattern_count];
    for row in 0..(1u16 << (2 * n)) {
        let pr = bernoulli_vector_probability(row, 2 * n, p_b);
        let mut pattern = 0usize;
        for (j, &a_j) in basis.iter().enumerate() {
            if conj_m2_symplectic_pairing(row, a_j, n) != 0 {
                pattern |= 1usize << (n - j);
            }
        }
        if conj_m2_symplectic_pairing(row, v, n) != 0 {
            pattern |= 1;
        }
        w[pattern] += pr;
    }
    w
}

fn w_lpn(n: usize, p_prime: f64) -> Vec<f64> {
    let pattern_count = 1usize << (n + 1);
    let mut w = vec![0.0; pattern_count];
    let col_probability = 1.0 / (1usize << n) as f64;
    for s in 0..(1usize << n) {
        w[s << 1] = col_probability * (1.0 - p_prime);
        w[(s << 1) | 1] = col_probability * p_prime;
    }
    w
}

fn t_dist_from_w(w: &[f64], n: usize, m: usize) -> (Vec<f64>, usize) {
    if n >= 4 && m <= 3 {
        return t_dist_from_w_no_symmetry(w, n, m);
    }
    let cosets = 1usize << n;
    let profile_group = preserving_affine_coset_permutations(w, n);
    let patterns: Vec<_> = w
.iter()
.enumerate()
.filter_map(|(pattern, &probability)| {
            if probability == 0.0 {
                None
            } else {
                Some((pattern, probability, deltas_for_pattern(pattern, n)))
            }
        })
.collect();
    let mut canonical_cache = HashMap::new();
    let mut states = HashMap::new();
    states.insert(
        canonical_profile_cached(vec![0u8; cosets], &profile_group, &mut canonical_cache),
        1.0f64,
);
    let mut max_states = states.len();

    for _ in 0..m {
        let mut next = HashMap::new();
        for (profile, mass) in states {
            for &(_, probability, ref deltas) in &patterns {
                let mut expanded = profile.clone();
                for (dst, delta) in expanded.iter_mut().zip(deltas.iter()) {
                    *dst += *delta;
                }
                let canonical =
                    canonical_profile_cached(expanded, &profile_group, &mut canonical_cache);
                *next.entry(canonical).or_insert(0.0) += mass * probability;
            }
        }
        max_states = max_states.max(next.len());
        states = next;
    }

    let mut dist = vec![0.0; m + 1];
    for (profile, mass) in states {
        let min_weight = profile.iter().copied().min().expect("profile is non-empty") as usize;
        dist[min_weight] += mass;
    }
    (dist, max_states)
}

fn t_dist_from_w_no_symmetry(w: &[f64], n: usize, m: usize) -> (Vec<f64>, usize) {
    let cosets = 1usize << n;
    let patterns: Vec<_> = w
.iter()
.enumerate()
.filter_map(|(pattern, &probability)| {
            if probability == 0.0 {
                None
            } else {
                Some((probability, deltas_for_pattern(pattern, n)))
            }
        })
.collect();
    let mut states = HashMap::new();
    states.insert(vec![0u8; cosets], 1.0f64);
    let mut max_states = states.len();

    for _ in 0..m {
        let mut next = HashMap::new();
        for (profile, mass) in states {
            for (probability, deltas) in &patterns {
                let mut expanded = profile.clone();
                for (dst, delta) in expanded.iter_mut().zip(deltas.iter()) {
                    *dst += *delta;
                }
                *next.entry(expanded).or_insert(0.0) += mass * probability;
            }
        }
        max_states = max_states.max(next.len());
        states = next;
    }

    let mut dist = vec![0.0; m + 1];
    for (profile, mass) in states {
        let min_weight = profile.iter().copied().min().expect("profile is non-empty") as usize;
        dist[min_weight] += mass;
    }
    (dist, max_states)
}

fn preserving_affine_coset_permutations(w: &[f64], n: usize) -> Vec<Vec<usize>> {
    affine_coset_permutations(n)
.into_iter()
.filter(|perm| pattern_law_is_preserved_by_coset_permutation(w, n, perm))
.collect()
}

fn affine_coset_permutations(n: usize) -> Vec<Vec<usize>> {
    let linear_maps = invertible_column_maps(n);
    let mut perms = Vec::with_capacity(linear_maps.len() * (1usize << n));
    for images in linear_maps {
        for translation in 0..(1u16 << n) {
            let mut perm = vec![0usize; 1usize << n];
            for old in 0..(1u16 << n) {
                perm[old as usize] = (linear_combination(&images, old) ^ translation) as usize;
            }
            perms.push(perm);
        }
    }
    perms
}

fn pattern_law_is_preserved_by_coset_permutation(w: &[f64], n: usize, perm: &[usize]) -> bool {
    for pattern in 0..w.len() {
        let mapped_pattern = map_pattern_by_coset_permutation(pattern, n, perm);
        if (w[pattern] - w[mapped_pattern]).abs() > 1e-12 {
            return false;
        }
    }
    true
}

fn map_pattern_by_coset_permutation(pattern: usize, n: usize, perm: &[usize]) -> usize {
    let deltas = deltas_for_pattern(pattern, n);
    let mut mapped_deltas = vec![0u8; deltas.len()];
    for (old, &new) in perm.iter().enumerate() {
        mapped_deltas[new] = deltas[old];
    }
    (0..(1usize << (n + 1)))
.find(|candidate| deltas_for_pattern(*candidate, n) == mapped_deltas)
.expect("affine coset permutation must map linear delta patterns to linear delta patterns")
}

fn canonical_profile_cached(
    profile: Vec<u8>,
    group: &[Vec<usize>],
    cache: &mut HashMap<Vec<u8>, Vec<u8>>,
) -> Vec<u8> {
    if group.len() <= 1 {
        return profile;
    }
    if let Some(canonical) = cache.get(&profile) {
        return canonical.clone();
    }
    let canonical = canonical_profile(profile.clone(), group);
    cache.insert(profile, canonical.clone());
    canonical
}

fn canonical_profile(profile: Vec<u8>, group: &[Vec<usize>]) -> Vec<u8> {
    if group.len() <= 1 {
        return profile;
    }
    group
.iter()
.map(|perm| {
            let mut mapped = vec![0u8; profile.len()];
            for (old, &new) in perm.iter().enumerate() {
                mapped[new] = profile[old];
            }
            mapped
        })
.min()
.expect("profile group is non-empty")
}

fn deltas_for_pattern(pattern: usize, n: usize) -> Vec<u8> {
    let y = (pattern & 1) as u8;
    let columns = pattern >> 1;
    (0..(1usize << n))
.map(|w| (y ^ ((columns & w).count_ones() as u8 & 1)) as u8)
.collect()
}

fn bernoulli_vector_probability(vector: u16, width: usize, p: f64) -> f64 {
    let weight = (vector & ((1u16 << width) - 1)).count_ones() as i32;
    p.powi(weight) * (1.0 - p).powi(width as i32 - weight)
}

fn quantized_w_key(w: &[f64]) -> Vec<u64> {
    w.iter()
.map(|x| (x * 1_000_000_000_000.0).round() as u64)
.collect()
}

fn canonical_w_key(w: &[f64], n: usize) -> Vec<u64> {
    if n >= 4 {
        return quantized_w_key(w);
    }
    invertible_column_maps(n)
.into_iter()
.map(|images| quantized_w_key(&map_w_columns(w, n, &images)))
.min()
.expect("at least identity map exists")
}

fn map_w_columns(w: &[f64], n: usize, images: &[u16]) -> Vec<f64> {
    let mut mapped = vec![0.0; w.len()];
    for s in 0..(1usize << n) {
        let mapped_s = linear_combination(images, s as u16) as usize;
        for y in 0..2usize {
            mapped[(mapped_s << 1) | y] += w[(s << 1) | y];
        }
    }
    mapped
}

fn invertible_column_maps(n: usize) -> Vec<Vec<u16>> {
    let mut maps = Vec::new();
    let nonzero: Vec<u16> = (1..(1u16 << n)).collect();
    extend_invertible_column_map(n, &nonzero, &mut Vec::new(), &mut maps);
    maps
}

fn extend_invertible_column_map(
    n: usize,
    candidates: &[u16],
    current: &mut Vec<u16>,
    maps: &mut Vec<Vec<u16>>,
) {
    if current.len() == n {
        maps.push(current.clone());
        return;
    }
    let span = span_set(current);
    for &candidate in candidates {
        if span.contains(&candidate) {
            continue;
        }
        current.push(candidate);
        extend_invertible_column_map(n, candidates, current, maps);
        current.pop();
    }
}

fn span_set(basis: &[u16]) -> HashSet<u16> {
    let mut span = HashSet::new();
    span.insert(0);
    for &v in basis {
        let current: Vec<_> = span.iter().copied().collect();
        for s in current {
            span.insert(s ^ v);
        }
    }
    span
}

fn sd1d(p: &[f64], q: &[f64]) -> f64 {
    0.5 * p
.iter()
.zip(q.iter())
.map(|(left, right)| (left - right).abs())
.sum::<f64>()
}
