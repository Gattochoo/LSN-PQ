#!/usr/bin/env python3
"""Seal (PI): pi(j) closed form via the nonsingular-symmetric-matrix count (the derivation).

the derivation: Y transverse to both L,L' (with dim(L cap L')=j) <=> the (n-j)x(n-j) bottom-right
principal submatrix of the symmetric matrix (in a W-adapted basis) is nonsingular.  Hence
  pi(j) = (G/N) * frac_nonsing(n-j),  frac_nonsing(m) = #{nonsingular symmetric m x m over F_2}/2^{m(m+1)/2}.
Claimed: frac_nonsing(m) = prod_{i=1}^{ceil(m/2)} (1 - 2^{-(2i-1)}).
This script (1) counts nonsingular symmetric matrices EXHAUSTIVELY for m=0..5 and checks the product
formula, (2) confirms R_inf = prod_{i>=1}(1-2^{-(2i-1)})^{-1} satisfies 298/125 < R_inf < 12/5 (the constant fix), so the averaging constant 10*R_inf < 24.
"""
from fractions import Fraction as Fr
import numpy as np
import itertools


def rank_f2_sym(bits, m):
    # build symmetric m x m from upper-tri bits, compute F_2 rank
    M = np.zeros((m, m), dtype=np.uint8)
    k = 0
    for i in range(m):
        for j in range(i, m):
            if (bits >> k) & 1:
                M[i, j] = M[j, i] = 1
            k += 1
    # gaussian elim
    M = M.copy(); r = 0; pr = 0
    for c in range(m):
        piv = None
        for i in range(pr, m):
            if M[i, c]:
                piv = i; break
        if piv is None:
            continue
        M[[pr, piv]] = M[[piv, pr]]
        for i in range(m):
            if i != pr and M[i, c]:
                M[i] ^= M[pr]
        pr += 1; r += 1
    return r


def prod_formula(m):
    p = Fr(1)
    for i in range(1, (m + 1) // 2 + (1 if m % 2 else 0) + 0):
        pass
    # ceil(m/2)
    import math
    for i in range(1, (m + 1) // 2 + (m % 2)):
        pass
    p = Fr(1)
    ceil_half = -(-m // 2)
    for i in range(1, ceil_half + 1):
        p *= (1 - Fr(1, 2 ** (2 * i - 1)))
    return p


def main():
    print("=== (1) nonsingular symmetric m x m over F_2: exhaustive count vs product formula ===")
    print(f"  {'m':>2} {'#nonsing':>10} {'2^(m(m+1)/2)':>13} {'fraction':>12} {'prod formula':>14} {'match':>6}")
    for m in range(0, 6):
        nb = m * (m + 1) // 2
        tot = 1 << nb
        cnt = sum(1 for bits in range(tot) if rank_f2_sym(bits, m) == m)
        frac = Fr(cnt, tot)
        pf = prod_formula(m)
        print(f"  {m:>2} {cnt:>10} {tot:>13} {str(frac):>12} {str(pf):>14} {str(frac == pf):>6}")

    print("\n=== (2) R_inf = prod_{i>=1} (1-2^{-(2i-1)})^{-1}: bounds ===")
    # partial products of the inverse
    inv = Fr(1)
    for i in range(1, 40):
        inv *= Fr(1) / (1 - Fr(1, 2 ** (2 * i - 1)))
    Rinf_approx = float(inv)
    print(f"  R_inf approx (40 terms) = {Rinf_approx:.8f}")
    print(f"  298/125 = {float(Fr(298,125)):.6f};  R_inf > 298/125 ? {inv > Fr(298,125)}")
    print(f"  12/5 = 2.4;  R_inf < 12/5 ? {inv < Fr(12,5)}")
    print(f"  => averaging constant 10*R_inf < 24 ? {10*inv < 24}")
    print("\n=== (3) pi(j)/p = frac_nonsing(n-j) reproduces the measured pi data (n<=4) ===")
    data = {2: [Fr(4,15), Fr(4,15), Fr(8,15)],
            3: [Fr(28,135), Fr(32,135), Fr(32,135), Fr(64,135)],
            4: [Fr(448,2295), Fr(448,2295), Fr(512,2295), Fr(512,2295), Fr(1024,2295)]}
    Nf = {2: 15, 3: 135, 4: 2295}
    for n in (2, 3, 4):
        G = 2 ** (n * (n + 1) // 2)
        ok = True
        for j in range(n + 1):
            pred = Fr(G, Nf[n]) * prod_formula(n - j)
            if pred != data[n][j]:
                ok = False
        print(f"  n={n}: pi(j) = (G/N)*frac_nonsing(n-j) matches measured data for all j? {ok}")


if __name__ == "__main__":
    main()
