mod inputs;
use inputs::*;

/* ---------- */

const INPUT: &str = utils::input_str!();

/* ---------- */

#[inline]
fn is_ascii_number(s: &'static str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

/* ---------- */

#[inline]
fn input_from_str(input: &'static str, wires: &mut Wires) -> Box<dyn Input> {
    if is_ascii_number(input) {
        let value = input.parse::<u16>().expect("a valid u16 for signal value");
        Signal::new(value).into_input()
    } else {
        wires.entry(input).into_input()
    }
}

/* ---------- */

fn assemble_circuit(line: &'static str, wires: &mut Wires) {
    let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
    let output = match parts.last() {
        Some(part) => *part,
        None => panic!("invalid line {line}")
    };

    let output = wires.entry(output);

    match parts.len() {
        3 => {
            let input = input_from_str(parts[0], wires);
            output.plug(input);
        }
        4 => {
            let input = input_from_str(parts[1], wires);
            let not_gate = Not::new(input);

            output.plug(not_gate.into_input());
        }
        5 if parts[1] == "AND" => {
            let left = input_from_str(parts[0], wires);
            let right = input_from_str(parts[2], wires);
            let and_gate = And::new(left, right);

            output.plug(and_gate.into_input());
        }
        5 if parts[1] == "OR" => {
            let left = input_from_str(parts[0], wires);
            let right = input_from_str(parts[2], wires);
            let or_gate = Or::new(left, right);

            output.plug(or_gate.into_input());
        }
        5 if parts[1] == "LSHIFT" => {
            let left = input_from_str(parts[0], wires);
            let right = input_from_str(parts[2], wires);
            let lshift_gate = LeftShift::new(left, right);

            output.plug(lshift_gate.into_input());
        }
        5 if parts[1] == "RSHIFT" => {
            let left = input_from_str(parts[0], wires);
            let right = input_from_str(parts[2], wires);
            let rshift_gate = RightShift::new(left, right);

            output.plug(rshift_gate.into_input());
        }
        _ => {
            panic!("Parse error => {line}")
        }
    }

    wires.push(output)
}

/* ---------- */

fn part1(wires: &mut Wires) -> u16 {
    INPUT.lines().for_each(|line| {
        assemble_circuit(line, wires)
    });

    let wire = wires.wire("a").expect("a existing wire id 'a'");
    wire.output().expect("a vaild output for 'a' wire")
}

/* ---------- */

fn part2(wires: &mut Wires, new_signal: u16) -> u16 {
    wires.reset();

    let b = wires.wire("b").expect("a existing wire id 'b'");
    b.plug(Signal::new(new_signal).into_input());

    let wire = wires.wire("a").expect("a existing wire id 'a'");
    wire.output().expect("a vaild output for 'a' wire")
}

/* ---------- */

fn main() {
    let mut wires = Wires::new();

    let ret = part1(&mut wires);
    println!("[PART 1] Answer = {}", ret);
    println!("[PART 2] Answer = {}", part2(&mut wires, ret));
}
