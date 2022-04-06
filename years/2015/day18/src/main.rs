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

const TOP_LEFT_ID: usize = grid_to_index((0, 0), ARRAY_LINE_SIZE);
const TOP_RIGHT_ID: usize = grid_to_index((99, 0), ARRAY_LINE_SIZE);
const BOT_LEFT_ID: usize = grid_to_index((0, 99), ARRAY_LINE_SIZE);
const BOT_RIGHT_ID: usize = grid_to_index((99, 99), ARRAY_LINE_SIZE);

/* ---------- */

enum Part {
    One,
    Two,
}

impl Default for Part {
    fn default() -> Self {
        Self::One
    }
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

#[derive(Copy, Clone, Debug, PartialEq)]
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

fn next_step(prev: &[LightState], next: &mut Vec<LightState>, part: Part) {
    prev.iter()
        .enumerate()
        .filter(|(index, _)| *index != TOP_LEFT_ID || !matches!(part, Part::Two))
        .filter(|(index, _)| *index != TOP_RIGHT_ID || !matches!(part, Part::Two))
        .filter(|(index, _)| *index != BOT_LEFT_ID || !matches!(part, Part::Two))
        .filter(|(index, _)| *index != BOT_RIGHT_ID || !matches!(part, Part::Two))
        .for_each(|(idx, light)| {
            let count = count_neighboor_on_at(prev, idx, ARRAY_LINE_SIZE);
            next[idx] = light.next(count);
        })
}

/* ---------- */

fn part1() -> usize {
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
        next_step(&prev, &mut next, Part::One);
        next = std::mem::replace(&mut prev, next);
    }

    prev.iter()
        .filter(|light| matches!(light, LightState::On))
        .count()
}

/* ---------- */

fn part2() -> usize {
    let mut prev = utils::input_bytes!()
        .iter()
        .filter_map(|byte| match byte {
            b'#' => Some(LightState::On),
            b'.' => Some(LightState::Off),
            _ => None,
        })
        .collect::<Vec<LightState>>();

    prev[TOP_LEFT_ID] = LightState::On;
    prev[TOP_RIGHT_ID] = LightState::On;
    prev[BOT_LEFT_ID] = LightState::On;
    prev[BOT_RIGHT_ID] = LightState::On;

    let mut next = vec![LightState::Off; prev.len()];
    next[TOP_LEFT_ID] = LightState::On;
    next[TOP_RIGHT_ID] = LightState::On;
    next[BOT_LEFT_ID] = LightState::On;
    next[BOT_RIGHT_ID] = LightState::On;

    for _ in 0..STEPS {
        next_step(&prev, &mut next, Part::Two);
        next = std::mem::replace(&mut prev, next);
    }

    prev.iter()
        .filter(|light| matches!(light, LightState::On))
        .count()
}

/* ---------- */

fn main() {
    // part2();
    utils::answer!()
}
