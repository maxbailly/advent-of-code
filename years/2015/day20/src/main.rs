const INPUT_RAW: u32 = 36000000;
const MAX_HOUSE: u32 = 50;

/* ---------- */

const fn house_max_elf(n_house: u32) -> u32 {
    (n_house - 1)  / MAX_HOUSE + 1
}

/* ---------- */

fn find_elves(n: u32, elf_min: Option<u32>) -> Vec<u32> {
    #[inline(always)]
    fn max_range(n: u32) -> u32 {
        ((n as f32).sqrt() + 1.0) as u32
    }

    let mut ret = vec![];
    let mut large_div = vec![];
    let elf_min = elf_min.unwrap_or(1);

    for i in 1..max_range(n) {
        if n % i == 0 {
            if i >= elf_min {
                ret.push(i);
            }

            if i > 1 && i * i != n {
                large_div.push(n / i)
            }
        }
    }

    ret.extend(large_div.iter().rev().skip_while(|&&inner| inner < elf_min));

    if n != 1 {
        ret.push(n);
    }

    ret
}

/* ---------- */

#[inline(always)]
fn nb_presents(house_id: u32, lazy_elves: bool, mult: u32) -> u32 {
    let min_elf = lazy_elves.then(|| house_max_elf(house_id));
    let houses = find_elves(house_id, min_elf);

    houses.iter().sum::<u32>() * mult
}

/* ---------- */

fn part1() -> u32 {
    let mut house_id = 1u32;

    while nb_presents(house_id, false, 10) < INPUT_RAW {
        house_id += 1;
    }

    house_id
}

/* ---------- */

fn part2() -> u32 {
    let mut house_id = 1u32;

    while nb_presents(house_id, true, 11) < INPUT_RAW {
        house_id += 1;
    }

    house_id
}

/* ---------- */

fn main() {
    utils::answer!()
}
