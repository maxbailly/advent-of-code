use std::collections::HashSet;

/* ---------- */

#[derive(Debug, Default)]
struct Transform {
    from: &'static str,
    to: &'static str
}

impl Transform {
    fn from(&self) -> &'static str {
        self.from
    }

    fn to(&self) -> &'static str {
        self.to
    }
}

impl From<&'static str> for Transform {
    fn from(trfrm_str: &'static str) -> Self {
        let parts = trfrm_str.split(" => ").collect::<Vec<&'static str>>();

        if parts.len() != 2 {
            panic!("expected a valid transform")
        }

        Self {
            from: parts[0],
            to: parts[1]
        }
    }
}

/* ---------- */

#[derive(Debug, Default, Clone)]
struct Molecule(String);

impl Molecule {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn replacements(&self, transform: Transform) -> Vec<String> {
        let needle_len = transform.from().len();

        self.0.match_indices(transform.from())
            .map(|(index, _)| {
                let mut new = self.0.clone();
                new.replace_range(index..needle_len + index, transform.to());
                new
            })
            .collect::<Vec<String>>()
    }
}

impl From<&'static str> for Molecule {
    fn from(mol_str: &'static str) -> Self {
        Self(mol_str.to_string())
    }
}

impl From<String> for Molecule {
    fn from(mol_str: String) -> Self {
        Self(mol_str)
    }
}

/* ---------- */

fn main() {
    let mut mol = Molecule::default();
    let mut unique_replacements = HashSet::new();

    utils::input_str!("1")
        .split('\n')
        .filter(|line| !line.is_empty())
        .rev()
        .for_each(|line| {
            if !line.contains(" => ") && mol.is_empty() {
                mol = line.into();
                return
            }

            let transform: Transform = line.into();
            let replacements = mol.replacements(transform);

            unique_replacements.extend(replacements.into_iter());
        });

    println!("result = {}", unique_replacements.len())
}
