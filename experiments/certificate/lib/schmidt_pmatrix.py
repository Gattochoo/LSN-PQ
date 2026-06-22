from fractions import Fraction as Fr


def q_binom2(n, k, q=2):
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    num = den = 1
    q2 = q * q
    for i in range(k):
        num *= q2 ** (n - i) - 1
        den *= q2 ** (i + 1) - 1
    return num // den


def F(m, r, s, q=2):
    nn = m // 2
    c = q ** (m - 1) if m % 2 == 0 else q**m
    res = 0
    for j in range(r + 1):
        res += (
            (-1) ** (r - j)
            * q ** ((r - j) * (r - j - 1))
            * q_binom2(nn - j, nn - r, q)
            * q_binom2(nn - s, j, q)
            * c**j
        )
    return res


def get_v(it, m, q=2):
    if it == (0, 1):
        return 1
    s = it[0] // 2
    if it[0] % 2 == 1:
        num = den = 1
        for i in range(2 * s + 1):
            num *= q**m - q**i
        for i in range(s):
            den *= q ** (2 * s) - q ** (2 * i)
        return num // (den * q**s)
    tau = it[1]
    num = den = 1
    for i in range(2 * s):
        num *= q**m - q**i
    for i in range(s):
        den *= q ** (2 * s) - q ** (2 * i)
    return int(Fr(q**s + tau, 2) * Fr(num, den))


def get_P(it, kt, m, q=2):
    if it == (0, 1):
        return Fr(1)
    r = kt[0] // 2
    s = it[0] // 2
    if kt == (0, 1):
        return Fr(get_v(it, m, q))
    if it[0] % 2 == 1:
        if kt[0] % 2 == 1:
            return Fr(-q ** (2 * s) * F(m - 1, s, r, q))
        eps = kt[1]
        if eps == 1:
            return Fr((q**m - q ** (2 * s)) * F(m, s, r, q))
        return Fr(-q ** (2 * s) * F(m - 1, s, r - 1, q))
    tau = it[1]
    if kt[0] % 2 == 1:
        return Fr(q ** (2 * s) * F(m - 1, s, r, q) + tau * q**s * F(m, s, r, q), 2)
    eps = kt[1]
    if eps == 1:
        return Fr(q**s * (q**s + tau) * F(m, s, r, q), 2)
    return Fr(q ** (2 * s) * F(m - 1, s, r - 1, q) + tau * q**s * F(m, s, r, q), 2)


def classes_of(m):
    cl = [(0, 1)]
    for s in range(1, m // 2 + 1):
        cl += [(2 * s, 1), (2 * s, -1)]
    for s in range(0, (m - 1) // 2 + 1):
        cl += [(2 * s + 1, 0)]
    return cl


def closed_form(m):
    if m % 2 == 0 and m >= 4:
        return Fr(13, 2) - Fr(5, 2 ** (m - 1))
    if m % 2 == 1 and m >= 5:
        return Fr(6) - Fr(11, 2**m)
    return None


def inv_matrix(B):
    n = len(B)
    aug = [[Fr(B[i][j]) for j in range(n)] + [Fr(1 if i == j else 0) for j in range(n)] for i in range(n)]
    for col in range(n):
        piv = None
        for r in range(col, n):
            if aug[r][col] != 0:
                piv = r
                break
        if piv is None:
            raise ValueError("singular basis")
        if piv != col:
            aug[col], aug[piv] = aug[piv], aug[col]
        pv = aug[col][col]
        aug[col] = [x / pv for x in aug[col]]
        for r in range(n):
            if r == col:
                continue
            fac = aug[r][col]
            if fac:
                aug[r] = [aug[r][j] - fac * aug[col][j] for j in range(2 * n)]
    return [row[n:] for row in aug]


def mat_vec(M, v):
    return [sum(M[i][j] * v[j] for j in range(len(v))) for i in range(len(M))]


def row_vec_mat(v, M):
    return [sum(v[i] * M[i][j] for i in range(len(v))) for j in range(len(M[0]))]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def simplex(A, b, c, basis, max_iter=100000):
    m = len(A)
    n = len(A[0])
    basis = list(basis)
    iterations = 0
    while True:
        iterations += 1
        if iterations > max_iter:
            raise RuntimeError("simplex iteration limit")
        B = [[A[i][basis[j]] for j in range(m)] for i in range(m)]
        Binv = inv_matrix(B)
        xB = mat_vec(Binv, b)
        cB = [c[j] for j in basis]
        y = row_vec_mat(cB, Binv)
        nonbasis = [j for j in range(n) if j not in basis]
        enter = None
        for j in nonbasis:
            col = [A[i][j] for i in range(m)]
            red = c[j] - dot(y, col)
            if red > 0:
                enter = j
                break
        if enter is None:
            x = [Fr(0) for _ in range(n)]
            for i, bi in enumerate(basis):
                x[bi] = xB[i]
            return {
                "x": x,
                "basis": basis,
                "objective": dot(c, x),
                "dual_y": y,
                "iterations": iterations,
            }
        d = mat_vec(Binv, [A[i][enter] for i in range(m)])
        ratios = []
        for i, di in enumerate(d):
            if di > 0:
                ratios.append((xB[i] / di, basis[i], i))
        if not ratios:
            raise RuntimeError("unbounded")
        _, _, leave_row = min(ratios)
        basis[leave_row] = enter


def phase_one(A, b):
    A = [[Fr(x) for x in row] for row in A]
    b = [Fr(x) for x in b]
    for i in range(len(b)):
        if b[i] < 0:
            A[i] = [-x for x in A[i]]
            b[i] = -b[i]
    m = len(A)
    n_orig = len(A[0])
    A1 = [row + [Fr(1 if i == j else 0) for j in range(m)] for i, row in enumerate(A)]
    c1 = [Fr(0) for _ in range(n_orig)] + [Fr(-1) for _ in range(m)]
    basis = list(range(n_orig, n_orig + m))
    res = simplex(A1, b, c1, basis)
    if res["objective"] != 0:
        raise RuntimeError(f"infeasible, phase-I objective {res['objective']}")
    basis = res["basis"]

    # Remove artificial variables from the basis. If an artificial row cannot
    # pivot to a real column, the row is redundant and is deleted.
    while True:
        changed = False
        for row_idx, bi in enumerate(list(basis)):
            if bi < n_orig:
                continue
            mcur = len(A)
            B = [[A[i][basis[j]] for j in range(mcur)] for i in range(mcur)]
            Binv = inv_matrix(B)
            in_basis = set(basis)
            pivot = None
            for j in range(n_orig):
                if j in in_basis:
                    continue
                d = mat_vec(Binv, [A[i][j] for i in range(mcur)])
                if d[row_idx] != 0:
                    pivot = j
                    break
            if pivot is not None:
                basis[row_idx] = pivot
            else:
                del A[row_idx]
                del b[row_idx]
                del basis[row_idx]
            changed = True
            break
        if not changed:
            return A, b, basis


def solve_lp(m):
    cl = classes_of(m)
    L = len(cl)
    N = m * (m + 1) // 2
    thr = 2 ** (N - 2 * m + 2)
    alpha = Fr(1, thr)
    P = [[get_P(it, kt, m) for kt in cl] for it in cl]

    nvars = 2 * L
    A = []
    b = []
    # a_(0,1) = alpha
    row = [Fr(0) for _ in range(nvars)]
    row[0] = Fr(1)
    A.append(row)
    b.append(alpha)
    # sum_k a_k = 1
    row = [Fr(0) for _ in range(nvars)]
    for k in range(L):
        row[k] = Fr(1)
    A.append(row)
    b.append(Fr(1))
    # P_i a - y_i = 0, with y_i >= 0.
    for i in range(L):
        row = [Fr(0) for _ in range(nvars)]
        for k in range(L):
            row[k] = P[i][k]
        row[L + i] = Fr(-1)
        A.append(row)
        b.append(Fr(0))

    c = [Fr(2 ** (m - kt[0])) for kt in cl] + [Fr(0) for _ in range(L)]
    A2, b2, basis = phase_one(A, b)
    res = simplex(A2, b2, c, basis)
    a = res["x"][:L]
    y_slack = res["x"][L:]
    rho = sum(Fr(2 ** (m - kt[0])) * a[k] for k, kt in enumerate(cl))
    return {
        "m": m,
        "classes": cl,
        "P": P,
        "thr": thr,
        "alpha": alpha,
        "a": a,
        "rho": rho,
        "slack": y_slack,
        "basis": res["basis"],
        "dual_y": res["dual_y"],
        "iterations": res["iterations"],
        "closed": closed_form(m),
    }


def fmt(fr):
    return str(fr.numerator) if fr.denominator == 1 else f"{fr.numerator}/{fr.denominator}"


if __name__ == "__main__":
    for m in range(2, 15):
        sol = solve_lp(m)
        cf = sol["closed"]
        print("m", m, "rho", fmt(sol["rho"]), "cf", fmt(cf) if cf is not None else "-", "dev", fmt(sol["rho"] - cf) if cf is not None else "-")
        print(" support")
        for kt, val in zip(sol["classes"], sol["a"]):
            if val:
                print("  ", kt, fmt(val))
        active = [it for it, val in zip(sol["classes"], sol["slack"]) if val == 0]
        print(" active_P", active)
