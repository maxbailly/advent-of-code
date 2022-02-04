const INPUT: &[u8] = utils::input_bytes!("part1.txt");

/* ---------- */

enum Direction {
    North,
    East,
    South,
    West
}

impl From<u8> for Direction {
    fn from(dir: u8) -> Self {
        match dir {
            b'^' => Self::North,
            b'>' => Self::East,
            b'v' => Self::South,
            b'<' => Self::West,
            _ => panic!("expected a valid direction")
        }
    }
}

impl From<&u8> for Direction {
    fn from(dir: &u8) -> Self {
        match dir {
            b'^' => Self::North,
            b'>' => Self::East,
            b'v' => Self::South,
            b'<' => Self::West,
            _ => panic!("expected a valid direction {:?}", dir)
        }
    }
}

/* ---------- */

#[derive(Debug, Default, PartialEq, Clone, Copy)]
struct Position(i8, i8);

impl Position {
    #[inline(always)]
    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.up(),
            Direction::East => self.right(),
            Direction::South => self.down(),
            Direction::West => self.left(),
        }
    }

    #[inline(always)]
    fn up(&mut self) {
        self.1 += 1
    }

    #[inline(always)]
    fn right(&mut self) {
        self.0 += 1
    }

    #[inline(always)]
    fn down(&mut self) {
        self.1 -= 1
    }

    #[inline(always)]
    fn left(&mut self) {
        self.0 -= 1
    }
}

/* ---------- */

enum Turn {
    Santa,
    RoboSanta
}

impl Turn {
    fn next(&mut self) {
        *self = match self {
            Self::Santa => Self::RoboSanta,
            Self::RoboSanta => Self::Santa
        }
    }
}

impl Default for Turn {
    fn default() -> Self {
        Self::Santa
    }
}

/* ---------- */

fn part1() -> usize {
    let mut pos = Position::default();
    let mut visited =  Vec::<Position>::with_capacity(INPUT.len() + 1);

    INPUT.iter().filter(|c| matches!(c, b'^' | b'>' | b'v' | b'<'))
        .map(Direction::from)
        .for_each(|dir| {
            pos.move_direction(dir);
            if !visited.iter().any(|visited_pos| *visited_pos == pos) {
                visited.push(pos);
            }
        });

    visited.len()
}

/* ---------- */

fn part2() -> usize {
    let mut turn = Turn::default();
    let mut santa_pos = Position::default();
    let mut robo_santa_pos = Position::default();
    let mut visited =  Vec::<Position>::with_capacity(INPUT.len() + 1);

    #[inline(always)]
    fn internal_move(pos: &mut Position, visited: &mut Vec<Position>, dir: Direction) {
        pos.move_direction(dir);

        if !visited.iter().any(|visited_pos| visited_pos == pos) {
            visited.push(*pos);
        }
    }

    INPUT.iter().filter(|c| matches!(c, b'^' | b'>' | b'v' | b'<'))
        .map(Direction::from)
        .for_each(|dir| {
            match turn {
                Turn::Santa => internal_move(&mut santa_pos, &mut visited, dir),
                Turn::RoboSanta => internal_move(&mut robo_santa_pos, &mut visited, dir)
            }

            turn.next()
        });

    visited.len()
}

/* ---------- */

fn main() {
    utils::answer!()
}
