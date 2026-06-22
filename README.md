# LSN-PQ

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.20805781.svg)](https://doi.org/10.5281/zenodo.20805781)

**The Lagrangian Subspace Noise (LSN) Problem**

Author: Kwanghoo Choo (ORCID [0009-0005-5682-8098](https://orcid.org/0009-0005-5682-8098))

LSN is a *candidate* new post-quantum hardness assumption: an LPN variant whose secret is a
Lagrangian subspace of a symplectic vector space over F₂. This repository contains the papers,
the verification experiments, and the supporting Rust implementations.

**Papers** (in `paper/`):
- `lsn-core.pdf` — *Lagrangian Subspace Noise: Exact Statistical-Query Lower Bounds and the
  Linear-Reduction Landscape* (the core mathematical paper, **v3.3.1, 52 pp — the current paper**).
- `lsn-paper.pdf` — *Lagrangian Subspace Noise: Cryptographic Constructions* — the companion
  constructions paper (LSN-SNARK; a KEM is left to future work). It builds on `lsn-core.pdf` and cites it for the
  hardness analysis (SQ lower bounds, the reduction landscape, parameters); it contains no
  separate hardness claims.

**Cite this archive**: DOI [10.5281/zenodo.20805781](https://doi.org/10.5281/zenodo.20805781)
(concept DOI — always resolves to the latest version, currently **v3.3.1**). The earlier version
history (`v2.0`–`v3.2.7`) is archived separately under DOI
[10.5281/zenodo.20646796](https://doi.org/10.5281/zenodo.20646796).

## Claims posture (read this first)

Every claim in the paper is classified as **theorem / evidence / conjecture**.
In particular: no "7th family" is claimed as proven; LSN hardness is an OPEN assumption
supported by exact statistical-query lower bounds and a near-complete linear-reduction
barrier landscape. The public-matrix, fixed, and conditionally-uniform adaptive cells are
closed unconditionally, as are the confined-noise and near-uniform diffuse strategies and
the deterministic half of the fourth cell; a single precisely-stated residual (randomized
marginal-adaptive) remains open. As of v3.3.1 the $2^{2n}$ SQ bound for the conditionally-uniform
cell is **unconditional** (an absolute constant), via the big-cell Delsarte square-function bound
and the averaging transfer; a sharp constant-5 pencil ratio and the marginal-adaptive residual
remain the labeled conditional and open items. No production-security claim is made.

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

- **Paper**: the core paper (`lsn-core.pdf`) is at **v3.3.1** (52 pp). Its headline result is an
  **unconditional** statistical-query lower bound for the conditionally-uniform adaptive cell:
  v3.3.1 closes the averaging step of the pencil-extremality argument — the big-cell Delsarte
  square-function bound and the averaging transfer are stated as theorems, upgrading the previously
  conditional $2^{2n}$ SQ bound to an absolute constant. An explicit constant dual certificate
  (appendix) reproduces the big-cell bound mechanically, verified exactly in rational arithmetic for
  $n = 4,\dots,32$. A submission to IACR Communications in Cryptology (CiC) is planned.
- **Open research**: the randomized marginal-adaptive corner of the linear-reduction landscape, and
  the membership↔stabilizer-decoding bridge. Both are stated as open problems in the paper, with
  named obstructions that block the natural maps but do **not** constitute impossibility results.
- **Implementation**: Python prototypes validate parameters and algorithms; Rust reference
  implementations (`impl/lsn_ref/`, `impl/lsn_cryptanalysis/`, `impl/polar_validation/`)
  are active. A production constant-time Rust implementation with full N=2048 validation
  and KAT generation is in progress.

## License

Dual-licensed:

- **Code** (`experiments/`, `impl/`, and future implementations): [Apache License 2.0](LICENSE) — permissive, with an explicit patent grant and retaliation clause.
- **Documents** (`paper/`, this README): [CC BY 4.0](LICENSE-docs) — free to share and adapt with attribution.

See `NOTICE` for the attribution line. No patents are or will be sought on the LSN constructions by the author; the scheme is offered royalty-free to maximize community analysis and adoption.
