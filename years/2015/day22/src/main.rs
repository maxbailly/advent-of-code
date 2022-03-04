const SPELLS: &[Spell] = &[
    Spell {
        name: "Magic Missile",
        cost: 53,
        damage: 4,
        self_heal: 0,
        effect: None,
        target: Target::Other,
    },
    Spell {
        name: "Drain",
        cost: 73,
        damage: 2,
        self_heal: 2,
        effect: None,
        target: Target::Other,
    },
    Spell {
        name: "Shield",
        cost: 113,
        damage: 0,
        self_heal: 0,
        effect: Some(SHIELD),
        target: Target::Itself,
    },
    Spell {
        name: "Poison",
        cost: 173,
        damage: 0,
        self_heal: 0,
        effect: Some(DMG_OVER_TIME),
        target: Target::Other,
    },
    Spell {
        name: "Mana Regen",
        cost: 229,
        damage: 0,
        self_heal: 0,
        effect: Some(MANA_REGEN),
        target: Target::Itself,
    },
];

const SHIELD: Effect = Effect::Shield(7, 6);
const MANA_REGEN: Effect = Effect::ManaRegen(101, 5);
const DMG_OVER_TIME: Effect = Effect::DmgOverTime(3, 6);
const MAX_EFFECTS: usize = 3;

/* ---------- */

#[derive(Debug)]
enum Err {
    OutOfMana,
    EffectAlreadyApplied,
}

impl std::fmt::Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            Self::OutOfMana => "out of mana",
            Self::EffectAlreadyApplied => "effect already applied to target",
        };

        write!(f, "{}", msg)
    }
}

/* ---------- */

#[derive(Debug, Clone, Copy)]
enum Target {
    Itself,
    Other,
}

/* ---------- */

#[derive(Debug, Copy, Clone)]
enum Effect {
    Shield(i16, u8),
    ManaRegen(i16, u8),
    DmgOverTime(i16, u8),
}

impl Effect {
    fn ticks(&self) -> u8 {
        match self {
            Self::Shield(_, t) | Self::ManaRegen(_, t) | Self::DmgOverTime(_, t) => *t,
        }
    }

    fn decrement_ticks(&mut self) {
        match self {
            Self::Shield(_, t) | Self::ManaRegen(_, t) | Self::DmgOverTime(_, t) => *t -= 1,
        }
    }
}

impl PartialEq for Effect {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Shield(_, _), Self::Shield(_, _))
                | (Self::ManaRegen(_, _), Self::ManaRegen(_, _))
                | (Self::DmgOverTime(_, _), Self::DmgOverTime(_, _))
        )
    }
}

/* ---------- */

#[derive(Debug, Clone, Copy)]
struct Spell {
    name: &'static str,
    pub cost: i16,
    pub damage: i16,
    pub self_heal: i16,
    pub effect: Option<Effect>,
    pub target: Target,
}

impl std::fmt::Display for Spell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/* ---------- */

#[derive(Debug, Copy, Clone)]
struct Entity {
    name: &'static str,
    hp: i16,
    mana: i16,
    damage: i16,
    armor: i16,
    effects: [Option<Effect>; MAX_EFFECTS],
}

impl Entity {
    fn new(name: &'static str, hp: i16, damage: i16, mana: i16, armor: i16) -> Self {
        Self {
            name,
            hp,
            mana,
            damage,
            armor,
            effects: [None; MAX_EFFECTS],
        }
    }

    #[inline(always)]
    fn attacks(&self, target: &mut Self) {
        target.hp -= if target.armor >= self.damage {
            1
        } else {
            self.damage - target.armor
        };
    }

    #[inline(always)]
    fn can_cast(&self, spell: &Spell, target: &Self) -> Result<(), Err> {
        if self.mana < spell.cost {
            return Err(Err::OutOfMana);
        }

        if let Some(effect) = &spell.effect {
            return match spell.target {
                Target::Itself => {
                    if self.is_effect_applied(effect) {
                        Err(Err::EffectAlreadyApplied)
                    } else {
                        Ok(())
                    }
                }
                Target::Other => {
                    if target.is_effect_applied(effect) {
                        Err(Err::EffectAlreadyApplied)
                    } else {
                        Ok(())
                    }
                }
            };
        }

        Ok(())
    }

    fn casts(&mut self, spell: &Spell, target: &mut Self) {
        self.mana -= spell.cost;

        match spell.target {
            Target::Itself => {
                if let Some(effect) = spell.effect {
                    self.apply_effect(effect);
                }
            }
            Target::Other => {
                if let Some(effect) = spell.effect {
                    target.apply_effect(effect);
                }
                target.hp -= spell.damage;
                self.hp += spell.self_heal;
            }
        }
    }

    #[inline(always)]
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    #[inline(always)]
    fn is_effect_applied(&self, effect: &Effect) -> bool {
        self.effects
            .iter()
            .filter(|eff| eff.is_some())
            .any(|eff| eff.expect("a valid effect") == *effect)
    }

    fn apply_effect(&mut self, effect: Effect) {
        let eff = self
            .effects
            .iter_mut()
            .find(|eff| eff.is_none())
            .expect("expected at least one free effect spot");
        let effect = eff.insert(effect);

        if let Effect::Shield(armor, _) = effect {
            self.armor += *armor;
        }
    }

    fn update_effects(&mut self) {
        self.effects.iter_mut().for_each(|eff| {
            let mut ended = false;

            if let Some(eff) = eff {
                match eff {
                    Effect::ManaRegen(mana_regen, _) => self.mana += *mana_regen,
                    Effect::DmgOverTime(dmg, _) => self.hp -= *dmg,
                    _ => (),
                }

                eff.decrement_ticks();

                if eff.ticks() == 0 {
                    ended = true
                }
            }

            if ended {
                if let Some(Effect::Shield(armor, _)) = eff.take() {
                    self.armor -= armor;
                }
            }
        });
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "- {} has {} hp, {} mana, {} armor",
            self.name, self.hp, self.mana, self.armor
        )
    }
}

/* ---------- */

#[derive(Debug)]
enum Turn {
    Player,
    Boss,
}

impl Turn {
    fn next(&self) -> Self {
        match self {
            Self::Player => Self::Boss,
            Self::Boss => Self::Player,
        }
    }
}

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Boss => write!(f, "Boss"),
            Self::Player => write!(f, "Player"),
        }
    }
}

/* ---------- */

fn sim(
    turn: Turn,
    mut player: Entity,
    mut boss: Entity,
    mana_spent: i16,
    min_mana_spent: &mut i16,
) {
    player.update_effects();
    boss.update_effects();

    if !boss.is_alive() {
        if mana_spent < *min_mana_spent {
            *min_mana_spent = mana_spent;
        }
        return;
    }

    if !player.is_alive() {
        return;
    }

    match turn {
        Turn::Boss => {
            boss.attacks(&mut player);
            sim(turn.next(), player, boss, mana_spent, min_mana_spent);
        }
        Turn::Player => SPELLS.iter().for_each(|spell| {
            if mana_spent + spell.cost < *min_mana_spent && player.can_cast(spell, &boss).is_ok() {
                let mut tmp_player = player;
                let mut tmp_boss = boss;

                tmp_player.casts(spell, &mut tmp_boss);
                sim(
                    turn.next(),
                    tmp_player,
                    tmp_boss,
                    mana_spent + spell.cost,
                    min_mana_spent,
                );
            }
        }),
    }
}

/* ---------- */

fn main() {
    let boss = Entity::new("Boss", 71, 10, 0, 0);
    let player = Entity::new("Player", 50, 0, 500, 0);
    let turn = Turn::Player;
    let mut min_mana_spent = i16::MAX;

    sim(turn, player, boss, 0, &mut min_mana_spent);

    println!("result = {}", min_mana_spent);
}
