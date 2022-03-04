const INPUT: u32 = 36000000 / 10;

/* ---------- */

fn find_elves(n: u32) -> Vec<u32> {
    #[inline(always)]
    fn max_range(n: u32) -> u32 {
        ((n as f32).sqrt() + 1.0) as u32
    }

    let mut ret = vec![1];
    let mut large_div = vec![];

    for i in 2..max_range(n) {
        if n % i == 0 {
            ret.push(i);
            if i * i != n {
                large_div.push(n / i)
            }
        }
    }

    ret.extend(large_div.iter().rev());
    ret.push(n);

    ret
}

/* ---------- */

#[inline(always)]
fn nb_presents(house_id: u32) -> u32 {
    find_elves(house_id).iter().sum()
}

/* ---------- */

fn main() {
    let mut house_id = 1u32;

    while nb_presents(house_id) < INPUT {
        house_id += 1;
    }

    println!("result = {}", house_id)
}
