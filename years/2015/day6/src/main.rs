use std::ops::Deref;

/* ---------- */

const LENGHT: usize = 1000;
const ARRAY_SIZE: usize = LENGHT * LENGHT;

const fn index_from_coord(x: u16, y: u16) -> usize {
    y as usize * LENGHT + x as usize
}

/* ---------- */

#[derive(Default, Debug)]
struct Coord(u16, u16);

impl Coord {
    fn x(&self) -> u16 {
        self.0
    }

    fn y(&self) -> u16 {
        self.1
    }
}

impl From<&str> for Coord {
    fn from(data: &str) -> Self {
        let split: Vec<&str> = data.split(',').collect();

        Self(
            split[0].parse::<u16>().unwrap(),
            split[1].parse::<u16>().unwrap(),
        )
    }
}

/* ---------- */

struct Instruction(fn(&mut LightState));

impl Instruction {
    fn new_from_str(instr_str: &str, action: Action) -> Self {
        Self(match instr_str {
            "on" => match action {
                Action::OnOff => LightState::turn_on,
                Action::Brightness => LightState::inc_brightness,
            },
            "off" => match action {
                Action::OnOff => LightState::turn_off,
                Action::Brightness => LightState::dec_brightness,
            },
            "toggle" => match action {
                Action::OnOff => LightState::toggle,
                Action::Brightness => LightState::inc_brightness_by_two,
            },
            _ => panic!("unknown instruction => {}", instr_str),
        })
    }
}

impl Deref for Instruction {
    type Target = fn(&mut LightState);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/* ---------- */

enum Action {
    OnOff,
    Brightness,
}

/* ---------- */

struct Command(Instruction, Coord, Coord);

impl Command {
    fn new(cmd_str: &str, action: Action) -> Self {
        let offset: usize = if cmd_str.starts_with("toggle") { 0 } else { 1 };
        let parts: Vec<&str> = cmd_str.split_whitespace().collect();

        Self(
            Instruction::new_from_str(parts[offset], action),
            Coord::from(parts[1 + offset]),
            Coord::from(parts[3 + offset]),
        )
    }
}

/* ---------- */

#[derive(Default, Debug, Copy, Clone, PartialEq)]
struct LightState(usize);

impl LightState {
    #[inline(always)]
    fn is_on(&self) -> bool {
        self.0 > 0
    }

    #[inline(always)]
    fn brightness(&self) -> usize {
        self.0
    }

    #[inline(always)]
    fn turn_on(&mut self) {
        self.0 = 1
    }

    #[inline(always)]
    fn turn_off(&mut self) {
        self.0 = 0
    }

    #[inline(always)]
    fn toggle(&mut self) {
        match self.is_on() {
            true => self.turn_off(),
            false => self.turn_on(),
        }
    }

    #[inline(always)]
    fn inc_brightness(&mut self) {
        self.0 += 1
    }

    #[inline(always)]
    fn dec_brightness(&mut self) {
        if self.0 > 0 {
            self.0 -= 1
        }
    }

    #[inline(always)]
    fn inc_brightness_by_two(&mut self) {
        self.0 += 2
    }
}

/* ---------- */

struct LightArray(Vec<LightState>);

impl LightArray {
    fn new() -> Self {
        let vec = vec![LightState::default(); ARRAY_SIZE];
        Self(vec)
    }

    fn count_lights(&self) -> usize {
        self.0.iter().filter(|light| light.is_on()).count()
    }

    fn total_brightness(&self) -> usize {
        self.0.iter().map(|light| light.brightness()).sum()
    }

    fn reset(&mut self) {
        self.0.iter_mut().for_each(|light| light.turn_off())
    }

    fn do_command(&mut self, Command(instr, from, to): Command) {
        for y in from.y()..=to.y() {
            for x in from.x()..=to.x() {
                instr(&mut self.0[index_from_coord(x, y)])
            }
        }
    }
}

/* ---------- */

fn part1(lights: &mut LightArray, input: &'static str) -> usize {
    input
        .lines()
        .map(|line| Command::new(line, Action::OnOff))
        .for_each(|cmd| lights.do_command(cmd));

    lights.count_lights()
}

/* ---------- */

fn part2(lights: &mut LightArray, input: &'static str) -> usize {
    input
        .lines()
        .map(|line| Command::new(line, Action::Brightness))
        .for_each(|cmd| lights.do_command(cmd));

    lights.total_brightness()
}

/* ---------- */

fn main() {
    let mut lights = LightArray::new();
    let input = utils::input_str!();

    println!("[PART 1] Answer = {}", part1(&mut lights, input));
    lights.reset();
    println!("[PART 2] Answer = {}", part2(&mut lights, input));
}

/* ---------- */

#[cfg(test)]
mod test {
    use crate::{Action, Command, Instruction, LightArray, LightState};

    #[test]
    fn test_instr_part1() {
        let mut light = LightState::default();

        let instr = Instruction::new_from_str("on", Action::OnOff);
        instr(&mut light);
        assert!(light.is_on());

        let instr = Instruction::new_from_str("off", Action::OnOff);
        instr(&mut light);
        assert!(!light.is_on());

        let instr = Instruction::new_from_str("toggle", Action::OnOff);
        instr(&mut light);
        assert!(light.is_on());
        let instr = Instruction::new_from_str("toggle", Action::OnOff);
        instr(&mut light);
        assert!(!light.is_on());
    }

    #[test]
    fn test_instr_part2() {
        let mut light = LightState::default();

        let instr = Instruction::new_from_str("on", Action::Brightness);
        instr(&mut light);
        assert_eq!(light.brightness(), 1);

        let instr = Instruction::new_from_str("off", Action::Brightness);
        instr(&mut light);
        assert_eq!(light.brightness(), 0);

        let instr = Instruction::new_from_str("toggle", Action::Brightness);
        instr(&mut light);
        assert_eq!(light.brightness(), 2);
        let instr = Instruction::new_from_str("toggle", Action::Brightness);
        instr(&mut light);
        assert_eq!(light.brightness(), 4);
    }

    #[test]
    fn test_part2() {
        let mut lights = LightArray::new();

        let cmd = Command::new("turn on 0,0 through 0,0", Action::Brightness);
        lights.do_command(cmd);
        assert_eq!(lights.total_brightness(), 1);

        lights.reset();
        assert_eq!(lights.total_brightness(), 0);

        let cmd = Command::new("toggle 0,0 through 999,999", Action::Brightness);
        lights.do_command(cmd);
        assert_eq!(lights.total_brightness(), 2000000);
    }
}
