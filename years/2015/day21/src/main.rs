use std::ops::{Add, AddAssign};

use serde::Deserialize;

/* ---------- */

const BOSS: Entity = Entity::new(100, 8, 2);

/* ---------- */

#[derive(Debug, Copy, Clone)]
struct Stats {
    pub hp: u8,
    pub damage: u8,
    pub armor: u8,
}

impl Stats {
    const fn new(hp: u8, damage: u8, armor: u8) -> Self {
        Self { hp, damage, armor }
    }

    const fn resistance(&self, dmg: u8) -> u8 {
        let max_hits = self.hp / (dmg - self.armor);

        match self.hp % dmg {
            0 => max_hits,
            _ => max_hits + 1,
        }
    }

    const fn total_dmg_relative_stats(&self) -> u8 {
        self.damage + self.armor
    }

    const fn dmg_required(&self, turns: u8) -> u8 {
        let dmg = self.hp / turns;

        match self.hp % turns {
            0 => dmg,
            _ => dmg + 1,
        }
    }
}

impl Add<Self> for Stats {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            hp: self.hp + other.hp,
            damage: self.damage + other.damage,
            armor: self.armor + other.armor,
        }
    }
}

impl Add<&Self> for Stats {
    type Output = Self;
    fn add(self, other: &Self) -> Self {
        Self {
            hp: self.hp + other.hp,
            damage: self.damage + other.damage,
            armor: self.armor + other.armor,
        }
    }
}

impl AddAssign<Self> for Stats {
    fn add_assign(&mut self, other: Self) {
        self.hp += other.hp;
        self.damage += other.damage;
        self.armor += other.armor;
    }
}

/* ---------- */

#[derive(Deserialize)]
struct Item {
    pub name: String,
    pub cost: u8,
    pub damage: u8,
    pub armor: u8,
}

impl Item {
    const fn stats(&self) -> Stats {
        Stats {
            hp: 0,
            damage: self.damage,
            armor: self.armor,
        }
    }
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Item (name: {}, cost: {}, dmg: {}, arm: {})",
            self.name, self.cost, self.damage, self.armor
        )
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/* ---------- */

struct InventoryStats {
    cost: u16,
    stats: Stats,
}

impl InventoryStats {
    fn new(
        weapon: &Item,
        armor: Option<&Item>,
        rring: Option<&Item>,
        lring: Option<&Item>,
    ) -> Self {
        let mut stats = weapon.stats();
        let mut cost = weapon.cost as u16;

        if let Some(armor) = armor {
            stats += armor.stats();
            cost += armor.cost as u16
        }

        if let Some(ring) = lring {
            stats += ring.stats();
            cost += ring.cost as u16
        }

        if let Some(ring) = rring {
            stats += ring.stats();
            cost += ring.cost as u16
        }

        Self { cost, stats }
    }

    fn stats(&self) -> &Stats {
        &self.stats
    }

    fn cost(&self) -> u16 {
        self.cost as u16
    }
}

impl std::fmt::Debug for InventoryStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "cost: {}, stats: {}",
            self.cost,
            self.stats.total_dmg_relative_stats()
        )
    }
}

/* ---------- */

struct Entity {
    base_stats: Stats,
    inventory: Option<InventoryStats>,
}

impl Entity {
    const fn new(hp: u8, damage: u8, armor: u8) -> Self {
        Self {
            base_stats: Stats::new(hp, damage, armor),
            inventory: None,
        }
    }

    fn resistance(&self, dmg: u8) -> u8 {
        match &self.inventory {
            Some(inv) => (self.base_stats + inv.stats()).resistance(dmg),
            None => self.base_stats.resistance(dmg),
        }
    }

    fn stats_required_to_kill_in_turns(&self, turns: u8) -> u8 {
        match &self.inventory {
            Some(inv) => (self.base_stats + inv.stats()).dmg_required(turns),
            None => self.base_stats.dmg_required(turns),
        }
    }

    fn damage(&self) -> u8 {
        let mut dmg = self.base_stats.damage;

        if let Some(inv) = &self.inventory {
            dmg += inv.stats().damage
        }

        dmg
    }
}

/* ---------- */

#[derive(Deserialize, Debug)]
struct Shop {
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<Item>,
}

impl Shop {
    fn generate_all_inv_combinaisons_with_higher_stats(&self, stats: u8) -> Vec<InventoryStats> {
        let mut combinaisons = vec![];

        let rings_loop = |weap: &Item, armor: Option<&Item>, combs: &mut Vec<InventoryStats>| {
            for r in 0..self.rings.len() {
                for l in 0..self.rings.len() {
                    if l == r {
                        continue;
                    }

                    let inv_stats = InventoryStats::new(
                        weap,
                        armor,
                        Some(&self.rings[r]),
                        Some(&self.rings[l]),
                    );
                    if stats <= inv_stats.stats().total_dmg_relative_stats() {
                        combs.push(inv_stats)
                    }
                }
            }

            for ring in &self.rings {
                let inv_stats = InventoryStats::new(weap, armor, Some(ring), None);
                if stats <= inv_stats.stats().total_dmg_relative_stats() {
                    combs.push(inv_stats)
                }
            }
        };

        for weap in &self.weapons {
            for armor in &self.armor {
                rings_loop(weap, Some(armor), &mut combinaisons);
            }
            rings_loop(weap, None, &mut combinaisons);

            let inv_stats = InventoryStats::new(weap, None, None, None);
            if stats <= inv_stats.stats().total_dmg_relative_stats() {
                combinaisons.push(inv_stats)
            }
        }

        combinaisons
    }

    fn find_cheapest_stuff_cost(&self, stats: u8) -> u16 {
        let combs = self.generate_all_inv_combinaisons_with_higher_stats(stats);

        combs
            .iter()
            .map(|inv_stats| inv_stats.cost())
            .min()
            .expect("expected a valid stuff cost")
    }
}

/* ---------- */

fn main() {
    let shop: Shop = serde_json::from_str(utils::input_str!("input.json")).unwrap();
    let player = Entity::new(100, 0, 0);
    let turns_to_survive = player.resistance(BOSS.damage());
    let stats_req = BOSS.stats_required_to_kill_in_turns(turns_to_survive);

    println!("res = {}", shop.find_cheapest_stuff_cost(stats_req));
}
