"""Exact Delsarte dual-feasibility gate for the symmetric-bilinear-forms scheme.

Checks the FULL dual feasibility exactly, including the (0,1)/membership row and the objective value.
The membership row is what separates a genuine certificate from a per-relation-only one: a dual that
satisfies every individual relation can still fail the (0,1) row and blow up the objective value.

gate(m, lam, mu, eta, B): eta is a dict {class: Fraction} with eta_i <= 0 (Delsarte dual sign).
  F(k) = lam + mu*[k==(0,1)] + sum_i eta_i P^m[i,k]    (i over ALL classes incl (0,1))
  feasible iff F(k) >= 2^{m-rank(k)} for EVERY class k (incl (0,1)) AND eta_i <= 0.
  value = lam + mu/thr; proves rho_max(m) <= value (weak duality). Need value <= B.
Returns dict with feasible, value, value_le_B, worst (class, slack), eta_sign_ok.

Usage for a claimed all-m proof: provide a CONSTRUCTOR m -> (lam,mu,eta) in CLOSED FORM, run gate over a
range of m exactly; if any m fails, the proof is wrong. Passing finite m is necessary, not sufficient (the
general-m algebra still needs proof) -- but it is the rail that catches composition/(0,1) errors.
"""
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), 'lib'))
from schmidt_pmatrix import get_P, get_v, classes_of, solve_lp
from fractions import Fraction as Fr


def thr_of(m):
    N = m * (m + 1) // 2
    return 1 << (N - 2 * m + 2)


def gate(m, lam, mu, eta, B, tol=Fr(0)):
    """eta: dict {class:Fraction}, eta_i<=0. Returns full verdict (exact)."""
    lam = Fr(lam); mu = Fr(mu)
    cls = classes_of(m)
    eta_full = {c: Fr(eta.get(c, 0)) for c in cls}
    eta_sign_ok = all(eta_full[c] <= 0 for c in cls)
    worst = None
    feasible = True
    for k in cls:
        f = Fr(1 << (m - k[0]))
        F = lam + (mu if k == (0, 1) else Fr(0)) + sum(eta_full[i] * get_P(i, k, m) for i in cls)
        slack = F - f
        if slack < -tol:
            feasible = False
        if worst is None or slack < worst[1]:
            worst = (k, slack)
    value = lam + Fr(mu, thr_of(m))
    return {
        "feasible": feasible and eta_sign_ok,
        "F_geq_f": feasible,
        "eta_sign_ok": eta_sign_ok,
        "value": value,
        "value_le_B": value <= Fr(B),
        "worst_class": worst[0],
        "worst_slack": worst[1],
        "PROVES_rho_le_value": feasible and eta_sign_ok,  # weak duality: feasible dual => rho_max <= value
    }


def optimal_dual(m):
    """Extract the exact optimal dual (lam,mu,eta-dict) from the exact simplex (the ground-truth dual)."""
    b = solve_lp(m)
    y = b["dual_y"]
    cls = classes_of(m)
    mu, lam = y[0], y[1]
    eta_list = list(y[2:])
    eta = {cls[idx]: eta_list[idx] for idx in range(len(cls))}
    return lam, mu, eta, b["rho"]


def self_test():
    """sanity: the optimal dual must PASS the gate with value=rho_max (and rho_max<=13/2)."""
    print("self-test: optimal dual through the gate (must be feasible, value=rho_max<=13/2):")
    for m in range(2, 13):
        lam, mu, eta, rho = optimal_dual(m)
        v = gate(m, lam, mu, eta, Fr(13, 2))
        ok = v["feasible"] and v["value"] == rho and v["value_le_B"]
        print(f"  m={m}: feasible={v['feasible']} value={v['value']}=rho_max({rho}) <=13/2={v['value_le_B']}  "
              f"worst={v['worst_class']}:{v['worst_slack']}  -> gate-self-test {'OK' if ok else 'FAIL'}")


if __name__ == "__main__":
    self_test()
