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

pub type ConjM2SpanMask = [u64; 4];

pub const fn conj_m2_fresh_evidence_target_ns() -> [usize; 2] {
    [3, 4]
}

pub fn conj_m2_symplectic_pairing(u: u16, v: u16, n: usize) -> u8 {
    assert!(
        (1..=4).contains(&n),
        "conj:m2 fresh helper supports 1 <= n <= 4"
);
    let mut acc = 0u8;
    for i in 0..n {
        let ux = ((u >> i) & 1) as u8;
        let vx = ((v >> i) & 1) as u8;
        let uz = ((u >> (i + n)) & 1) as u8;
        let vz = ((v >> (i + n)) & 1) as u8;
        acc ^= (ux & vz) ^ (uz & vx);
    }
    acc
}

pub fn conj_m2_fresh_evidence_lagrangian_count(n: usize) -> usize {
    conj_m2_lagrangian_span_masks(n).len()
}

pub fn conj_m2_fresh_evidence_lagrangian_count_json() -> String {
    format!(
        concat!(
            "{{\n",
            "  \"experiment\": \"-conj-m2-fresh-lagrangian-counts\",\n",
            "  \"target\": \"conj:m2 n=3,4 fresh evidence\",\n",
            "  \"n3_lagrangian_count\": {},\n",
            "  \"n4_lagrangian_count\": {},\n",
            "  \"archive_use\": \"method reference only; no archived numeric outputs reused\",\n",
            "  \"status\": \"OPEN\",\n",
            "  \"reduction_claim\": false,\n",
            "  \"security_claim\": false,\n",
            "  \"production_constant_time_claim\": false\n",
            "}}\n"
),
        conj_m2_fresh_evidence_lagrangian_count(3),
        conj_m2_fresh_evidence_lagrangian_count(4)
)
}

pub fn conj_m2_lagrangian_span_masks(n: usize) -> Vec<ConjM2SpanMask> {
    assert!(
        (1..=4).contains(&n),
        "conj:m2 fresh helper supports 1 <= n <= 4"
);
    let mut current = HashSet::new();
    current.insert(mask_from_vectors(&[0]));

    for _dimension in 0..n {
        let mut next = HashSet::new();
        for span in &current {
            let vectors = conj_m2_span_mask_vectors(span);
            for candidate in 1..(1u16 << (2 * n)) {
                if conj_m2_span_mask_contains(span, candidate) {
                    continue;
                }
                if vectors
.iter()
.any(|&basis_vec| conj_m2_symplectic_pairing(candidate, basis_vec, n) != 0)
                {
                    continue;
                }
                let mut expanded = vectors.clone();
                expanded.extend(vectors.iter().map(|&span_vec| span_vec ^ candidate));
                next.insert(mask_from_vectors(&expanded));
            }
        }
        current = next;
    }

    let mut spans: Vec<_> = current.into_iter().collect();
    spans.sort();
    spans
}

pub fn conj_m2_span_mask_weight(mask: &ConjM2SpanMask) -> usize {
    mask.iter().map(|word| word.count_ones() as usize).sum()
}

pub fn conj_m2_span_mask_contains(mask: &ConjM2SpanMask, vector: u16) -> bool {
    let idx = vector as usize;
    let word = idx / 64;
    let bit = idx % 64;
    ((mask[word] >> bit) & 1) == 1
}

pub fn conj_m2_span_is_totally_isotropic(mask: &ConjM2SpanMask, n: usize) -> bool {
    let vectors = conj_m2_span_mask_vectors(mask);
    for (i, &left) in vectors.iter().enumerate() {
        for &right in &vectors[i + 1..] {
            if conj_m2_symplectic_pairing(left, right, n) != 0 {
                return false;
            }
        }
    }
    true
}

pub fn conj_m2_span_mask_vectors(mask: &ConjM2SpanMask) -> Vec<u16> {
    let mut vectors = Vec::with_capacity(conj_m2_span_mask_weight(mask));
    for (word_idx, word) in mask.iter().enumerate() {
        let mut bits = *word;
        while bits != 0 {
            let bit = bits.trailing_zeros() as usize;
            vectors.push((word_idx * 64 + bit) as u16);
            bits &= bits - 1;
        }
    }
    vectors
}

fn mask_from_vectors(vectors: &[u16]) -> ConjM2SpanMask {
    let mut mask = [0u64; 4];
    for &vector in vectors {
        let idx = vector as usize;
        let word = idx / 64;
        let bit = idx % 64;
        mask[word] |= 1u64 << bit;
    }
    mask
}
