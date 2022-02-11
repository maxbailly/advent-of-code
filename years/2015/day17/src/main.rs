use std::collections::HashSet;
use std::hash::Hash;

/* ---------- */

const EGGNOG_AMOUNT: u8 = 150;

/* ---------- */

#[derive(Hash, PartialEq, Eq, Clone)]
struct Container {
    id: usize,
    quantity: u8
}

impl Container {
    fn new(id: usize, quantity: u8) -> Self {
        Self {
            id,
            quantity
        }
    }

    fn quantity(&self) -> u8 {
        self.quantity
    }
}

impl std::fmt::Debug for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.quantity)
    }
}

/* ---------- */

fn equal<T>(a: &[T], b: &[T]) -> bool
where
    T: Eq + Hash
{
    let a: HashSet<_> = a.iter().collect();
    let b: HashSet<_> = b.iter().collect();

    a == b
}

/* ---------- */

fn count_combinaisons(containers: &mut Vec<Container>, amount: u8) -> usize {
    fn try_combinaisons(
        containers: &mut Vec<Container>,
        amount: u8,
        combinaisons: &mut Vec<Vec<Container>>,
        selection: &mut Vec<Container>,
    ) {
        let len = containers.len();

        if amount == 0 && !combinaisons.iter().any(|comb| equal(comb, selection)) {
            combinaisons.push(selection.clone())
        }

        for idx in 0..len {
            let quantity = containers[idx].quantity();

            if amount >= quantity {
                let container = containers.remove(idx);
                selection.push(container);

                try_combinaisons(containers, amount - quantity, combinaisons, selection);

                let container = selection.pop().expect("failed to pop last container selected");
                containers.insert(idx, container);
            }
        }
    }

    let mut selection: Vec<Container> = Vec::with_capacity(containers.len());
    let mut combinaisons: Vec<Vec<Container>> = Vec::new();

    try_combinaisons(containers, amount, &mut combinaisons, &mut selection);

    combinaisons.len()
}

/* ---------- */

fn main() {
    let mut containers: Vec<Container> = utils::input_str!().lines()
        .enumerate()
        .map(|(id, line)| {
            let quantity = line.parse().expect("failed to parse container size");
            Container::new(id, quantity)
        })
        .collect();

    let res = count_combinaisons(&mut containers, EGGNOG_AMOUNT);
    println!("result = {}", res)
}
