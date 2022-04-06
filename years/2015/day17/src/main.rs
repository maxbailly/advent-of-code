use itertools::Itertools;

/* ---------- */

const EGGNOG_QUANTITY: usize = 150;

/* ---------- */

fn part1(containers: &[usize]) -> usize {
    containers
        .iter()
        .powerset()
        .filter(|set| set.iter().copied().sum::<usize>() == EGGNOG_QUANTITY)
        .count()
}

/* ---------- */

fn part2(containers: &[usize]) -> usize {
    let mut min = usize::MAX;
    let mut count = 1usize;


    containers
        .iter()
        .powerset()
        .filter(|set| set.iter().copied().sum::<usize>() == EGGNOG_QUANTITY)
        .for_each(|set| {
            let amount = set.len();

            match amount {
                _ if amount < min => {
                    count = 1;
                    min = amount;
                }
                _ if amount == min => count += 1,
                _ => ()
            }
        });

    count
}

/* ---------- */

fn main() {
    let containers: Vec<usize> = utils::input_str!()
        .lines()
        .map(|line| line.parse().expect("failed to parse container size"))
        .collect::<_>();

    utils::answer!(&containers);
}
