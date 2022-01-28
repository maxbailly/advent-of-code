use std::ops::Add;
use std::iter::Sum;

use integer_partitions::Partitions;

/* ----------- */

type Ingredients = Vec<Ingredient>;

/* ----------- */

#[derive(Default, Debug, Copy, Clone)]
struct Ingredient {
    _name: &'static str,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn values_for_teaspoons(&self, teaspoons: i32) -> Self {
        Self {
            _name: self._name,
            capacity: self.capacity * teaspoons,
            durability: self.durability * teaspoons,
            flavor: self.flavor * teaspoons,
            texture: self.texture * teaspoons,
            calories: self.calories
        }
    }

    fn score(&self) -> i32 {
        if self.capacity <= 0 || self.durability <= 0 || self.flavor <= 0 || self.texture <= 0 {
            return 0
        }

        self.capacity * self.durability * self.flavor * self.texture
    }
}

impl Add<Self> for Ingredient {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            _name: "Mixed",
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories
        }
    }
}

impl Sum for Ingredient {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Ingredient::default(), |a, b| { a + b })
    }
}

impl From<&'static str> for Ingredient {
    fn from(line: &'static str) -> Self {
        let parts = line.split(": ").collect::<Vec<&str>>();

        let mut ret = Self {
            _name: parts[0],
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        };

        parts[1].split(", ").for_each(|part | {
            let parts = part.split_once(' ').expect("a valid ingredient stat");

            match parts {
                ("capacity", val) => ret.capacity = val.parse().expect("(capacity) a valid i8"),
                ("durability", val) => ret.durability = val.parse().expect("(durability) a valid i8"),
                ("flavor", val) => ret.flavor = val.parse().expect("(flavor) a valid i8"),
                ("texture", val) => ret.texture = val.parse().expect("(texture) a valid i8"),
                ("calories", val) => ret.calories = val.parse().expect("(calories) a valid i8"),
                _ => panic!("unknown stat")
            }
        });

        ret
    }
}

/* ----------- */

fn rotate_array<T: Copy>(arr: &mut Vec<T>) {
    let arr_len = arr.len();

    if arr_len == 0 {
        return
    }

    let first = arr[0];
    for i in 1..arr_len {
        arr[i - 1] = arr[i]
    }

    arr[arr_len - 1] = first
}

/* ----------- */

fn score(ingrs: &[Ingredient], proportions: &[usize]) -> i32 {
    let mixed = ingrs.iter().enumerate()
        .map(|(idx, ingr)| ingr.values_for_teaspoons(proportions[idx] as i32))
        .sum::<Ingredient>();

    mixed.score()
}

/* ----------- */

fn all_proportions_score(ingrs: &[Ingredient], proportions: &[usize]) -> i32 {
    let mut intermediate_score = 0;
    let mut prop = Vec::from(proportions);

    for _ in 0..prop.len() {
        let score = score(ingrs, &prop);

        if score > intermediate_score {
            intermediate_score = score
        }

        rotate_array(&mut prop)
    }

    intermediate_score
}

/* ----------- */

fn main() {
    let ingredients: Ingredients = utils::input_str!("part1.txt").lines()
        .map(Ingredient::from)
        .collect();
    let mut max_score = 0;

    let mut proportions = Partitions::new(100);
    while let Some(prop) = proportions.next() {
        if prop.len() == ingredients.len() {
            let new_score = all_proportions_score(&ingredients, prop);

            if new_score > max_score {
                max_score = new_score
            }
        }
    }

    println!("result = {}", max_score)
}
