"""Self-contained verification of the big-cell Delsarte certificate (Appendix; Theorem thm:bigcell).

Checks the closed-form constant dual --- lambda=8, eta_i = -5/2^(2m) on the support S, and
mu = 2^m - 8 + 5 V_m/4^m (V_m the total valency over S) --- against the FULL dual-feasibility
constraints (including the (0,1)/membership row), exactly in rational arithmetic, for m=4..32, and
verifies that the objective ratio against the Schmidt-cell average stays below 5 (-> 37/8 = 4.625).
"""
import sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, 'lib'))
sys.path.insert(0, HERE)
from schmidt_pmatrix import classes_of, get_v
from endtoend_gate import gate, thr_of
from fractions import Fraction as Fr


def support(m):
    """S = {(4,-)} U {(2a,+-) : a>=3} U {(2b+1,0) : b>=2}, in (rank, type) labelling."""
    h = m // 2
    S = [(4, -1)]
    for a in range(3, h + 1):
        S += [(2 * a, 1), (2 * a, -1)]
    for b in range(2, (m - 1) // 2 + 1):
        S.append((2 * b + 1, 0))
    return S


def main():
    allok = True
    for m in range(4, 33):
        S = support(m)
        c = Fr(-5, 1 << (2 * m))
        lam = Fr(8)
        Vm = sum(get_v(i, m) for i in S)
        mu = (1 << m) - 8 - c * Vm  # = 2^m - 8 + 5 V_m / 4^m
        eta = {i: c for i in S}
        rho_avg = Fr((1 << (m + 1)) - 1, 1 << m)  # Schmidt-cell average
        B = 5 * rho_avg
        G = gate(m, lam, mu, eta, B)
        val = G['value']
        ratio = val / rho_avg
        ok = G['feasible'] and val <= B
        allok &= ok
        print(f"m={m:>2}: feasible={G['feasible']}  value={float(val):.5f} <= 5*rho_avg({float(B):.4f})  "
              f"ratio={float(ratio):.5f}  worst={G['worst_class']}:{G['worst_slack']}  {'OK' if ok else '<<<FAIL'}")
    print(f"\n==== constant-eta certificate: dual-feasible AND ratio < 5 for m=4..32:  {allok} ====")


if __name__ == "__main__":
    main()
