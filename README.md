# LSN-PQ

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.20646796.svg)](https://doi.org/10.5281/zenodo.20646796)

**The Lagrangian Subspace Noise (LSN) Problem**

Author: Kwanghoo Choo (ORCID [0009-0005-5682-8098](https://orcid.org/0009-0005-5682-8098))

LSN is a *candidate* new post-quantum hardness assumption: an LPN variant whose secret is a
Lagrangian subspace of a symplectic vector space over F₂. This repository contains the papers,
all verification experiments, and the complete research record.

**Papers** (in `paper/`):
- `lsn-core.pdf` — *Lagrangian Subspace Noise: Exact Statistical-Query Lower Bounds and the
  Linear-Reduction Landscape* (the core mathematical paper, **v3.2.7, 52 pp — the current paper**).
- `lsn-paper.pdf` — *Lagrangian Subspace Noise: Cryptographic Constructions* — the companion
  constructions paper (LSN-SNARK; a KEM is left to future work). It builds on `lsn-core.pdf` and cites it for the
  hardness analysis (SQ lower bounds, the reduction landscape, parameters); it contains no
  separate hardness claims.

**Cite this archive** (all versions, always resolves to the latest): DOI [10.5281/zenodo.20646796](https://doi.org/10.5281/zenodo.20646796).
Each tagged release (`v2.0`, `v2.1`, `v3.0`, `v3.1`, `v3.2`, `v3.2.1`, `v3.2.2`, `v3.2.3`, `v3.2.4`, `v3.2.5`, `v3.2.6`, `v3.2.7`) is archived on Zenodo with its own version DOI;
the concept DOI above always resolves to the most recent.

## Claims posture (read this first)

Every claim in the paper is classified as **theorem / evidence / conjecture**.
In particular: no "7th family" is claimed as proven; LSN hardness is an OPEN assumption
supported by exact statistical-query lower bounds and a near-complete linear-reduction
barrier landscape. The public-matrix, fixed, and conditionally-uniform adaptive cells are
closed unconditionally, as are the confined-noise and near-uniform diffuse strategies and
the deterministic half of the fourth cell; a single precisely-stated residual (randomized
marginal-adaptive) remains open. The conditional $2^{2n}$ SQ bound rests on a pencil-extremality
conjecture shown (v3) to follow from a single dual-polar supersaturation inequality (a sufficient
reduction). No production-security claim is made.

## Building

Build with [Tectonic](https://tectonic-typesetting.github.io/):

```bash
cd paper
tectonic lsn-core.tex   # core paper (hardness analysis)
tectonic lsn-paper.tex  # companion constructions paper (LSN-SNARK; cites lsn-core)
```

## Repository structure

```
paper/           LaTeX sources and PDFs (lsn-core = core paper;
                 lsn-paper = companion technical report).
experiments/     Python verification scripts organized by topic (certificate/,
                 moments/, correlation/, barriers/). Every numerical claim in the
                 paper has a reproducing script here.
impl/            Rust crates: lsn_ref (reference scaffold + KAT), lsn_cryptanalysis
                 (attack screens), polar_validation (polar-code decoder validation).
```

External citations are verified against the primary sources, and every numerical result is
reproducible from the scripts in `experiments/`; the central certificate (`experiments/certificate/`)
is self-contained and runs with the standard library only.

## Status

- Paper: the core paper (`lsn-core.pdf`) is a v3 major revision (current **v3.2.7**), submitted to
  the IACR Cryptology ePrint Archive (pending approval); an arXiv submission is planned. v3 adds two new closure results
  (confined-noise and near-uniform diffuse), corrects the open-cell barrier to statistical
  distance ($I(x;y\mid C)=\Theta(n)$), shows the pencil-extremality conjecture follows from a single
  supersaturation inequality, and hardens the source-novelty positioning. v3.1 corrects a reversed
  inequality in that supersaturation reduction (it is sufficient, not an equivalence). **v3.2** proves
  the sympLPN formulation inherits a constant-rate-LPN hardness floor (from the reduction of Khesin et
  al.), adds a noise-universal $M_2$ statistical-query law (over $\mathbb{F}_2$ and $\mathbb{F}_q$) and
  the self-dual-code view with structural inertness; the new results are independently code-verified
  and the paper passed a full-paper desk-screen and a resubmission gate. **v3.2.1** corrects bibliographic
  and citation errors in both papers (all references independently verified against external databases); the mathematical content is unchanged from v3.2.
  **v3.2.2** withdraws the companion's LSN-KEM — the natural reconciliation construction is insecure (its
  reconciliation mask is publicly computable) and the constant-noise regime blocks the standard dual-LPN
  repair — leaving a SNARK-only companion with the KEM documented as future work; it also tightens labeling
  discipline across four adversarial cross-verification rounds (asymptotic-vs-concrete separation,
  evidence/conjecture labels, cross-paper consistency). The core mathematical content is unchanged from v3.2.
  **v3.2.3** is a four-round cross-review revision: it verifies citation integrity (author lists and
  theorem-number attributions checked against the primary sources, one attribution corrected), tightens
  theorem/evidence/conjecture labeling (the distribution-mismatch statement is now an explicit conjecture;
  the entropy-support count is stated in expectation), and right-sizes the hedging of the two conditional
  results (pencil-extremality and the marginal-adaptive barrier) and of the attack-cost discussion; the two
  open problems and the standard-model gap are stated explicitly. The core mathematical content is unchanged from v3.2.
  **v3.2.4** is a focused cross-review revision of the §9 linear-reduction
  transport theorems: it adds a fully-proven random-Gram corank lemma (first-moment isotropic-subspace
  count, replacing an enumeration-only argument), corrects the near-full-rank distinguisher's parameter
  regime (the alternating output Gram has even rank, so the rank test is vacuous at the odd-$n$ boundary;
  the non-vacuous regime is $n-2c\ge 2$), promotes the sample-freshness statistical-distance identity to a
  numbered proposition (separating the same-secret and rerandomized-secret comparisons), and fixes minor
  constants (a Gaussian-binomial bound, a reachability exponent). Every new or corrected claim is
  exact-verified by GF(2) enumeration in SageMath. The exact SQ lower bounds and
  the assumption posture are unchanged; the revisions are confined to the transport theorems of §9 and
  their supporting lemma.
  **v3.2.5** is a referee-hardening polish pass (no change to the headline SQ results or the
  open-assumption posture), verified across four independent checks (three AI engines plus exact
  GF(2)/SageMath computation): the statistical-query parameter tables are reframed as target SQ
  exponents rather than bit-security levels; the random-Gram lemma's symmetric case is stated as an
  upper bound with the orthogonal polar-space count; the constant-rate-LPN hardness floor is written
  as a formal four-step proof; the sample-freshness identity's noise range is confirmed exact across
  $p\in(0,\tfrac12)$ by enumeration; the valid-output classification is promoted to a numbered
  proposition; and a companion-paper citation initial is corrected. Each numerical and exact claim is
  reproducible from the verification scripts in `experiments/`.
  **v3.2.6** is a further referee-hardening revision (headline SQ results and open-assumption posture
  unchanged): the pencil-extremality discussion now separates the unconstrained Delsarte LP (which yields
  only the trivial bound) from the size-constrained LP, which reproduces the exact pencil ratio for
  $n\le 8$ but requires all Delsarte levels (an all-or-nothing certificate, open for all $n$); and the
  distribution-mismatch (open-cell) discussion adds a full statistical-distance truth-test for the
  confined-noise generator. Verified across four independent agent checks plus exact GF(2)/SageMath
  computation.
  **v3.2.7** is an open-problem hardening pass (headline results unchanged): it adds a scalar-method
  obstruction theorem for the pencil-extremality conjecture — a nonnegative self-dual multiset satisfies
  every scalar, support, and self-dual constraint yet has unbounded degree ratio, so no scalar/self-dual
  functional inequality can prove the conjecture; the essential missing input is a distinctness/projection
  bound, exactly what the size-constrained LP encodes. The marginal-adaptive open-cell discussion is
  likewise sharpened (the min-syndrome-weight over-dispersion decreases toward $1$, so that scalar
  statistic cannot by itself close the conjecture). All cited theorems were re-verified against the
  primary-source PDFs before release.
- Open research: the randomized marginal-adaptive corner of the linear-reduction landscape, and
  the membership↔stabilizer-decoding bridge. Both are stated as open problems in the paper, with
  named obstructions that block the natural maps but do **not** constitute impossibility results.
- Implementation: Python prototypes validate parameters and algorithms; Rust reference
  implementations (`impl/lsn_ref/`, `impl/lsn_cryptanalysis/`, `impl/polar_validation/`)
  are active. A production constant-time Rust implementation with full N=2048 validation
  and KAT generation is in progress.

## License

Dual-licensed:

- **Code** (`experiments/`, `impl/`, and future implementations): [Apache License 2.0](LICENSE) — permissive, with an explicit patent grant and retaliation clause.
- **Documents** (`paper/`, this README): [CC BY 4.0](LICENSE-docs) — free to share and adapt with attribution.

See `NOTICE` for the attribution line. No patents are or will be sought on the LSN constructions by the author; the scheme is offered royalty-free to maximize community analysis and adoption.
