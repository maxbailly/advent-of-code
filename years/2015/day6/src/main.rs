const LENGHT: usize = 1000;
const ARRAY_SIZE: usize = LENGHT*LENGHT;

const fn index_from_coord(x: u16, y: u16) -> usize {
    y as usize * LENGHT + x as usize
}

/* ---------- */

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

        Self (
            split[0].parse::<u16>().unwrap(),
            split[1].parse::<u16>().unwrap()
        )
    }
}

/* ---------- */

enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
    Unknown
}

impl From<&str> for Instruction {
    fn from(data: &str) -> Self {
        match data {
            "on" => Self::TurnOn,
            "off" => Self::TurnOff,
            "toggle" => Self::Toggle,
            _ => Self::Unknown
        }
    }
}

/* ---------- */

struct Command(Instruction, Coord, Coord);

impl From<&str> for Command {
    fn from(data: &str) -> Self {
        let offset: usize = if data.starts_with("toggle") { 0 } else { 1 };
        let parts: Vec<&str> = data.split_whitespace().collect();

        Self (
            Instruction::from(parts[offset]),
            Coord::from(parts[1 + offset]),
            Coord::from(parts[3 + offset])
        )
    }
}

/* ---------- */

enum LightState {
    On,
    Off
}

impl LightState {
    fn is_on(&self) -> bool {
        matches!(self, Self::On)
    }

    fn turn_on(&mut self) {
        *self = Self::On
    }

    fn turn_off(&mut self) {
        *self = Self::Off
    }

    fn toggle(&mut self) {
        match self {
            Self::On => *self = Self::Off,
            Self::Off => *self = Self::On
        }
    }
}

impl Default for LightState {
    fn default() -> Self {
        Self::Off
    }
}

/* ---------- */

struct LightArray(Vec<LightState>);

impl LightArray {
    fn new() -> Self {
        let mut arr = Vec::with_capacity(ARRAY_SIZE);

        for _ in 0..ARRAY_SIZE {
            arr.push(LightState::default())
        }

        Self(arr)
    }

    fn count_lights(&self) -> usize {
        self.0.iter()
            .filter(|light|  light.is_on())
            .count()
    }

    fn do_command(&mut self, Command(instr, from, to): Command) {
        let fnc = match instr {
            Instruction::TurnOn => LightState::turn_on,
            Instruction::TurnOff => LightState::turn_off,
            Instruction::Toggle => LightState::toggle,
            Instruction::Unknown => return
        };

        for y in from.y()..=to.y() {
            for x in from.x()..=to.x() {
                fnc(&mut self.0[index_from_coord(x, y)])
            }
        }
    }
}

/* ---------- */

fn main() {
    let mut lights = LightArray::new();

    utils::inputs_str!("1").split('\n')
        .map(Command::from)
        .for_each(|cmd| lights.do_command(cmd));

    println!("result {}", lights.count_lights())
}
