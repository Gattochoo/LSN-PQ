#!/usr/bin/env python3
"""SEAL Proof-of-(PI): the geometric reduction behind pi(j), proven + verified n<=5.

CLAIM (clean symplectic linear algebra, all n): in a W-adapted symplectic basis with
  L  = <e_1..e_n>                       (e_i = bit i,  f_i = bit i+n; omega(e_i,f_i)=1)
  L' = <e_1..e_j, f_{j+1}..f_n>          (dim(L cap L')=j, L' Lagrangian)
every Lagrangian Y transverse to L is the graph Y_A = <f_i + sum_k A_{ik} e_k> of a unique
SYMMETRIC A in Sym_n(F_2), and
  Y_A cap L' = 0   <=>   the bottom-right (n-j)x(n-j) block A[j:,j:] is nonsingular over F_2.
Hence #{Y transverse to both} = (#A with A[j:,j:] nonsingular) = 2^{(free)} * N_sym(n-j),
so pi(j) = (G/N) * frac_nonsing(n-j),  G=2^{n(n+1)/2}, N=prod(2^i+1).

This script verifies, for n=2..5 and every j, by EXHAUSTIVE enumeration of A in Sym_n (cheap:
2^{n(n+1)/2} <= 2^15) that the two conditions (Y_A cap L' = 0)  and  (A[j:,j:] nonsingular)
agree for EVERY A, and that the resulting count equals G*frac_nonsing(n-j).  (No Lagrangian
enumeration needed -- only Sym_n -- so n=5 is fast.)
"""
import numpy as np
import itertools


def omega(u, v, n):
    s = 0
    for i in range(n):
        s ^= (((u >> i) & 1) & ((v >> (i + n)) & 1)) ^ (((u >> (i + n)) & 1) & ((v >> i) & 1))
    return s


def span(basis):
    s = {0}
    for b in basis:
        s |= {x ^ b for x in s}
    return s


def rank_f2(M):
    M = [row[:] for row in M]
    rows = len(M); cols = len(M[0]) if rows else 0
    r = 0; pr = 0
    for c in range(cols):
        piv = None
        for i in range(pr, rows):
            if M[i][c]:
                piv = i; break
        if piv is None:
            continue
        M[pr], M[piv] = M[piv], M[pr]
        for i in range(rows):
            if i != pr and M[i][c]:
                M[i] = [a ^ b for a, b in zip(M[i], M[pr])]
        pr += 1; r += 1
    return r


def frac_nonsing_formula(m):
    from fractions import Fraction as Fr
    p = Fr(1)
    for i in range(1, -(-m // 2) + 1):  # i=1..ceil(m/2)
        p *= (1 - Fr(1, 2 ** (2 * i - 1)))
    return p


def main():
    from fractions import Fraction as Fr
    triu = lambda n: [(i, k) for i in range(n) for k in range(i, n)]
    Nlag = lambda n: int(np.prod([2 ** i + 1 for i in range(1, n + 1)]))
    print(f"  {'n':>2} {'j':>2} {'reduction holds (all A)':>24} {'count==G*frac':>14} {'pi(j) ok':>9}")
    all_ok = True
    for n in range(2, 6):
        G = 1 << (n * (n + 1) // 2)
        N = Nlag(n)
        tlist = triu(n)
        # L = bits 0..n-1
        Lset = span([1 << i for i in range(n)])
        for j in range(0, n + 1):
            # L' = <e_0..e_{j-1}, f_j..f_{n-1}>  (bits 0..j-1  and  j+n..2n-1)
            Lp_basis = [1 << a for a in range(j)] + [1 << (b + n) for b in range(j, n)]
            Lpset = span(Lp_basis)
            mism = 0
            cnt_transverse = 0
            cnt_nonsing = 0
            for bits in range(1 << len(tlist)):
                # build symmetric A
                A = [[0] * n for _ in range(n)]
                for idx, (a, b) in enumerate(tlist):
                    if (bits >> idx) & 1:
                        A[a][b] = A[b][a] = 1
                # Y_A basis: f_i + sum_k A[i][k] e_k  = bit(i+n) ^ sum_k A[i][k] bit(k)
                Yb = []
                for i in range(n):
                    v = 1 << (i + n)
                    for k in range(n):
                        if A[i][k]:
                            v ^= (1 << k)
                    Yb.append(v)
                Yset = span(Yb)
                transverse = (len(Yset & Lpset) == 1)  # Y cap L' = {0}
                # bottom-right (n-j)x(n-j) block A[j:,j:]
                if n - j == 0:
                    nonsing = True
                else:
                    blk = [[A[a][b] for b in range(j, n)] for a in range(j, n)]
                    nonsing = (rank_f2(blk) == n - j)
                if transverse != nonsing:
                    mism += 1
                cnt_transverse += transverse
                cnt_nonsing += nonsing
            red_ok = (mism == 0)
            # count check: cnt_transverse == G * frac_nonsing(n-j)
            expect = Fr(G) * frac_nonsing_formula(n - j)
            cnt_ok = (Fr(cnt_transverse) == expect)
            # pi(j) = cnt_transverse / N  vs  (G/N)*frac_nonsing(n-j)
            pi_ok = (Fr(cnt_transverse, N) == Fr(G, N) * frac_nonsing_formula(n - j))
            all_ok = all_ok and red_ok and cnt_ok and pi_ok
            print(f"  {n:>2} {j:>2} {('YES (mism=0)' if red_ok else f'NO ({mism})'):>24} "
                  f"{str(cnt_ok):>14} {str(pi_ok):>9}")
    print(f"\n  ==> geometric reduction (transverse <=> bottom-right block nonsingular) and "
          f"pi(j)=(G/N)frac_nonsing(n-j) VERIFIED for all n=2..5, all j: {all_ok}")
    print("  (proof is all-n linear algebra; this is the exhaustive small-case confirmation, "
          "incl n=5 which needs only Sym_5=32768, no Lagrangian enumeration)")


if __name__ == "__main__":
    main()
