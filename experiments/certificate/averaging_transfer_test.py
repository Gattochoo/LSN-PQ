#!/usr/bin/env python3
"""DECISIVE TEST of the conj:pencil AVERAGING STEP (big cell -> all Lagrangians).

Independent independent verification.  conj:pencil = ratio rho(S)/rho_avg <= 5 for
large S, where rho(S) = |S|^{-2} sum_{L,L' in S} K(L,L'),  K(L,L') = 2^{dim(L cap L')}.

Paper line 545 claims a "verified averaging step": pick a random Lagrangian Y,
restrict to the big cell  S_Y = { L in S: L cap Y = 0 }  (Y-transverse), where the
big-cell theorem gives rho_BC(S_Y) <= 5 rho_avg, then average over Y.

My averaging argument (derived independently):
  pi(j):= Pr_Y[ L, L' both Y-transverse | dim(L cap L') = j ]   (depends only on j)
  E_Y[ sum_{L,L' in S_Y} K ]  =  sum_{L,L' in S} pi(j_{LL'}) K(L,L')           (exact)
  E_Y[ |S_Y|^2 ]              =  sum_{L,L' in S} pi(j_{LL'})                    (exact)
  pi(j) in [pi_min, pi_max].
  IF   sum pi(j)K  <=  5 rho_avg * sum pi(j)        (CLAIM C)
  THEN rho_Lagr(S) = sumK/M^2 <= 5 rho_avg * (pi_max/pi_min).               (averaging done)

CLAIM C is the big-cell theorem applied to *every* Y and re-summed with |S_Y|^2 weights.
It can only FAIL on bad Y where |S_Y| is below the big-cell density threshold
(2^{-2n+2}) and rho_BC(S_Y) overshoots 5 rho_avg with non-trivial |S_Y|^2 weight.

THIS SCRIPT decides whether CLAIM C holds for junta/near-pencil S (the suspected gap):
  - exact pi(j) by enumerating Y for a representative pair at each j
  - exact LHS=sum pi K, RHS=5 rho_avg sum pi, and the slack RHS-LHS
  - direct E_Y enumeration cross-check (E_Y[sum_{S_Y}K] vs E_Y[|S_Y|^2])
  - bad-Y mass:  E_Y[ |S_Y|^2 * 1[density < 2^{-2n+2}] ]
  - the resulting averaging bound vs the true rho_Lagr(S)
Families: pure k=1 & k=2 pencils, near-2-pencil (paper line 543 obstruction),
global random-large.  All exact (Fraction).
"""
from fractions import Fraction as Fr
import random
import itertools

random.seed(31)


def omega(u, v, n):
    s = 0
    for i in range(n):
        s ^= (((u >> i) & 1) & ((v >> (i + n)) & 1)) ^ (((u >> (i + n)) & 1) & ((v >> i) & 1))
    return s


def span(bs):
    s = {0}
    for b in bs:
        s |= {x ^ b for x in s}
    return frozenset(s)


def enum_lagr(n):
    N2 = 2 * n
    cur = {frozenset([0, v]): (v,) for v in range(1, 1 << N2)}
    for k in range(2, n + 1):
        nxt = {}
        for fs, bs in cur.items():
            for w in range(1, 1 << N2):
                if w in fs:
                    continue
                if all(omega(b, w, n) == 0 for b in bs):
                    nb = bs + (w,)
                    nfs = span(nb)
                    if len(nfs) == (1 << k) and nfs not in nxt:
                        nxt[nfs] = nb
        cur = nxt
    return [fs for fs in cur]


def dimcap(A, B):
    return (len(A & B)).bit_length() - 1  # log2 |A cap B|


def main():
    for n in (2, 3, 4):
        Lags = enum_lagr(n)
        N = len(Lags)
        # true rho_avg = (1/N^2) sum_{L,L'} 2^{dim cap}
        # compute via degree vector over points (fast, exact):
        # sum_{L,L'} |L cap L'| = sum_x d(x)^2 with d(x)=#{L: x in L}
        from collections import Counter
        deg = Counter()
        for L in Lags:
            for x in L:
                deg[x] += 1
        sum_pair_K_all = sum(c * c for c in deg.values())  # = sum_{L,L'} 2^{dim cap}
        rho_avg = Fr(sum_pair_K_all, N * N)

        # pi(j): representative pair at each j, enumerate Y over all Lagr
        # build one representative pair (L0, Lj) with dim(L0 cap Lj)=j for each j=0..n
        L0 = Lags[0]
        reps = {}
        for L in Lags:
            j = dimcap(L0, L)
            if j not in reps:
                reps[j] = L
            if len(reps) == n + 1:
                break
        pi = {}
        for j, Lj in reps.items():
            cnt = 0
            for Y in Lags:
                if (len(L0 & Y) == 1) and (len(Lj & Y) == 1):  # both transverse (cap = {0})
                    cnt += 1
            pi[j] = Fr(cnt, N)
        pi_min = min(pi.values())
        pi_max = max(pi.values())

        # families
        u = 1
        pencil1 = [L for L in Lags if u in L]
        w2 = next(w for w in range(1, 1 << (2 * n)) if w != u and omega(u, w, n) == 0)
        W = span([u, w2])
        pencil2 = [L for L in Lags if W <= L]
        restL = [L for L in Lags if not (W <= L)]
        L0t = next(L for L in restL if len(L & W) == 1)  # a Lagrangian transverse to W
        near2 = pencil2 + [L0t] + random.sample([L for L in restL if L is not L0t],
                                                min(len(restL) - 1, max(1, len(pencil2))))
        szL = max(4, round(N / 2 ** (2 * n)) * 4)
        glob = random.sample(Lags, min(N, szL))
        fams = {
            "pencil-k1": pencil1,
            "pencil-k2": pencil2,
            "near-2-pencil": near2,
            f"global-random(M~{szL})": glob,
        }

        thr_density = Fr(1, 2 ** (2 * n - 2))  # big-cell density threshold 2^{-2n+2}
        Gbig = None  # |big cell| for a fixed Y = #{L: L cap Y = {0}}

        print(f"\n{'='*96}\nn={n}: N={N}, rho_avg={float(rho_avg):.5f}  pi(j)=" +
              ", ".join(f"{j}:{float(pi[j]):.4f}" for j in sorted(pi)) +
              f"  pi_max/pi_min={float(pi_max/pi_min):.4f}")
        print(f"  averaging const loss 5*pi_max/pi_min = {float(5*pi_max/pi_min):.3f} (x rho_avg)")
        print(f"  {'family':22} {'M':>5} {'rhoLagr/avg':>11} {'CLAIM C slack':>14} "
              f"{'badY mass/M^2':>13} {'avg-bound/avg':>13} {'holds?':>7}")

        for name, S in fams.items():
            M = len(S)
            Sset = [frozenset(L) for L in S]
            # pairwise K and dim
            sumK = 0
            LHS = Fr(0)   # sum pi(j) K
            sumpi = Fr(0)  # sum pi(j)
            for a in range(M):
                for b in range(M):
                    j = dimcap(Sset[a], Sset[b])
                    K = 1 << j
                    sumK += K
                    LHS += pi[j] * K
                    sumpi += pi[j]
            rhoLagr = Fr(sumK, M * M)
            RHS = 5 * rho_avg * sumpi
            claimC_slack = RHS - LHS  # >=0 means CLAIM C holds
            # direct E_Y enumeration: bad-Y mass and rho_BC overshoot
            badmass = Fr(0)       # sum_Y |S_Y|^2 * 1[bad] / N
            EY_SY2 = Fr(0)        # sum_Y |S_Y|^2 / N  (== sumpi, cross-check)
            EY_sumK = Fr(0)       # sum_Y sum_{S_Y} K / N (== LHS, cross-check)
            for Y in Lags:
                SY = [L for L in Sset if len(L & Y) == 1]
                gsize = len(SY)
                if gsize == 0:
                    continue
                sK = 0
                for a in range(gsize):
                    for b in range(gsize):
                        sK += 1 << dimcap(SY[a], SY[b])
                EY_sumK += Fr(sK)
                EY_SY2 += Fr(gsize * gsize)
                if Gbig is None:
                    Gbig = sum(1 for L in Lags if len(L & Y) == 1)
                dens = Fr(gsize, Gbig)
                if dens < thr_density:
                    badmass += Fr(gsize * gsize)
            EY_sumK /= N
            EY_SY2 /= N
            badmass /= N
            # cross-check exact identities
            assert EY_SY2 == sumpi, (name, EY_SY2, sumpi)
            assert EY_sumK == LHS, (name, EY_sumK, LHS)
            avg_bound = 5 * rho_avg * (pi_max / pi_min)  # the certified bound (if CLAIM C held)
            holds = "YES" if claimC_slack >= 0 else "NO"
            print(f"  {name:22} {M:>5} {float(rhoLagr/rho_avg):>11.4f} "
                  f"{float(claimC_slack):>14.4f} {float(badmass/(M*M)):>13.5f} "
                  f"{float(avg_bound/rho_avg):>13.3f} {holds:>7}")

        print("  [CLAIM C slack >=0  => averaging works for this S without decomposition;"
              " <0 => bad-Y gap is real]")


if __name__ == "__main__":
    main()
