const COL: usize = 3075;
const ROW: usize = 2981;
const FIRST_CODE: usize = 20151125;

const MULT_BY: usize = 252533;
const MOD_BY: usize = 33554393;

/* ---------- */

const fn int_sum(mut n: usize) -> usize {
    let mut sum = 0;

    while n > 0 {
        sum += n;
        n -= 1;
    }

    sum
}

/* ---------- */

const fn grid_to_idx(col: usize, row: usize) -> usize {
    let line_num = col + row - 1;
    int_sum(line_num - 1) + col
}

/* ---------- */

const fn get_code_at_index(idx: usize) -> usize {
    let mut val = FIRST_CODE;

    if idx == 1 {
        return val;
    }

    let mut n = 2;
    while n <= idx {
        val = (val * MULT_BY) % MOD_BY;
        n += 1;
    }

    val
}

/* ---------- */

fn main() {
    let idx = grid_to_idx(COL, ROW);
    let val = get_code_at_index(idx);

    println!("result = {}", val);
}
