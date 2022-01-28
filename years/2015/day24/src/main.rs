type Result = std::result::Result<(), ()>;

/* ---------- */

#[derive(Debug)]
struct IdealConfiguration {
    qe: u64,
    count: usize
}

impl IdealConfiguration {
    #[inline(always)]
    fn new() -> Self {
        Self {
            qe: u64::MAX,
            count: usize::MAX
        }
    }

    fn apply(&mut self, group: &Group) {
        let grp_count = group.count();

        if self.count < grp_count {
            return
        }

        let grp_qe = group.qe();

        if self.count > grp_count {
            self.count = grp_count;
            self.qe = grp_qe;
        } else if self.count == grp_count && self.qe > grp_qe {
            self.qe = grp_qe;
        }
    }

    #[inline(always)]
    fn qe(&self) -> u64 {
        self.qe
    }
}

/* ---------- */

#[derive(Debug)]
struct Group {
    items: Vec<u64>,
    target_weight: u64,
    current_weight: u64
}

impl Group {
    #[inline(always)]
    fn new(cap: usize, target_weight: u64) -> Self {
        Self {
            items: Vec::with_capacity(cap),
            target_weight,
            current_weight: 0
        }
    }

    fn count(&self) -> usize {
        self.items.len()
    }

    fn qe(&self) -> u64 {
        self.items.iter().product()
    }

    fn push(&mut self, item: u64) -> Result {
        if self.current_weight + item > self.target_weight {
            return Err(())
        }

        self.current_weight += item;
        self.items.push(item);

        Ok(())
    }

    #[inline(always)]
    fn pop(&mut self) {
        let item = self.items.pop().expect("expected a non-empty group");
        self.current_weight -= item;
    }
}

/* ---------- */

fn get_all_configurations(pkgs: &[u64]) -> IdealConfiguration {
    fn rec_find_configs(pkgs: &[u64], idx: usize, g1: &mut Group, g2: &mut Group, g3: &mut Group, conf: &mut IdealConfiguration) {
        if idx == 0 {
            conf.apply(g1);
            return
        }

        if g1.push(pkgs[idx - 1]).is_ok() {
            rec_find_configs(pkgs, idx - 1, g1, g2, g3, conf);
            g1.pop();
        }

        if g2.push(pkgs[idx - 1]).is_ok() {
            rec_find_configs(pkgs, idx - 1, g1, g2, g3, conf);
            g2.pop();
        }

        if g3.push(pkgs[idx - 1]).is_ok() {
            rec_find_configs(pkgs, idx - 1, g1, g2, g3, conf);
            g3.pop();
        }
   }

    let total_weight = pkgs.iter().sum::<u64>();

    if total_weight % 3 != 0 {
        panic!("problem can't be solved")
    }

    let len = pkgs.len();
    let target_weight = total_weight / 3;
    let mut g1 = Group::new(len, target_weight);
    let mut g2 = Group::new(len, target_weight);
    let mut g3 = Group::new(len, target_weight);

    let mut conf = IdealConfiguration::new();

    rec_find_configs(pkgs, len, &mut g1, &mut g2, &mut g3, &mut conf);

    conf
}

/* ---------- */

fn main() {
    let pkgs = utils::input_str!("part1.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();

    let conf = get_all_configurations(&pkgs);

    println!("result = {}", conf.qe())
}
