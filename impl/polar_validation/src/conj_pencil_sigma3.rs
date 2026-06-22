// Copyright 2026 Kwanghoo Choo
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct ConjPencilSigma3IncidenceReport {
    pub n: usize,
    pub lagrangian_count: usize,
    pub isotropic_three_space_count: usize,
    pub three_spaces_per_lagrangian: usize,
    pub lagrangians_per_three_space: usize,
    pub status: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

#[derive(Clone, Debug)]
pub struct ConjPencilSigma3SubsetReport {
    pub n: usize,
    pub subset_size: usize,
    pub sigma3: u64,
    pub sigma3_over_m_squared: f64,
    pub construction: &'static str,
    pub status: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

#[derive(Clone, Debug)]
pub struct ConjPencilSigma3FullPencilGate {
    pub n: usize,
    pub pencil_size: usize,
    pub sigma3_over_pencil_size_squared: f64,
    pub expected_gate_value: f64,
    pub gate_passed: bool,
    pub status: &'static str,
    pub claim_level: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

#[derive(Clone, Debug)]
pub struct ConjPencilSigma3MaxSearchRow {
    pub n: usize,
    pub subset_size: usize,
    pub inspected_candidate_count: usize,
    pub full_pencil_ratio: f64,
    pub best_ratio: f64,
    pub best_construction: &'static str,
    pub search_kind: &'static str,
    pub exceeded_full_pencil: bool,
    pub status: &'static str,
    pub claim_level: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

#[derive(Clone, Debug)]
pub struct ConjPencilSigma3V2SearchRow {
    pub n: usize,
    pub subset_size: usize,
    pub inspected_candidate_count: usize,
    pub restart_count: usize,
    pub full_pencil_ratio: f64,
    pub best_ratio: f64,
    pub best_construction: &'static str,
    pub search_kind: &'static str,
    pub exceeded_full_pencil: bool,
    pub status: &'static str,
    pub claim_level: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

#[derive(Clone, Debug)]
pub struct ConjPencilSigma3V2SpecialConfigRow {
    pub n: usize,
    pub subset_size: usize,
    pub full_pencil_ratio: f64,
    pub config_ratio: f64,
    pub construction: &'static str,
    pub exceeded_full_pencil: bool,
    pub status: &'static str,
    pub claim_level: &'static str,
    pub reduction_claim: bool,
    pub security_claim: bool,
}

pub fn conj_pencil_sigma3_incidence_report(n: usize) -> ConjPencilSigma3IncidenceReport {
    let lagrangians = enumerate_lagrangians(n);
    let three_spaces = enumerate_isotropic_subspace_keys(n, 3);
    let three_spaces_per_lagrangian = subspace_keys_inside_basis(&lagrangians[0], 3).len();
    let lagrangians_per_three_space = if three_spaces.is_empty() {
        0
    } else {
        lagrangians.len() * three_spaces_per_lagrangian / three_spaces.len()
    };
    ConjPencilSigma3IncidenceReport {
        n,
        lagrangian_count: lagrangians.len(),
        isotropic_three_space_count: three_spaces.len(),
        three_spaces_per_lagrangian,
        lagrangians_per_three_space,
        status: "OPEN",
        reduction_claim: false,
        security_claim: false,
    }
}

pub fn conj_pencil_sigma3_n3_sanity_report(subset_size: usize) -> ConjPencilSigma3SubsetReport {
    let lagrangians = enumerate_lagrangians(3);
    let subset_size = subset_size.min(lagrangians.len());
    let selected: Vec<_> = lagrangians.iter().take(subset_size).cloned().collect();
    let sigma3 = sigma3_for_lagrangian_subset(&selected);
    ConjPencilSigma3SubsetReport {
        n: 3,
        subset_size,
        sigma3,
        sigma3_over_m_squared: sigma3 as f64 / (subset_size * subset_size) as f64,
        construction: "n=3 sanity: 3-space equals Lagrangian",
        status: "OPEN",
        reduction_claim: false,
        security_claim: false,
    }
}

pub fn conj_pencil_sigma3_pencil_report(
    n: usize,
    core_dimension: usize,
    subset_limit: Option<usize>,
) -> ConjPencilSigma3SubsetReport {
    let mut selected = lagrangians_containing_standard_core(n, core_dimension);
    if let Some(limit) = subset_limit {
        selected.truncate(limit);
    }
    let sigma3 = sigma3_for_lagrangian_subset(&selected);
    ConjPencilSigma3SubsetReport {
        n,
        subset_size: selected.len(),
        sigma3,
        sigma3_over_m_squared: sigma3 as f64 / (selected.len() * selected.len()) as f64,
        construction: match core_dimension {
            1 => "k=1 pencil seed",
            2 => "k=2 pencil seed",
            _ => "k>=3 pencil seed",
        },
        status: "OPEN",
        reduction_claim: false,
        security_claim: false,
    }
}

pub fn conj_pencil_sigma3_report_json() -> String {
    conj_pencil_sigma3_report_json_with_n5(true)
}

pub fn conj_pencil_sigma3_max_search_report() -> Vec<ConjPencilSigma3MaxSearchRow> {
    [4usize, 5, 6]
.into_iter()
.map(conj_pencil_sigma3_max_search_row)
.collect()
}

pub fn conj_pencil_sigma3_max_search_report_json() -> String {
    let gates = [4usize, 5, 6]
.into_iter()
.map(conj_pencil_sigma3_full_pencil_gate)
.collect::<Vec<_>>();
    let rows = conj_pencil_sigma3_max_search_report();
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("  \"experiment\": \"-sigma3-max-search-n456\",\n");
    out.push_str("  \"claim_level\": \"EVIDENCE\",\n");
    out.push_str("  \"status\": \"OPEN\",\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str(
        "  \"target\": \"max Sigma_3(S)/|S|^2 search over full-k2-pencil-size subsets\",\n",
);
    out.push_str("  \"search_scope\": \"bounded deterministic lower-bound search: full k=2 pencil gate plus structured candidate seeds; not an exhaustive max proof\",\n");
    out.push_str("  \"full_k2_pencil_gates\": [\n");
    for (idx, gate) in gates.iter().enumerate() {
        let comma = if idx + 1 == gates.len() { "" } else { "," };
        out.push_str(&format!("    {}{}\n", full_pencil_gate_json(gate), comma));
    }
    out.push_str("  ],\n");
    out.push_str("  \"search_rows\": [\n");
    for (idx, row) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        out.push_str(&format!("    {}{}\n", max_search_row_json(row), comma));
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

pub fn conj_pencil_sigma3_max_search_v2_report() -> Vec<ConjPencilSigma3V2SearchRow> {
    [4usize, 5, 6]
.into_iter()
.map(conj_pencil_sigma3_v2_search_row)
.collect()
}

pub fn conj_pencil_sigma3_v2_special_config_report() -> Vec<ConjPencilSigma3V2SpecialConfigRow> {
    [4usize, 5, 6]
.into_iter()
.flat_map(conj_pencil_sigma3_v2_special_config_rows)
.collect()
}

pub fn conj_pencil_sigma3_max_search_v2_report_json() -> String {
    let gates = [4usize, 5, 6]
.into_iter()
.map(conj_pencil_sigma3_full_pencil_gate)
.collect::<Vec<_>>();
    let rows = conj_pencil_sigma3_max_search_v2_report();
    let specials = conj_pencil_sigma3_v2_special_config_report();
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("  \"experiment\": \"-sigma3-max-search-v2-n456\",\n");
    out.push_str("  \"claim_level\": \"EVIDENCE\",\n");
    out.push_str("  \"status\": \"OPEN\",\n");
    out.push_str("  \"reduction_claim\": false,\n");
    out.push_str("  \"security_claim\": false,\n");
    out.push_str("  \"production_constant_time_claim\": false,\n");
    out.push_str(
        "  \"target\": \"v2 audit search for max Sigma_3(S)/|S|^2 over full-k2-pencil-size subsets\",\n",
);
    out.push_str("  \"search_scope\": \"bounded randomized-restart evidence search over quotient-lift and symplectic-transvection k=2 pencil images; not an exhaustive max proof\",\n");
    out.push_str("  \"full_k2_pencil_gates\": [\n");
    for (idx, gate) in gates.iter().enumerate() {
        let comma = if idx + 1 == gates.len() { "" } else { "," };
        out.push_str(&format!("    {}{}\n", full_pencil_gate_json(gate), comma));
    }
    out.push_str("  ],\n");
    out.push_str("  \"search_rows\": [\n");
    for (idx, row) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        out.push_str(&format!("    {}{}\n", v2_search_row_json(row), comma));
    }
    out.push_str("  ],\n");
    out.push_str("  \"special_config_rows\": [\n");
    for (idx, row) in specials.iter().enumerate() {
        let comma = if idx + 1 == specials.len() { "" } else { "," };
        out.push_str(&format!(
            "    {}{}\n",
            v2_special_config_row_json(row),
            comma
));
    }
    out.push_str("  ],\n");
    out.push_str("  \"verdict\": \"bounded evidence search only: escalate values and any full-pencil exceedance to Track A; no reduction or security claim\"\n");
    out.push_str("}\n");
    out
}

pub fn conj_pencil_sigma3_light_report_json() -> String {
    conj_pencil_sigma3_report_json_with_n5(false)
}

fn conj_pencil_sigma3_report_json_with_n5(include_n5: bool) -> String {
    let n3 = conj_pencil_sigma3_n3_sanity_report(7);
    let n4_incidence = conj_pencil_sigma3_incidence_report(4);
    let n4_scale = nearest_conj_pencil_scale(4);
    let n4_pencil = conj_pencil_sigma3_pencil_report(4, 2, Some(n4_scale));
    let n5_fields = if include_n5 {
        let n5_incidence = conj_pencil_sigma3_incidence_report(5);
        let n5_scale = nearest_conj_pencil_scale(5);
        let n5_pencil = conj_pencil_sigma3_pencil_report(5, 2, Some(n5_scale));
        let n6_scale = nearest_conj_pencil_scale(6);
        let n6_pencil = conj_pencil_sigma3_pencil_report(6, 2, Some(n6_scale));
        format!(
            concat!(
                ",\n",
                "  \"n5_incidence\": {},\n",
                "  \"n5_k2_pencil_scale_seed\": {},\n",
                "  \"n6_k2_pencil_scale_seed\": {}\n"
),
            incidence_report_json(&n5_incidence),
            subset_report_json(&n5_pencil),
            subset_report_json(&n6_pencil)
)
    } else {
        "\n".to_string()
    };

    format!(
        concat!(
            "{{\n",
            "  \"experiment\": \"-conj-pencil-sigma3-search-n6\",\n",
            "  \"claim_level\": \"EVIDENCE\",\n",
            "  \"status\": \"OPEN\",\n",
            "  \"reduction_claim\": false,\n",
            "  \"security_claim\": false,\n",
            "  \"production_constant_time_claim\": false,\n",
            "  \"target\": \"empirical Sigma_3(S)/M^2 constant for conj:pencil supersaturation\",\n",
            "  \"interpretation\": \"k=2 pencil scale seeds grow across n=4,5,6; this is evidence against a bare absolute Sigma_3 <= A M^2 premise, not a proof and not a conj:pencil break\",\n",
            "  \"n3_sanity\": {},\n",
            "  \"n4_incidence\": {},\n",
            "  \"n4_k2_pencil_scale_seed\": {}{}",
            "}}\n"
),
        subset_report_json(&n3),
        incidence_report_json(&n4_incidence),
        subset_report_json(&n4_pencil),
        n5_fields
)
}

pub fn nearest_conj_pencil_scale(n: usize) -> usize {
    let lagrangians = lagrangian_count_formula(n);
    ((lagrangians as f64) / 2f64.powi((2 * n) as i32)).round() as usize
}

pub fn conj_pencil_sigma3_full_pencil_gate(n: usize) -> ConjPencilSigma3FullPencilGate {
    let ratio = full_k2_pencil_sigma3_ratio(n);
    let expected_gate_value = match n {
        4 => 7.0 / 5.0,
        5 => 421.0 / 135.0,
        6 => 83.0 / 17.0,
        _ => ratio,
    };
    ConjPencilSigma3FullPencilGate {
        n,
        pencil_size: lagrangian_count_formula(n - 2),
        sigma3_over_pencil_size_squared: ratio,
        expected_gate_value,
        gate_passed: (ratio - expected_gate_value).abs() < 0.01,
        status: "OPEN",
        claim_level: "EVIDENCE",
        reduction_claim: false,
        security_claim: false,
    }
}

pub fn sigma3_for_lagrangian_subset(lagrangians: &[Vec<u16>]) -> u64 {
    let mut counts: HashMap<Vec<u16>, u32> = HashMap::new();
    for lagrangian in lagrangians {
        for three_space in subspace_keys_inside_basis(lagrangian, 3) {
            *counts.entry(three_space).or_insert(0) += 1;
        }
    }
    counts
.values()
.map(|count| u64::from(*count) * u64::from(*count))
.sum()
}

pub fn lagrangian_count_formula(n: usize) -> usize {
    (1..=n).map(|i| (1usize << i) + 1).product()
}

fn conj_pencil_sigma3_max_search_row(n: usize) -> ConjPencilSigma3MaxSearchRow {
    let full_pencil = conj_pencil_sigma3_full_pencil_gate(n);
    let bounded = bounded_candidate_max_search(n, full_pencil.pencil_size);
    let (best_ratio, best_construction, search_kind, inspected_candidate_count) =
        if let Some(search) = bounded {
            (
                search
.best_ratio
.max(full_pencil.sigma3_over_pencil_size_squared),
                if search.best_ratio > full_pencil.sigma3_over_pencil_size_squared {
                    "bounded greedy/local candidate pool"
                } else {
                    "full k=2 pencil"
                },
                search.search_kind,
                search.inspected_candidate_count,
)
        } else {
            (
                full_pencil.sigma3_over_pencil_size_squared,
                "full k=2 pencil",
                "full pencil gate only; candidate-pool search deferred",
                full_pencil.pencil_size,
)
        };
    ConjPencilSigma3MaxSearchRow {
        n,
        subset_size: full_pencil.pencil_size,
        inspected_candidate_count,
        full_pencil_ratio: full_pencil.sigma3_over_pencil_size_squared,
        best_ratio,
        best_construction,
        search_kind,
        exceeded_full_pencil: best_ratio > full_pencil.sigma3_over_pencil_size_squared + 1e-12,
        status: "OPEN",
        claim_level: "EVIDENCE",
        reduction_claim: false,
        security_claim: false,
    }
}

fn full_k2_pencil_sigma3_ratio(n: usize) -> f64 {
    assert!(n >= 4);
    let m = n - 2;
    let pencil_size = lagrangian_count_formula(m) as f64;
    let weighted_sum = (0..=m)
.map(|j| {
            let intersection_count =
                gaussian_binomial(m, j) * (1u128 << ((m - j) * (m - j + 1) / 2));
            intersection_count * gaussian_binomial(j + 2, 3)
        })
.sum::<u128>() as f64;
    weighted_sum / pencil_size
}

struct BoundedSearchResult {
    best_ratio: f64,
    inspected_candidate_count: usize,
    search_kind: &'static str,
}

fn bounded_candidate_max_search(n: usize, subset_size: usize) -> Option<BoundedSearchResult> {
    if n > 5 {
        return None;
    }
    let candidates = bounded_k2_pencil_candidate_pool(n, 8);
    if candidates.len() < subset_size {
        return None;
    }
    let incidence = sigma3_candidate_incidence(&candidates);
    let selected = greedy_sigma3_subset(&incidence, subset_size);
    let improved = local_swap_improve(&incidence, selected);
    let sigma3 = sigma3_score_for_indices(&incidence, &improved);
    Some(BoundedSearchResult {
        best_ratio: sigma3 as f64 / (subset_size * subset_size) as f64,
        inspected_candidate_count: candidates.len(),
        search_kind: "bounded greedy plus single-swap local search over k=2 pencil candidate pool",
    })
}

fn conj_pencil_sigma3_v2_search_row(n: usize) -> ConjPencilSigma3V2SearchRow {
    let full_pencil = conj_pencil_sigma3_full_pencil_gate(n);
    let subset_size = full_pencil.pencil_size;
    let candidates = v2_candidate_pool(n);
    let incidence = sigma3_candidate_incidence(&candidates);
    let restart_count = v2_restart_count(n);
    let selected = if n <= 5 {
        random_restart_greedy_local(&incidence, subset_size, restart_count)
    } else {
        seeded_swap_search(&incidence, subset_size, restart_count, v2_n6_swap_budget())
    };
    let best_sigma3 = sigma3_score_for_indices(&incidence, &selected);
    let search_ratio = best_sigma3 as f64 / (subset_size * subset_size) as f64;
    let best_ratio = search_ratio.max(full_pencil.sigma3_over_pencil_size_squared);
    ConjPencilSigma3V2SearchRow {
        n,
        subset_size,
        inspected_candidate_count: candidates.len(),
        restart_count,
        full_pencil_ratio: full_pencil.sigma3_over_pencil_size_squared,
        best_ratio,
        best_construction: if search_ratio > full_pencil.sigma3_over_pencil_size_squared {
            "v2 randomized-restart candidate search"
        } else {
            "full k=2 pencil"
        },
        search_kind: if n <= 5 {
            "v2 random-restart greedy/local over quotient-lift and symplectic-transvection pencil images"
        } else {
            "v2 n=6 quotient-lift seeded randomized swap search"
        },
        exceeded_full_pencil: best_ratio > full_pencil.sigma3_over_pencil_size_squared + 1e-12,
        status: "OPEN",
        claim_level: "EVIDENCE",
        reduction_claim: false,
        security_claim: false,
    }
}

fn conj_pencil_sigma3_v2_special_config_rows(n: usize) -> Vec<ConjPencilSigma3V2SpecialConfigRow> {
    let full_pencil = conj_pencil_sigma3_full_pencil_gate(n);
    let subset_size = full_pencil.pencil_size;
    [
        (
            "near-2-pencil: P_W plus one transverse Lagrangian",
            near_two_pencil_subset(n),
),
        (
            "two coordinate k=2 pencils truncated to full-pencil size",
            pencil_union_subset(n, 2, subset_size),
),
        (
            "three coordinate k=2 pencils truncated to full-pencil size",
            pencil_union_subset(n, 3, subset_size),
),
        (
            "dim-(n-1) heavy-pair seeds plus full-pencil fill",
            heavy_pair_seeded_subset(n, subset_size),
),
    ]
.into_iter()
.map(|(construction, subset)| {
        let sigma3 = sigma3_for_lagrangian_subset(&subset);
        let config_ratio = sigma3 as f64 / (subset.len() * subset.len()) as f64;
        ConjPencilSigma3V2SpecialConfigRow {
            n,
            subset_size: subset.len(),
            full_pencil_ratio: full_pencil.sigma3_over_pencil_size_squared,
            config_ratio,
            construction,
            exceeded_full_pencil: config_ratio
                > full_pencil.sigma3_over_pencil_size_squared + 1e-12,
            status: "OPEN",
            claim_level: "EVIDENCE",
            reduction_claim: false,
            security_claim: false,
        }
    })
.collect()
}

fn v2_candidate_pool(n: usize) -> Vec<Vec<u16>> {
    let mut pool = Vec::new();
    let mut seen = HashSet::new();
    push_unique_lagrangians(
        &mut pool,
        &mut seen,
        lagrangians_containing_standard_core(n, 2),
);
    for core in v2_coordinate_core_pairs(n) {
        push_unique_lagrangians(
            &mut pool,
            &mut seen,
            lagrangians_containing_coordinate_x_core(n, &core),
);
    }
    for seed in 0..v2_transvection_seed_count(n) {
        push_unique_lagrangians(
            &mut pool,
            &mut seen,
            symplectic_transvection_pencil_image(n, seed),
);
    }
    pool
}

fn v2_coordinate_core_pairs(n: usize) -> Vec<Vec<usize>> {
    let mut pairs = combinations(n, 2);
    pairs.truncate(match n {
        4 => 6,
        5 => 8,
        _ => 3,
    });
    pairs
}

fn v2_transvection_seed_count(n: usize) -> usize {
    match n {
        4 => 8,
        5 => 5,
        _ => 1,
    }
}

fn v2_restart_count(n: usize) -> usize {
    match n {
        4 => 8,
        5 => 4,
        _ => 8,
    }
}

fn v2_n6_swap_budget() -> usize {
    96
}

fn push_unique_lagrangians(
    pool: &mut Vec<Vec<u16>>,
    seen: &mut HashSet<Vec<u16>>,
    lagrangians: Vec<Vec<u16>>,
) {
    for lagrangian in lagrangians {
        let key = subspace_key(&lagrangian);
        if seen.insert(key) {
            pool.push(lagrangian);
        }
    }
}

fn random_restart_greedy_local(
    incidence: &[Vec<usize>],
    subset_size: usize,
    restart_count: usize,
) -> Vec<usize> {
    let mut best = greedy_sigma3_subset(incidence, subset_size);
    best = local_swap_improve(incidence, best);
    let mut best_score = sigma3_score_for_indices(incidence, &best);
    for restart in 0..restart_count {
        let order = deterministic_shuffle_indices(incidence.len(), restart as u64 + 1);
        let selected = greedy_sigma3_subset_with_order(incidence, subset_size, &order);
        let improved = local_swap_improve(incidence, selected);
        let score = sigma3_score_for_indices(incidence, &improved);
        if score > best_score {
            best_score = score;
            best = improved;
        }
    }
    best
}

fn greedy_sigma3_subset_with_order(
    incidence: &[Vec<usize>],
    subset_size: usize,
    order: &[usize],
) -> Vec<usize> {
    let space_count = incidence
.iter()
.flat_map(|row| row.iter().copied())
.max()
.map(|x| x + 1)
.unwrap_or(0);
    let mut counts = vec![0u16; space_count];
    let mut selected = Vec::with_capacity(subset_size);
    let mut used = vec![false; incidence.len()];
    while selected.len() < subset_size {
        let mut best = None;
        for idx in order {
            if used[*idx] {
                continue;
            }
            let gain = incidence[*idx]
.iter()
.map(|space| 2u64 * u64::from(counts[*space]) + 1)
.sum::<u64>();
            if best.is_none_or(|(_, best_gain)| gain > best_gain) {
                best = Some((*idx, gain));
            }
        }
        let (idx, _) = best.expect("candidate pool must have enough elements");
        used[idx] = true;
        selected.push(idx);
        for space in &incidence[idx] {
            counts[*space] += 1;
        }
    }
    selected
}

fn seeded_swap_search(
    incidence: &[Vec<usize>],
    subset_size: usize,
    restart_count: usize,
    swap_budget: usize,
) -> Vec<usize> {
    let mut best: Vec<_> = (0..subset_size).collect();
    let mut best_score = sigma3_score_for_indices(incidence, &best);
    for restart in 0..restart_count {
        let mut selected = (0..subset_size).collect::<Vec<_>>();
        let mut used = vec![false; incidence.len()];
        for idx in &selected {
            used[*idx] = true;
        }
        let mut counts = counts_for_selected(incidence, &selected);
        let mut rng = splitmix64(restart as u64 + 0x9e37_79b9_7f4a_7c15);
        for _ in 0..swap_budget {
            let remove_pos = (next_splitmix64(&mut rng) as usize) % selected.len();
            let add_idx = subset_size
                + (next_splitmix64(&mut rng) as usize) % (incidence.len() - subset_size);
            if used[add_idx] {
                continue;
            }
            let remove_idx = selected[remove_pos];
            for space in &incidence[remove_idx] {
                counts[*space] -= 1;
            }
            let remove_loss = incidence[remove_idx]
.iter()
.map(|space| 2u64 * u64::from(counts[*space]) + 1)
.sum::<u64>();
            let add_gain = incidence[add_idx]
.iter()
.map(|space| 2u64 * u64::from(counts[*space]) + 1)
.sum::<u64>();
            if add_gain > remove_loss {
                used[remove_idx] = false;
                used[add_idx] = true;
                selected[remove_pos] = add_idx;
                for space in &incidence[add_idx] {
                    counts[*space] += 1;
                }
            } else {
                for space in &incidence[remove_idx] {
                    counts[*space] += 1;
                }
            }
        }
        let score = sigma3_score_for_indices(incidence, &selected);
        if score > best_score {
            best_score = score;
            best = selected;
        }
    }
    best
}

fn counts_for_selected(incidence: &[Vec<usize>], selected: &[usize]) -> Vec<u16> {
    let space_count = incidence
.iter()
.flat_map(|row| row.iter().copied())
.max()
.map(|x| x + 1)
.unwrap_or(0);
    let mut counts = vec![0u16; space_count];
    for idx in selected {
        for space in &incidence[*idx] {
            counts[*space] += 1;
        }
    }
    counts
}

fn deterministic_shuffle_indices(len: usize, seed: u64) -> Vec<usize> {
    let mut keyed = (0..len)
.map(|idx| (splitmix64(seed ^ idx as u64), idx))
.collect::<Vec<_>>();
    keyed.sort_by_key(|(key, _)| *key);
    keyed.into_iter().map(|(_, idx)| idx).collect()
}

fn symplectic_transvection_pencil_image(n: usize, seed: usize) -> Vec<Vec<u16>> {
    lagrangians_containing_standard_core(n, 2)
.into_iter()
.map(|basis| {
            basis
.into_iter()
.map(|vector| apply_transvection_sequence(vector, n, seed))
.collect()
        })
.collect()
}

fn apply_transvection_sequence(mut vector: u16, n: usize, seed: usize) -> u16 {
    for step in 0..(n + 2) {
        let transvection = seeded_nonzero_vector(n, seed, step);
        if symplectic_pairing(vector, transvection, n) == 1 {
            vector ^= transvection;
        }
    }
    vector
}

fn seeded_nonzero_vector(n: usize, seed: usize, step: usize) -> u16 {
    let width = 2 * n;
    let mask = (1u64 << width) - 1;
    let mut value = splitmix64(((seed as u64) << 32) ^ step as u64) & mask;
    if value == 0 {
        value = 1;
    }
    value as u16
}

fn near_two_pencil_subset(n: usize) -> Vec<Vec<u16>> {
    let mut subset = lagrangians_containing_standard_core(n, 2);
    subset.push(transverse_z_lagrangian(n));
    subset
}

fn pencil_union_subset(n: usize, pencil_count: usize, subset_size: usize) -> Vec<Vec<u16>> {
    let mut pool = Vec::new();
    let mut seen = HashSet::new();
    push_unique_lagrangians(
        &mut pool,
        &mut seen,
        lagrangians_containing_standard_core(n, 2),
);
    for core in v2_coordinate_core_pairs(n).into_iter().take(pencil_count) {
        push_unique_lagrangians(
            &mut pool,
            &mut seen,
            lagrangians_containing_coordinate_x_core(n, &core),
);
    }
    pool.truncate(subset_size);
    pool
}

fn heavy_pair_seeded_subset(n: usize, subset_size: usize) -> Vec<Vec<u16>> {
    let mut pool = Vec::new();
    let mut seen = HashSet::new();
    for core in combinations(n, n - 1) {
        push_unique_lagrangians(
            &mut pool,
            &mut seen,
            lagrangians_containing_coordinate_x_core(n, &core),
);
    }
    push_unique_lagrangians(
        &mut pool,
        &mut seen,
        lagrangians_containing_standard_core(n, 2),
);
    pool.truncate(subset_size);
    pool
}

fn transverse_z_lagrangian(n: usize) -> Vec<u16> {
    (0..n).map(|idx| 1u16 << (n + idx)).collect()
}

fn bounded_k2_pencil_candidate_pool(n: usize, max_cores: usize) -> Vec<Vec<u16>> {
    let lagrangians = enumerate_lagrangians(n);
    let cores = enumerate_isotropic_subspace_bases(n, 2);
    let mut pool = HashMap::new();
    for core in cores.into_iter().take(max_cores) {
        for lagrangian in &lagrangians {
            let key = subspace_key(lagrangian);
            if core.iter().all(|v| subspace_key_contains(&key, *v)) {
                pool.insert(key, lagrangian.clone());
            }
        }
    }
    let mut pool: Vec<_> = pool.into_iter().collect();
    pool.sort_by(|left, right| left.0.cmp(&right.0));
    pool.into_iter().map(|(_, basis)| basis).collect()
}

fn sigma3_candidate_incidence(candidates: &[Vec<u16>]) -> Vec<Vec<usize>> {
    let mut ids = HashMap::new();
    let mut incidence = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        let mut row = Vec::new();
        for three_space in subspace_keys_inside_basis(candidate, 3) {
            let next = ids.len();
            row.push(*ids.entry(three_space).or_insert(next));
        }
        row.sort_unstable();
        row.dedup();
        incidence.push(row);
    }
    incidence
}

fn greedy_sigma3_subset(incidence: &[Vec<usize>], subset_size: usize) -> Vec<usize> {
    let space_count = incidence
.iter()
.flat_map(|row| row.iter().copied())
.max()
.map(|x| x + 1)
.unwrap_or(0);
    let mut counts = vec![0u16; space_count];
    let mut selected = Vec::with_capacity(subset_size);
    let mut used = vec![false; incidence.len()];
    while selected.len() < subset_size {
        let mut best = None;
        for (idx, row) in incidence.iter().enumerate() {
            if used[idx] {
                continue;
            }
            let gain = row
.iter()
.map(|space| 2u64 * u64::from(counts[*space]) + 1)
.sum::<u64>();
            if best.is_none_or(|(_, best_gain)| gain > best_gain) {
                best = Some((idx, gain));
            }
        }
        let (idx, _) = best.expect("candidate pool must have enough elements");
        used[idx] = true;
        selected.push(idx);
        for space in &incidence[idx] {
            counts[*space] += 1;
        }
    }
    selected
}

fn local_swap_improve(incidence: &[Vec<usize>], mut selected: Vec<usize>) -> Vec<usize> {
    let space_count = incidence
.iter()
.flat_map(|row| row.iter().copied())
.max()
.map(|x| x + 1)
.unwrap_or(0);
    let mut counts = vec![0u16; space_count];
    let mut used = vec![false; incidence.len()];
    for idx in &selected {
        used[*idx] = true;
        for space in &incidence[*idx] {
            counts[*space] += 1;
        }
    }

    let mut improved = true;
    while improved {
        improved = false;
        'search: for selected_pos in 0..selected.len() {
            let remove_idx = selected[selected_pos];
            for space in &incidence[remove_idx] {
                counts[*space] -= 1;
            }
            let remove_loss = incidence[remove_idx]
.iter()
.map(|space| 2u64 * u64::from(counts[*space]) + 1)
.sum::<u64>();

            let mut best_add = None;
            for (candidate_idx, row) in incidence.iter().enumerate() {
                if used[candidate_idx] {
                    continue;
                }
                let add_gain = row
.iter()
.map(|space| 2u64 * u64::from(counts[*space]) + 1)
.sum::<u64>();
                if add_gain > remove_loss
                    && best_add.is_none_or(|(_, best_gain)| add_gain > best_gain)
                {
                    best_add = Some((candidate_idx, add_gain));
                }
            }

            if let Some((add_idx, _)) = best_add {
                used[remove_idx] = false;
                used[add_idx] = true;
                selected[selected_pos] = add_idx;
                for space in &incidence[add_idx] {
                    counts[*space] += 1;
                }
                improved = true;
                break 'search;
            }

            for space in &incidence[remove_idx] {
                counts[*space] += 1;
            }
        }
    }
    selected
}

fn sigma3_score_for_indices(incidence: &[Vec<usize>], selected: &[usize]) -> u64 {
    let mut counts = HashMap::new();
    for idx in selected {
        for space in &incidence[*idx] {
            *counts.entry(*space).or_insert(0u32) += 1;
        }
    }
    counts
.values()
.map(|count| u64::from(*count) * u64::from(*count))
.sum()
}

fn gaussian_binomial(n: usize, k: usize) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut numerator = 1u128;
    let mut denominator = 1u128;
    for i in 0..k {
        numerator *= (1u128 << (n - i)) - 1;
        denominator *= (1u128 << (k - i)) - 1;
    }
    numerator / denominator
}

fn full_pencil_gate_json(gate: &ConjPencilSigma3FullPencilGate) -> String {
    format!(
        concat!(
            "{{\"n\": {}, \"pencil_size\": {}, ",
            "\"sigma3_over_pencil_size_squared\": {:.12}, ",
            "\"expected_gate_value\": {:.12}, \"gate_passed\": {}, ",
            "\"status\": \"{}\", \"claim_level\": \"{}\", ",
            "\"reduction_claim\": {}, \"security_claim\": {}}}"
),
        gate.n,
        gate.pencil_size,
        gate.sigma3_over_pencil_size_squared,
        gate.expected_gate_value,
        gate.gate_passed,
        gate.status,
        gate.claim_level,
        gate.reduction_claim,
        gate.security_claim
)
}

fn max_search_row_json(row: &ConjPencilSigma3MaxSearchRow) -> String {
    format!(
        concat!(
            "{{\"n\": {}, \"subset_size\": {}, ",
            "\"inspected_candidate_count\": {}, ",
            "\"full_pencil_ratio\": {:.12}, \"best_ratio\": {:.12}, ",
            "\"best_construction\": \"{}\", \"search_kind\": \"{}\", ",
            "\"exceeded_full_pencil\": {}, \"status\": \"{}\", ",
            "\"claim_level\": \"{}\", \"reduction_claim\": {}, ",
            "\"security_claim\": {}}}"
),
        row.n,
        row.subset_size,
        row.inspected_candidate_count,
        row.full_pencil_ratio,
        row.best_ratio,
        row.best_construction,
        row.search_kind,
        row.exceeded_full_pencil,
        row.status,
        row.claim_level,
        row.reduction_claim,
        row.security_claim
)
}

fn v2_search_row_json(row: &ConjPencilSigma3V2SearchRow) -> String {
    format!(
        concat!(
            "{{\"n\": {}, \"subset_size\": {}, ",
            "\"inspected_candidate_count\": {}, \"restart_count\": {}, ",
            "\"full_pencil_ratio\": {:.12}, \"best_ratio\": {:.12}, ",
            "\"best_construction\": \"{}\", \"search_kind\": \"{}\", ",
            "\"exceeded_full_pencil\": {}, \"status\": \"{}\", ",
            "\"claim_level\": \"{}\", \"reduction_claim\": {}, ",
            "\"security_claim\": {}}}"
),
        row.n,
        row.subset_size,
        row.inspected_candidate_count,
        row.restart_count,
        row.full_pencil_ratio,
        row.best_ratio,
        row.best_construction,
        row.search_kind,
        row.exceeded_full_pencil,
        row.status,
        row.claim_level,
        row.reduction_claim,
        row.security_claim
)
}

fn v2_special_config_row_json(row: &ConjPencilSigma3V2SpecialConfigRow) -> String {
    format!(
        concat!(
            "{{\"n\": {}, \"subset_size\": {}, ",
            "\"full_pencil_ratio\": {:.12}, \"config_ratio\": {:.12}, ",
            "\"construction\": \"{}\", \"exceeded_full_pencil\": {}, ",
            "\"status\": \"{}\", \"claim_level\": \"{}\", ",
            "\"reduction_claim\": {}, \"security_claim\": {}}}"
),
        row.n,
        row.subset_size,
        row.full_pencil_ratio,
        row.config_ratio,
        row.construction,
        row.exceeded_full_pencil,
        row.status,
        row.claim_level,
        row.reduction_claim,
        row.security_claim
)
}

fn incidence_report_json(report: &ConjPencilSigma3IncidenceReport) -> String {
    format!(
        concat!(
            "{{\"n\": {}, ",
            "\"lagrangian_count\": {}, ",
            "\"isotropic_three_space_count\": {}, ",
            "\"three_spaces_per_lagrangian\": {}, ",
            "\"lagrangians_per_three_space\": {}, ",
            "\"status\": \"{}\", ",
            "\"reduction_claim\": {}, ",
            "\"security_claim\": {}}}"
),
        report.n,
        report.lagrangian_count,
        report.isotropic_three_space_count,
        report.three_spaces_per_lagrangian,
        report.lagrangians_per_three_space,
        report.status,
        report.reduction_claim,
        report.security_claim
)
}

fn subset_report_json(report: &ConjPencilSigma3SubsetReport) -> String {
    format!(
        concat!(
            "{{\"n\": {}, ",
            "\"subset_size\": {}, ",
            "\"sigma3\": {}, ",
            "\"sigma3_over_m_squared\": {:.12}, ",
            "\"construction\": \"{}\", ",
            "\"status\": \"{}\", ",
            "\"reduction_claim\": {}, ",
            "\"security_claim\": {}}}"
),
        report.n,
        report.subset_size,
        report.sigma3,
        report.sigma3_over_m_squared,
        report.construction,
        report.status,
        report.reduction_claim,
        report.security_claim
)
}

fn enumerate_lagrangians(n: usize) -> Vec<Vec<u16>> {
    enumerate_isotropic_subspace_bases(n, n)
}

fn lagrangians_containing_standard_core(n: usize, core_dimension: usize) -> Vec<Vec<u16>> {
    assert!(core_dimension <= n);
    let quotient_rank = n - core_dimension;
    let quotient_lagrangians = enumerate_lagrangians(quotient_rank);
    quotient_lagrangians
.into_iter()
.map(|quotient_basis| lift_quotient_lagrangian(n, core_dimension, &quotient_basis))
.collect()
}

fn lagrangians_containing_coordinate_x_core(n: usize, core_indices: &[usize]) -> Vec<Vec<u16>> {
    assert!(core_indices.len() <= n);
    let mut remaining = Vec::new();
    for idx in 0..n {
        if !core_indices.contains(&idx) {
            remaining.push(idx);
        }
    }
    let quotient_rank = remaining.len();
    enumerate_lagrangians(quotient_rank)
.into_iter()
.map(|quotient_basis| {
            lift_coordinate_x_core_quotient_lagrangian(n, core_indices, &remaining, &quotient_basis)
        })
.collect()
}

fn lift_coordinate_x_core_quotient_lagrangian(
    n: usize,
    core_indices: &[usize],
    remaining: &[usize],
    quotient_basis: &[u16],
) -> Vec<u16> {
    let quotient_rank = remaining.len();
    let mut basis: Vec<u16> = core_indices.iter().map(|idx| 1u16 << idx).collect();
    for vector in quotient_basis {
        let mut lifted = 0u16;
        for (quotient_idx, original_idx) in remaining.iter().enumerate() {
            if ((vector >> quotient_idx) & 1) == 1 {
                lifted ^= 1u16 << original_idx;
            }
            if ((vector >> (quotient_rank + quotient_idx)) & 1) == 1 {
                lifted ^= 1u16 << (n + original_idx);
            }
        }
        basis.push(lifted);
    }
    basis
}

fn lift_quotient_lagrangian(n: usize, core_dimension: usize, quotient_basis: &[u16]) -> Vec<u16> {
    let quotient_rank = n - core_dimension;
    let mut basis: Vec<u16> = (0..core_dimension).map(|i| 1u16 << i).collect();
    for vector in quotient_basis {
        let mut lifted = 0u16;
        for idx in 0..quotient_rank {
            if ((vector >> idx) & 1) == 1 {
                lifted ^= 1u16 << (core_dimension + idx);
            }
            if ((vector >> (quotient_rank + idx)) & 1) == 1 {
                lifted ^= 1u16 << (n + core_dimension + idx);
            }
        }
        basis.push(lifted);
    }
    basis
}

fn enumerate_isotropic_subspace_keys(n: usize, k: usize) -> Vec<Vec<u16>> {
    let mut keys: Vec<_> = enumerate_isotropic_subspace_bases(n, k)
.into_iter()
.map(|basis| subspace_key(&basis))
.collect();
    keys.sort();
    keys
}

fn enumerate_isotropic_subspace_bases(n: usize, k: usize) -> Vec<Vec<u16>> {
    let dim = 2 * n;
    let mut keyed_bases = HashMap::new();
    for pivots in combinations(dim, k) {
        enumerate_rref_rows_for_pivots(dim, &pivots, |basis| {
            if is_totally_isotropic(basis, n) {
                keyed_bases.insert(subspace_key(basis), basis.to_vec());
            }
        });
    }
    let mut keyed_bases: Vec<_> = keyed_bases.into_iter().collect();
    keyed_bases.sort_by(|left, right| left.0.cmp(&right.0));
    keyed_bases.into_iter().map(|(_, basis)| basis).collect()
}

fn subspace_keys_inside_basis(parent_basis: &[u16], k: usize) -> Vec<Vec<u16>> {
    let mut keys = HashSet::new();
    for pivots in combinations(parent_basis.len(), k) {
        enumerate_rref_rows_for_pivots(parent_basis.len(), &pivots, |coordinate_basis| {
            let mapped: Vec<_> = coordinate_basis
.iter()
.map(|coordinates| linear_combination(parent_basis, *coordinates))
.collect();
            keys.insert(subspace_key(&mapped));
        });
    }
    let mut keys: Vec<_> = keys.into_iter().collect();
    keys.sort();
    keys
}

fn enumerate_rref_rows_for_pivots<F>(dim: usize, pivots: &[usize], mut visit: F)
where
    F: FnMut(&[u16]),
{
    let mut free_positions = Vec::new();
    for (row, pivot) in pivots.iter().enumerate() {
        for col in (pivot + 1)..dim {
            if !pivots.contains(&col) {
                free_positions.push((row, col));
            }
        }
    }

    let free_count = free_positions.len();
    for mask in 0..(1usize << free_count) {
        let mut rows: Vec<u16> = pivots.iter().map(|pivot| 1u16 << pivot).collect();
        for (idx, (row, col)) in free_positions.iter().enumerate() {
            if ((mask >> idx) & 1) == 1 {
                rows[*row] |= 1u16 << col;
            }
        }
        visit(&rows);
    }
}

fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    fn rec(out: &mut Vec<Vec<usize>>, cur: &mut Vec<usize>, start: usize, n: usize, k: usize) {
        if cur.len() == k {
            out.push(cur.clone());
            return;
        }
        let remaining = k - cur.len();
        for value in start..=(n - remaining) {
            cur.push(value);
            rec(out, cur, value + 1, n, k);
            cur.pop();
        }
    }

    let mut out = Vec::new();
    rec(&mut out, &mut Vec::new(), 0, n, k);
    out
}

fn is_totally_isotropic(basis: &[u16], n: usize) -> bool {
    for (idx, left) in basis.iter().enumerate() {
        for right in basis.iter().skip(idx) {
            if symplectic_pairing(*left, *right, n) != 0 {
                return false;
            }
        }
    }
    true
}

fn symplectic_pairing(left: u16, right: u16, n: usize) -> u8 {
    let mask = (1u16 << n) - 1;
    let left_x = left & mask;
    let left_z = (left >> n) & mask;
    let right_x = right & mask;
    let right_z = (right >> n) & mask;
    (((left_x & right_z).count_ones() + (left_z & right_x).count_ones()) & 1) as u8
}

fn subspace_key(basis: &[u16]) -> Vec<u16> {
    let mut span = vec![0u16];
    for vector in basis {
        let current = span.clone();
        for existing in current {
            span.push(existing ^ *vector);
        }
    }
    span.sort_unstable();
    span.dedup();
    span
}

fn linear_combination(basis: &[u16], coefficients: u16) -> u16 {
    let mut out = 0u16;
    for (idx, vector) in basis.iter().enumerate() {
        if ((coefficients >> idx) & 1) == 1 {
            out ^= *vector;
        }
    }
    out
}

fn subspace_key_contains(key: &[u16], vector: u16) -> bool {
    key.binary_search(&vector).is_ok()
}

fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

fn next_splitmix64(state: &mut u64) -> u64 {
    *state = splitmix64(*state);
    *state
}
