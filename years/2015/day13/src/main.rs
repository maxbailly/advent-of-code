use std::cell::RefCell;
use std::collections::HashMap;

/* ---------- */

type Relations = HashMap<&'static str, i32>;
type Persons = HashMap<&'static str, Relations>;

/* ---------- */

#[inline(always)]
fn get_happiness(modifier: &str, val_str: &str) -> i32 {
    let val = val_str.parse().expect("failed to parse happiness value");

    match modifier {
        "gain" => val,
        "lose" => -val,
        _ => panic!("unknown modifier {}", modifier)
    }
}

/* ---------- */

fn calc_optimized_happiness(persons: &Persons) -> i32 {
    let placement = RefCell::new(Vec::new());
    let mut happiness = i32::MIN;

    fn rec_internal(persons: &Persons, placement: &RefCell<Vec<&str>>, happiness: &mut i32) {
        persons.iter()
            .filter(|(name, _)| {
                !placement.borrow().iter().any(|ref placed| placed == name)
            })
            .for_each(|(name, _)| {
                placement.borrow_mut().push(name);
                rec_internal(persons, placement, happiness);
                placement.borrow_mut().pop();
            });

        if placement.borrow().len() == persons.len() {
            let happ = calc_placement_happiness(persons, placement);

            if happ > *happiness {
                *happiness = happ
            }
        }
    }

    rec_internal(persons, &placement, &mut happiness);

    happiness
}

fn calc_placement_happiness(persons: &Persons, placement: &RefCell<Vec<&str>>) -> i32 {
    let placement = placement.borrow();

    placement.iter().enumerate()
        .map(|(idx, &person)| {
            let left_person = match idx {
                0 => placement[placement.len() - 1],
                _ => placement[idx - 1]
            };

            let right_person = match idx {
                _ if idx == placement.len() - 1 => placement[0],
                _ => placement[idx + 1]
            };

            get_person_happiness(persons, person, left_person, right_person)
        })
        .sum()
}

/* ---------- */

#[inline(always)]
fn get_person_happiness(persons: &Persons, person: &str, left_person: &str, right_person: &str) -> i32 {
    persons.get(person).expect("apparenlty, some peeps aren't registered for some reasons")
        .iter()
        .filter(|(&name, _)| name == left_person || name == right_person)
        .map(|(_, happiness)| happiness)
        .sum()
}

/* ---------- */

fn main() {
    let mut persons = Persons::default();

    utils::input_str!().lines()
        .for_each(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();

            let person1 = parts[0];
            let person2 = parts.last().expect("can't find 2nd person's name").trim_matches('.');
            let happiness = get_happiness(parts[2], parts[3]);

            let p1_rel = persons.entry(person1).or_insert_with(Relations::default);
            p1_rel.entry(person2).or_insert(happiness);

            persons.entry(person2).or_insert_with(Relations::default);
        });

    println!("result => {}", calc_optimized_happiness(&persons));
}
