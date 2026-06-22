from experiments.lib.marginal_invariants import (
    apply_row_combination,
    dual_rows_for_columns,
    dot_f2,
    min_coset_weight,
    parity_check_rows,
    rows_to_columns,
    syndrome,
)


def test_dot_f2_parity():
    assert dot_f2(0b1011, 0b1100) == 1
    assert dot_f2(0b1011, 0b0100) == 0


def test_parity_check_rows_annihilate_columns():
    # C has two columns in F_2^3: 011 and 101, rank 2.
    cols = [0b011, 0b101]
    checks = parity_check_rows(cols, m=3)
    assert len(checks) == 1
    assert all(dot_f2(h, c) == 0 for h in checks for c in cols)
    assert checks[0] != 0


def test_syndrome_is_zero_on_column_space():
    cols = [0b011, 0b101]
    checks = parity_check_rows(cols, m=3)
    y = cols[0] ^ cols[1]
    assert syndrome(checks, y) == 0


def test_rows_to_columns_transposes_bitmask_matrix():
    rows = [0b01, 0b11, 0b10]
    assert rows_to_columns(rows, n_cols=2) == [0b011, 0b110]


def test_dual_rows_for_columns_lift_uniform_c_rows():
    cols = [0b011, 0b101]
    duals = dual_rows_for_columns(cols, m=3)

    assert all(dot_f2(duals[j], cols[k]) == int(j == k) for j in range(2) for k in range(2))
    for coeffs in range(1 << len(cols)):
        lifted = apply_row_combination(duals, coeffs)
        assert [dot_f2(lifted, col) for col in cols] == [
            (coeffs >> j) & 1 for j in range(len(cols))
        ]


def test_min_coset_weight_against_column_space():
    cols = [0b011, 0b101]
    assert min_coset_weight(cols, y=0b110, m=3) == 0
    assert min_coset_weight(cols, y=0b001, m=3) == 1
