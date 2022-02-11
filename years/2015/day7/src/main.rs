use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

/* ----------- */

struct WireInternal {
    id: &'static str,
    source: Option<Source>,
    signal: Option<u16>
}

impl WireInternal {
    fn new(id: &'static str) -> Self {
        let signal = id.parse().ok();

        Self {
            id,
            source: None,
            signal
        }
    }

    fn id(&self) -> &'static str {
        self.id
    }

    fn signal(&mut self) -> Option<u16> {
        if self.signal.is_some() {
            return self.signal
        }

        let source = self.source.as_ref().expect("expected a valid source");
        self.signal = source.eval();

        self.signal
    }

    fn set_source(&mut self, source: Source) {
        self.source = Some(source)
    }
}

impl std::fmt::Debug for WireInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let src = match &self.source {
            Some(src) => format!("{:?}", src),
            _ => String::from("None")
        };

        let sig = match self.signal {
            Some(signal) => format!("{}", signal),
            _ => String::from("None")
        };

        write!(f, "Wire(id: {}, source: {}, signal: {})", self.id, src, sig)
    }
}

/* ----------- */

struct Wire(Rc<RefCell<WireInternal>>);

impl Wire {
    fn new(id: &'static str) -> Self {
        Self(Rc::new(RefCell::new(WireInternal::new(id))))
    }

    fn id(&self) -> &'static str {
        self.0.borrow().id()
    }

    fn signal(&self) -> Option<u16> {
        self.0.borrow_mut().signal()
    }

    fn set_source(&self, source: Source) {
        self.0.borrow_mut().set_source(source)
    }
}

impl std::fmt::Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0.deref().borrow())
    }
}

impl Clone for Wire {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

/* ----------- */

#[derive(Default)]
struct Wires(Vec<Wire>);

impl Wires {
    fn new() -> Self {
        Self::default()
    }

    fn find_ref_or_new(&mut self, id: &'static str) -> Wire {
        match self.0.iter().find(|it | it.id() == id) {
            Some(wire) => wire.clone(),
            None => {
                let new = Wire::new(id);
                self.0.push(new.clone());

                new
            }
        }
    }

    fn wire_signal(&self, id: &'static str) -> Option<u16> {
        self.0.iter().find(|it | it.id() == id)
            .expect("expecter a known wire id")
            .signal()
    }

    #[allow(dead_code)]
    fn eval(&self) {
        self.0.iter().for_each(|wire| { wire.signal(); } );
    }
}

impl std::fmt::Debug for Wires {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

/* ----------- */

enum Source {
    Signal { signal: u16 },
    Wire { input: Wire },
    Not { input: Wire },
    And { input1: Wire, input2: Wire },
    Or { input1: Wire, input2: Wire },
    LeftShift { input: Wire, amount: u8 },
    RightShift { input: Wire, amount: u8 }
}

impl Source {
    fn new_signal(signal: u16) -> Self {
        Self::Signal { signal }
    }

    fn new_wire(input: Wire) -> Self {
        Self::Wire { input }
    }

    fn new_gate(gate_type: &'static str, input1: Wire, input2: Option<Wire>) -> Self {
        match gate_type {
            "NOT" => Self::Not { input: input1 },
            "AND" => {
                let input2 = input2.expect("AND gate expect 2 inputs");
                Self::And { input1, input2 }
            }
            "OR" => {
                let input2 = input2.expect("OR gate expect 2 inputs");
                Self::Or { input1, input2 }
            }
            _ => panic!("unknown gate type {}", gate_type)
        }
    }

    fn new_shift(gate_type: &'static str, input: Wire, amount: u8) -> Self {
        match gate_type {
            "LSHIFT" => Self::LeftShift { input, amount },
            "RSHIFT" => Self::RightShift { input, amount },
            _ => panic!("unknown shift type {}", gate_type)
        }
    }

    fn eval(&self) -> Option<u16>{
        match self {
            Self::Signal { signal } => Some(*signal),
            Self::Wire { input } => input.signal(),
            Self::Not { input } => {
                input.signal().map(|val| !val)
            }
            Self::And { input1, input2 } => {
                let sig1 = input1.signal();
                let sig2 = input2.signal();

                match (sig1, sig2) {
                    (Some(val1), Some(val2)) => Some(val1 & val2),
                    _ => None
                }
            }
            Self::Or { input1, input2 } => {
                let sig1 = input1.signal();
                let sig2 = input2.signal();

                match (sig1, sig2) {
                    (Some(val1), Some(val2)) => Some(val1 | val2),
                    _ => None
                }
            }
            Self::LeftShift { input, amount } => input.signal().map(|val| val << *amount),
            Self::RightShift { input, amount } => input.signal().map(|val| val >> *amount)
        }
    }
}

impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Signal { signal } =>
                write!(f, "Source::Signal(in: {})", signal),
            Self::Wire { input } =>
                write!(f, "Source::Wire(in: {})", input.id()),
            Self::Not { input } =>
                write!(f, "Source::Not(in: {})", input.id()),
            Self::And { input1, input2 } =>
                write!(f, "Source::And(in1: {}, in2: {})", input1.id(), input2.id()),
            Self::Or { input1, input2 } =>
                write!(f, "Source::Or(in1: {}, in2: {})", input1.id(), input2.id()),
            Self::LeftShift { input, amount } =>
                write!(f, "Source::LeftShift(in: {}, amnt: {})", input.id(), amount),
            Self::RightShift { input, amount } =>
                write!(f, "Source::RightShift(in: {}, amnt: {})", input.id(), amount),
        }
    }
}

/* ----------- */

fn parse_line(line: &'static str, wires: &mut Wires) {
    let parts = line.split_whitespace().collect::<Vec<&'static str>>();
    let output = parts.last().expect("expected something when retrievng the output wire");

    let source = match parts.len() {
        3 => {
            match parts[0].parse() {
                Ok(signal) => Source::new_signal(signal),
                _ => {
                    let input = wires.find_ref_or_new(parts[0]);
                    Source::new_wire(input)
                }
            }
        }
        4 => {
            let input = wires.find_ref_or_new(parts[1]);
            Source::new_gate(parts[0], input, None)
        }
        5 if parts[1] == "AND" || parts[1] == "OR" => {
            let input1 = wires.find_ref_or_new(parts[0]);
            let input2 = wires.find_ref_or_new(parts[2]);

            Source::new_gate(parts[1], input1, Some(input2))
        }
        5 if parts[1] == "LSHIFT" || parts[1] == "RSHIFT" => {
            let input = wires.find_ref_or_new(parts[0]);
            let amount = parts[2].parse()
                .expect("expect a valid u8 when parsing shift amount value");

            Source::new_shift(parts[1], input, amount)
        }
        _ => panic!("that's some weird line => {}", line)
    };

    let wire = wires.find_ref_or_new(output);
    wire.set_source(source);
}

/* ----------- */

fn main() {
    let mut wires = Wires::new();

    utils::input_str!().lines().for_each(|line| parse_line(line, &mut wires));

    let res = wires.wire_signal("a").unwrap();
    println!("result = {}", res)
}
