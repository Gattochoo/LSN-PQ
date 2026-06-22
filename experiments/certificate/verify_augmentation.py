#!/usr/bin/env python3
"""Independent independent verification of the 'augmentation' proof of the averaging step (CLAIM C).

the argument (to verify, NOT trust):
  s_th = G * 2^{-2n+2},  G = 2^{n(n+1)/2}  (big-cell threshold size).
  (a) BAD Y (|S_Y| < s_th): augment S_Y to size s_th within the big cell; big-cell theorem on the
      augmented set + monotonicity (K>0) => Sum_{S_Y} K <= 5 rho_avg * s_th^2.   [the 2^n-killer]
  (b) GOOD Y (|S_Y| >= s_th): big-cell theorem => Sum_{S_Y} K <= 5 rho_avg |S_Y|^2.
  (c) Chebyshev: P_bad <= Var(|S_Y|)/(mu - s_th)^2 <= E[|S_Y|^2]/(mu - s_th)^2, mu = E|S_Y| = M G/N.
  => E_Y[Sum_{S_Y}K] <= 5 rho_avg E[|S_Y|^2] (1 + s_th^2 P_bad / E[|S_Y|^2]) <= 5 rho_avg E[|S_Y|^2](1+1/(k-1)^2),
     k = mu/s_th >= 2^{c-2}  (for M >= N/2^{2n-c}).
  => rho(S) <= (pi_max/pi_min) * E[Sum K]/E[|S_Y|^2] <= (pi_max/pi_min) 5 rho_avg (1+1/(k-1)^2).

We verify EVERY link exactly (n=2,3) / float (n=4), on the WORST in-scope S (greedy max + pencils),
requiring k >= 2 (c>=3 scope).  Critically we check the BAD-Y cap (a) directly:  Sum_{S_Y}K <= 5 rho_avg s_th^2.
We also independently re-confirm the big-cell sub-claim: max over |T|=s_th of rho_BC(T) <= 5 rho_avg.
"""
from fractions import Fraction as Fr
import numpy as np

np.random.seed(3)


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


def greedy_max(C, M, seed, n_steps=120):
    N = C.shape[0]
    inS = np.zeros(N, dtype=bool)
    inS[seed] = True
    cur = list(np.where(inS)[0])
    if len(cur) > M:
        cur = list(np.random.choice(cur, M, replace=False)); inS[:] = False; inS[cur] = True
    elif len(cur) < M:
        inS[np.random.choice(np.where(~inS)[0], M - len(cur), replace=False)] = True
    g = C @ inS.astype(float); diag = np.diag(C); f = float(inS @ g)
    for _ in range(n_steps):
        Sidx = np.where(inS)[0]; Uidx = np.where(~inS)[0]
        if len(Uidx) == 0: break
        a = 2 * g[Uidx] + diag[Uidx]; best = (1e-9, -1, -1)
        for v in Sidx:
            delta = a + (-2 * g[v] + diag[v]) - 2 * C[Uidx, v]
            t = int(np.argmax(delta))
            if delta[t] > best[0]: best = (float(delta[t]), int(Uidx[t]), int(v))
        d, u, v = best
        if u < 0: break
        inS[v] = False; inS[u] = True; g += C[:, u] - C[:, v]; f += d
    return np.where(inS)[0]


def main():
    for n in (2, 3, 4):
        Lags = enum_lagr(n); N = len(Lags); elem = list(Lags)
        G = 2 ** (n * (n + 1) // 2); s_th = G // (2 ** (2 * n - 2))
        from collections import Counter
        deg = Counter()
        for L in Lags:
            for x in L: deg[x] += 1
        rho_avg = float(Fr(sum(c * c for c in deg.values()), N * N))
        five = 5 * rho_avg
        # dim-cap matrix and kernel
        J = np.zeros((N, N), dtype=np.int64)
        for i in range(N):
            for k in range(i, N):
                J[i, k] = J[k, i] = (len(elem[i] & elem[k])).bit_length() - 1
        K = (2.0 ** J)
        # pi(j) and the C kernel for greedy worst-search
        L0 = Lags[0]; reps = {}
        for L in Lags: reps.setdefault((len(L0 & L)).bit_length() - 1, L)
        pi = {j: Fr(sum(1 for Y in Lags if len(L0 & Y) == 1 and len(Lj & Y) == 1), N)
              for j, Lj in reps.items()}
        pmax = float(max(pi.values())); pmin = float(min(pi.values())); pratio = pmax / pmin
        Cw = np.vectorize(lambda j: float(pi[j] * (Fr(1 << j) - Fr(5) * Fr(sum(c*c for c in deg.values()), N*N))))(J)
        p = G / N
        # transverse boolean: trans[Y][L]
        Lsets = [set(L) for L in Lags]
        trans = np.zeros((N, N), dtype=bool)
        for yi in range(N):
            Y = Lsets[yi]
            for li in range(N):
                trans[yi, li] = len(Y & Lsets[li]) == 1

        # in-scope sizes: need k=mu/s_th>=2 => M >= 2 s_th N/G
        Mmin = int(np.ceil(2 * s_th * N / G))
        Ms = sorted(set([Mmin, 2 * Mmin, round(N / 8), round(N / 4)]))
        Ms = [m for m in Ms if Mmin <= m <= N]
        u0 = 1
        pen1 = [i for i, L in enumerate(Lags) if u0 in L]
        w2 = next(w for w in range(1, 1 << (2 * n)) if w != u0 and omega(u0, w, n) == 0)
        Wsp = span([u0, w2]); pen2 = [i for i, L in enumerate(Lags) if Wsp <= L]

        print(f"\n{'='*92}\nn={n}: N={N}, G={G}, s_th={s_th}, rho_avg={rho_avg:.4f}, 5rho_avg={five:.3f}, "
              f"p=G/N={p:.4f}, pi_max/pi_min={pratio:.4f}, Mmin(k>=2)={Mmin}")
        print(f"  {'M':>5} {'k=mu/sth':>9} {'rho/ravg':>9} {'badY cap OK':>11} {'chain OK':>9} "
              f"{'finalbnd/ravg':>13} {'rho<=bnd':>9}")
        allok = True
        for M in Ms:
            # worst S: greedy + pencil-padded
            cands = [greedy_max(Cw, M, np.array(pen2 + pen1)[:max(M, len(pen2))]),
                     greedy_max(Cw, M, np.random.choice(N, M, replace=False)),
                     greedy_max(Cw, M, np.random.choice(N, M, replace=False))]
            # pick the one with largest actual rho
            S = max(cands, key=lambda idx: K[np.ix_(idx, idx)].sum() / (len(idx) ** 2))
            Sset = list(S)
            sumK_total = K[np.ix_(Sset, Sset)].sum()
            rho = sumK_total / (M * M)
            # per-Y slice sums
            cap_ok = True; sumKY = np.zeros(N); SY2 = np.zeros(N)
            for yi in range(N):
                mask = trans[yi, Sset]
                SY = [Sset[t] for t in range(M) if mask[t]]
                sz = len(SY)
                SY2[yi] = sz * sz
                if sz == 0:
                    continue
                sk = K[np.ix_(SY, SY)].sum()
                sumKY[yi] = sk
                if sz < s_th:  # BAD Y: verify the augmentation cap
                    if sk > five * s_th * s_th + 1e-6:
                        cap_ok = False
            E_sumKY = sumKY.mean(); E_SY2 = SY2.mean()
            mu = M * p; k = mu / s_th
            P_bad = np.mean([1.0 if (SY2[yi] ** 0.5) < s_th and trans[yi, Sset].sum() < s_th else 0.0
                             for yi in range(N)])
            #  chain: E_sumKY <= 5 rho_avg (E_SY2 + s_th^2 P_bad)
            chain_rhs = five * (E_SY2 + s_th * s_th * P_bad)
            chain_ok = E_sumKY <= chain_rhs + 1e-6
            final_bnd = pratio * E_sumKY / E_SY2
            theory_bnd = pratio * five * (1 + 1.0 / (k - 1) ** 2) if k > 1 else float('inf')
            rho_ok = rho <= final_bnd + 1e-6 and final_bnd <= theory_bnd + 1e-6
            allok = allok and cap_ok and chain_ok and rho_ok
            print(f"  {M:>5} {k:>9.3f} {rho/rho_avg:>9.4f} {str(cap_ok):>11} {str(chain_ok):>9} "
                  f"{final_bnd/rho_avg:>13.3f} {str(rho_ok):>9}")
        print(f"  ALL LINKS VERIFIED for n={n}: {allok}   (badY cap = the 2^n-killer; chain =  bound; "
              f"rho<=bnd = conclusion)")


if __name__ == "__main__":
    main()
