#!/usr/bin/env python3
"""MAKE-OR-BREAK: can any in-scope subset S BREAK CLAIM C?
CLAIM C <=> f(S):= 1_S^T C 1_S <= 0,  C(L,L')=c_{dim(L cap L')}, c_j=pi(j)(2^j-5 rho_avg).
If f(S)>0 for some S with |S| at/above the conj:pencil (k<=2 pencil) scale, the averaging-via-CLAIM-C
route DIES.  If f(S)<=0 across an honest worst-case search, that's strong EVIDENCE the route is sound.

n=2: EXACT max over ALL subsets of each size (N=15, exhaustive feasible).
n=3,4: greedy local-search (structured seeds: pencils, near-pencils, pencil-unions + random restarts)
       maximizing f(S) at fixed |S|, swept over M from the k<=2 pencil scale upward.
Reports max f(S) per M and the actual rho(S)/rho_avg of the worst S; flags any f>0.
"""
from fractions import Fraction as Fr
from itertools import combinations
import numpy as np

np.random.seed(7)


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


def build(n):
    Lags = enum_lagr(n)
    N = len(Lags)
    from collections import Counter
    deg = Counter()
    for L in Lags:
        for x in L:
            deg[x] += 1
    rho_avg = Fr(sum(c * c for c in deg.values()), N * N)
    # pairwise dim matrix
    J = np.zeros((N, N), dtype=np.int8)
    elem = [L for L in Lags]
    for i in range(N):
        Li = elem[i]
        for k in range(i, N):
            j = (len(Li & elem[k])).bit_length() - 1
            J[i, k] = j
            J[k, i] = j
    # pi(j)
    L0 = Lags[0]
    reps = {}
    for L in Lags:
        reps.setdefault((len(L0 & L)).bit_length() - 1, L)
    pi = {}
    for j, Lj in reps.items():
        cnt = sum(1 for Y in Lags if len(L0 & Y) == 1 and len(Lj & Y) == 1)
        pi[j] = Fr(cnt, N)
    cvec = {j: pi[j] * (Fr(1 << j) - 5 * rho_avg) for j in pi}
    C = np.array([[float(cvec[J[i, k]]) for k in range(N)] for i in range(N)])
    K = (2.0 ** J)  # kernel for rho
    return Lags, N, rho_avg, pi, cvec, C, K, J


def rho_ratio(idx, K, rho_avg):
    sub = K[np.ix_(idx, idx)]
    M = len(idx)
    return float(Fr(int(round(sub.sum())) if False else 0)) if False else (sub.sum() / M / M) / float(rho_avg)


def greedy_max(C, M, seed_idx, n_steps=200):
    """local search maximizing x^T C x at fixed support size M, starting from seed_idx."""
    N = C.shape[0]
    inS = np.zeros(N, dtype=bool)
    inS[seed_idx] = True
    if inS.sum() != M:  # adjust to size M
        cur = list(np.where(inS)[0])
        if len(cur) > M:
            cur = list(np.random.choice(cur, M, replace=False))
            inS[:] = False
            inS[cur] = True
        else:
            add = list(np.random.choice(np.where(~inS)[0], M - len(cur), replace=False))
            inS[add] = True
    g = C @ inS.astype(float)
    diag = np.diag(C)
    f = float(inS.astype(float) @ g)
    for _ in range(n_steps):
        Sidx = np.where(inS)[0]
        Uidx = np.where(~inS)[0]
        a = 2 * g[Uidx] + diag[Uidx]          # add-score over u not in S
        best = (0.0, -1, -1)
        for v in Sidx:
            rv = -2 * g[v] + diag[v]
            delta = a + rv - 2 * C[Uidx, v]    # delta f for swap (u<-v)
            t = int(np.argmax(delta))
            if delta[t] > best[0] + 1e-12:
                best = (float(delta[t]), int(Uidx[t]), int(v))
        d, u, v = best
        if u < 0:
            break
        inS[v] = False
        inS[u] = True
        g += C[:, u] - C[:, v]
        f += d
    return f, np.where(inS)[0]


def main():
    for n in (2, 3, 4):
        Lags, N, rho_avg, pi, cvec, C, K, J = build(n)
        # k<=2 pencil scale
        M2 = 1
        for i in range(1, n - 1):
            M2 *= (2 ** i + 1)  # |Lagr(2(n-2),2)|
        thr_c2 = max(2, -(-N // (2 ** (2 * n - 2))))  # ceil(N/2^{2n-2})
        print(f"\n{'='*84}\nn={n}: N={N}, rho_avg={float(rho_avg):.5f}, k=2 pencil size M2={M2}, "
              f"thr(c=2)={thr_c2}")
        print(f"  c_j = " + ", ".join(f"{j}:{float(cvec[j]):+.4f}" for j in sorted(cvec)))

        if n == 2:
            print("  EXACT max f(S) over ALL subsets per size M:")
            for M in range(2, N + 1):
                best = -1e9
                bestidx = None
                for comb in combinations(range(N), M):
                    idx = np.array(comb)
                    f = float(idx_q(C, idx))
                    if f > best:
                        best = f
                        bestidx = idx
                r = (K[np.ix_(bestidx, bestidx)].sum() / M / M) / float(rho_avg)
                flag = "  <<< CLAIM C BROKEN" if best > 1e-9 else ""
                print(f"    M={M:>3}: max f={best:+.5f}  rho/avg(worst)={r:.4f}{flag}")
        else:
            Ms = sorted(set([M2, max(M2, thr_c2), thr_c2, 2 * thr_c2,
                             round(N / 16), round(N / 8), round(N / 4)]))
            Ms = [m for m in Ms if 2 <= m <= N]
            # structured seeds
            u0 = 1
            pen1 = [i for i, L in enumerate(Lags) if u0 in L]
            w2 = next(w for w in range(1, 1 << (2 * n)) if w != u0 and omega(u0, w, n) == 0)
            W = span([u0, w2])
            pen2 = [i for i, L in enumerate(Lags) if W <= L]
            seeds_base = {"pencil1": pen1, "pencil2": pen2}
            print("  greedy worst-case max f(S) (structured seeds + random restarts):")
            for M in Ms:
                bestf = -1e9
                bestidx = None
                seeds = []
                for nm, s in seeds_base.items():
                    seeds.append(np.array(s))
                # pencil2 + extras; union of pencils
                seeds.append(np.array(pen2 + pen1)[:max(M, len(pen2))])
                for _ in range(8):
                    seeds.append(np.random.choice(N, min(M, N), replace=False))
                for sd in seeds:
                    f, idx = greedy_max(C, M, sd, n_steps=120)
                    if f > bestf:
                        bestf = f
                        bestidx = idx
                r = (K[np.ix_(bestidx, bestidx)].sum() / M / M) / float(rho_avg)
                flag = "  <<< CLAIM C BROKEN" if bestf > 1e-9 else ""
                print(f"    M={M:>4}: max f={bestf:+.4f}  rho/avg(worst)={r:.4f}{flag}")
        print("  [max f<=0 at all in-scope M => CLAIM C survives; f>0 => averaging-via-CLAIM-C route dies]")


def idx_q(C, idx):
    sub = C[np.ix_(idx, idx)]
    return sub.sum()


if __name__ == "__main__":
    main()
