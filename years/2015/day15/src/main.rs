use std::iter::Sum;
use std::ops::{Add, Mul};

use kombini::Kombini;
use partitions::Partitions;

/* ---------- */

const INPUT: &str = utils::input_str!();

/* ---------- */

#[derive(Default, Debug, Copy, Clone)]
struct Stats {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Stats {
    fn score(&self) -> u32 {
        if self.capacity <= 0 || self.durability <= 0 || self.flavor <= 0 || self.texture <= 0 {
            return 0;
        }

        (self.capacity * self.durability * self.flavor * self.texture) as u32
    }

    fn calories(&self) -> i32 {
        self.calories
    }
}

impl Add for Stats {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl Mul<usize> for Stats {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            capacity: self.capacity * rhs as i32,
            durability: self.durability * rhs as i32,
            flavor: self.flavor * rhs as i32,
            texture: self.texture * rhs as i32,
            calories: self.calories * rhs as i32,
        }
    }
}

impl Sum for Stats {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |a, b| a + b)
    }
}

impl From<&'static str> for Stats {
    fn from(line: &'static str) -> Self {
        let parts = line.split(": ").collect::<Vec<&str>>();

        let mut ret = Self {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        };

        parts[1].split(", ").for_each(|part| {
            let parts = part.split_once(' ').expect("a valid ingredient stat");

            match parts {
                ("capacity", val) => ret.capacity = val.parse().expect("(capacity) a valid i32"),
                ("durability", val) => {
                    ret.durability = val.parse().expect("(durability) a valid i32")
                }
                ("flavor", val) => ret.flavor = val.parse().expect("(flavor) a valid i32"),
                ("texture", val) => ret.texture = val.parse().expect("(texture) a valid i32"),
                ("calories", val) => ret.calories = val.parse().expect("(calories) a valid i32"),
                _ => panic!("unknown stat"),
            }
        });

        ret
    }
}

/* ---------- */

fn main() {
    let gen = Partitions::<4>::new(100);
    let ingrs: Vec<Stats> = INPUT.lines().map(Stats::from).collect();

    let mut max_score = 0;
    let mut max_score_with_cals = 0;

    for parts in gen {
        for teaspoons in Kombini::from(parts) {
            let recipe = ingrs
                .iter()
                .enumerate()
                .map(|(index, ingr)| *ingr * teaspoons[index])
                .sum::<Stats>();

            let score = recipe.score();

            if score > max_score {
                max_score = score
            }

            if recipe.calories() == 500 && score > max_score_with_cals {
                max_score_with_cals = score
            }
        }
    }

    println!("[PART 1] Answer = {max_score}");
    println!("[PART 2] Answer = {max_score_with_cals}");
}
