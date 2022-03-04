const ARRAY_LINE_SIZE: usize = 100;
const STEPS: usize = 100;

/* ---------- */

#[inline(always)]
const fn index_to_grid(idx: usize, size: usize) -> (usize, usize) {
    (idx % size, idx / size)
}

#[inline(always)]
const fn grid_to_index((x, y): (usize, usize), size: usize) -> usize {
    y * size + x
}

/* ---------- */

fn count_neighboor_on_at(array: &[LightState], idx: usize, size: usize) -> usize {
    let mut count = 0usize;
    let grid = index_to_grid(idx, size);
    let bounds = (
        if grid.0 != 0 { grid.0 - 1 } else { 0 },
        if grid.1 != 0 { grid.1 - 1 } else { 0 },
        if grid.0 != size - 1 {
            grid.0 + 1
        } else {
            size - 1
        },
        if grid.1 != size - 1 {
            grid.1 + 1
        } else {
            size - 1
        },
    );

    for y in (bounds.1)..=(bounds.3) {
        for x in (bounds.0)..=(bounds.2) {
            if x == grid.0 && y == grid.1 {
                continue;
            }

            let idx = grid_to_index((x, y), size);
            if matches!(array[idx], LightState::On) {
                count += 1;
            }
        }
    }

    count
}

/* ---------- */

#[derive(Copy, Clone, Debug)]
enum LightState {
    On,
    Off,
}

impl LightState {
    fn next(&self, neightboors: usize) -> Self {
        match self {
            LightState::Off if neightboors == 3 => LightState::On,
            LightState::On if neightboors == 2 || neightboors == 3 => LightState::On,
            _ => LightState::Off,
        }
    }
}

/* ---------- */

fn next_step(prev: &[LightState], next: &mut Vec<LightState>) {
    prev.iter().enumerate().for_each(|(idx, light)| {
        let count = count_neighboor_on_at(prev, idx, ARRAY_LINE_SIZE);
        next[idx] = light.next(count);
    })
}

/* ---------- */

fn main() {
    let mut prev = utils::input_bytes!()
        .iter()
        .filter_map(|byte| match byte {
            b'#' => Some(LightState::On),
            b'.' => Some(LightState::Off),
            _ => None,
        })
        .collect::<Vec<LightState>>();

    let mut next = vec![LightState::Off; prev.len()];

    for _ in 0..STEPS {
        next_step(&prev, &mut next);
        next = std::mem::replace(&mut prev, next);
    }

    let res = prev
        .iter()
        .filter(|light| matches!(light, LightState::On))
        .count();

    println!("result = {}", res);
}
