const INPUT: &str = "hepxcrrq";

/* ---------- */

#[inline(always)]
const fn inc_char(c: u8) -> (u8, bool) {
    match c {
        b'z' => (b'a', true),
        b'h' | b'k' | b'n' => (c + 2, false),
        _ => (c + 1, false),
    }
}

/* ---------- */

fn increment_passwd(passwd: &[u8]) -> String {
    let len = passwd.len();
    let mut ret = String::with_capacity(len);
    let mut should_increment = true;

    passwd.iter().rev().for_each(|&c| {
        let new_char = if should_increment {
            let (new_char, retain) = inc_char(c);

            should_increment = retain;
            new_char
        } else if c == b'i' || c == b'l' || c == b'o' {
            ret = String::from_utf8(vec![b'a'; ret.len()]).expect("failed to fill string with a's");
            c + 1
        } else {
            c
        };

        ret.insert(0, new_char as char);
    });

    if should_increment {
        ret.push('a')
    }

    ret
}

/* ---------- */

#[derive(Default, Debug)]
struct Password {
    inner: String,
    nb_pairs: u8,
    has_inc_straight: bool,
}

impl Password {
    fn next(&mut self) {
        self.nb_pairs = 0;
        self.has_inc_straight = false;
        self.inner = increment_passwd(self.inner.as_bytes());

        self.check()
    }

    fn check(&mut self) {
        let mut last = 0u8;

        let mut curr_inc_straight = 1u8;
        let mut overlap = false;

        self.inner.as_bytes().iter().for_each(|&c| {
            if c == last && !overlap {
                self.nb_pairs += 1;
                overlap = true;
            } else if overlap {
                overlap = false;
            }

            if !self.has_inc_straight && c == last + 1 {
                curr_inc_straight += 1;
                self.has_inc_straight = curr_inc_straight >= 3;
            } else if !self.has_inc_straight && c != last + 1 {
                curr_inc_straight = 1;
            }

            last = c;
        });
    }

    fn good(&self) -> bool {
        self.nb_pairs >= 2 && self.has_inc_straight
    }
}

impl From<&str> for Password {
    fn from(passwd: &str) -> Self {
        Self {
            inner: String::from(passwd),
            nb_pairs: 0,
            has_inc_straight: false,
        }
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/* ---------- */

fn part1(mut passwd: Password) -> Password {
    loop {
        passwd.next();
        if passwd.good() {
            break;
        }
    }

    passwd
}

/* ---------- */

fn main() {
    let passwd = Password::from(INPUT);

    let passwd = part1(passwd);
    println!("[PART 1] Answer = {passwd}");

    let passwd = part1(passwd);
    println!("[PART 2] Answer = {passwd}");
}
