const OUR_AUNT: Aunt = Aunt {
    id: 0,
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1)
};

/* ---------- */

#[derive(Default, Debug)]
struct Aunt {
    id: u16,
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>
}

impl Aunt {
    fn new(id: u16) -> Self {
        Self {
            id,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None
        }
    }
}

impl From<&str> for Aunt {
    fn from(line: &str) -> Self {
        let parts = line.split_once(": ").expect("failed to split id from desc");
        let id = parts.0.split_once(' ')
            .expect("failed to get aunt id").1
            .parse().expect("failed to parse aunt id");
        let mut aunt = Self::new(id);

        parts.1.split(", ").for_each(|desc| {
            let (thing, number_str) = desc.split_once(": ").expect("failed to split desc");
            let number = number_str.parse().expect("failed to parse thing number");

            match thing {
                "children" => aunt.children = Some(number),
                "cats" => aunt.cats = Some(number),
                "samoyeds" => aunt.samoyeds = Some(number),
                "pomeranians" => aunt.pomeranians = Some(number),
                "akitas" => aunt.akitas = Some(number),
                "vizslas" => aunt.vizslas = Some(number),
                "goldfish" => aunt.goldfish = Some(number),
                "trees" => aunt.trees = Some(number),
                "cars" => aunt.cars = Some(number),
                "perfumes" => aunt.perfumes = Some(number),
                _ => panic!("unknown thing"),
            }
        });

        aunt
    }
}

/* ---------- */

fn main() {
    utils::input_str!("1").split('\n').map(Aunt::from)
        .filter(|aunt| aunt.children == OUR_AUNT.children || aunt.children.is_none())
        .filter(|aunt| aunt.cats == OUR_AUNT.cats || aunt.cats.is_none())
        .filter(|aunt| aunt.samoyeds == OUR_AUNT.samoyeds || aunt.samoyeds.is_none())
        .filter(|aunt| aunt.pomeranians == OUR_AUNT.pomeranians || aunt.pomeranians.is_none())
        .filter(|aunt| aunt.akitas == OUR_AUNT.akitas || aunt.akitas.is_none())
        .filter(|aunt| aunt.vizslas == OUR_AUNT.vizslas || aunt.vizslas.is_none())
        .filter(|aunt| aunt.goldfish == OUR_AUNT.goldfish || aunt.goldfish.is_none())
        .filter(|aunt| aunt.trees == OUR_AUNT.trees || aunt.trees.is_none())
        .filter(|aunt| aunt.cars == OUR_AUNT.cars || aunt.cars.is_none())
        .filter(|aunt| aunt.perfumes == OUR_AUNT.perfumes || aunt.perfumes.is_none())
        .for_each(|aunt| println!("result = {}", aunt.id));
}
