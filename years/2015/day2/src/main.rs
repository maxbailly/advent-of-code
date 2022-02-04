const fn rect_area(l: u32, w: u32) -> u32 {
    l * w
}

const fn rect_perimeter(l: u32, w: u32) -> u32 {
    (l + w) * 2
}

const fn box_volume(l: u32, w: u32, h: u32) -> u32 {
    l * w * h
}

/* ---------- */

#[derive(Default, Debug)]
struct Present {
    lenght: u32,
    width: u32,
    height: u32
}

impl Present {
    fn paper_amount(&self) -> u32 {
        let side_areas = [
            rect_area(self.lenght, self.width),
            rect_area(self.width, self.height),
            rect_area(self.lenght, self.height),
        ];

        let min_area = side_areas.iter().min().expect("failed to find minimum area");

        side_areas[0] * 2 + side_areas[1] * 2
            + side_areas[2] * 2
            + min_area
    }

    fn ribbon_amount(&self) -> u32 {
        let perimeters = [
            rect_perimeter(self.lenght, self.width),
            rect_perimeter(self.width, self.height),
            rect_perimeter(self.lenght, self.height),
        ];

        let min_perim = perimeters.iter().min().expect("expected a min value");

        min_perim + box_volume(self.lenght, self.width, self.height)
    }
}

impl From<&'static str> for Present {
    fn from(present_str: &'static str) -> Self {
        let parts = present_str.split('x').collect::<Vec<&'static str>>();

        Self {
            lenght: parts[0].parse().expect("expected a valid length"),
            width: parts[1].parse().expect("expected a valid width"),
            height: parts[2].parse().expect("expected a valid height")
        }
    }
}

/* ---------- */

fn part1(input: &'static str) -> u32 {
    input.lines()
        .map(Present::from)
        .map(|present| present.paper_amount())
        .sum()
}

/* ---------- */

fn part2(input: &'static str) -> u32 {
    input.lines()
        .map(Present::from)
        .map(|present| present.ribbon_amount())
        .sum()
}

/* ---------- */

fn main() {
    let input = utils::input_str!("part1.txt");
    utils::answer!(input)
}

/* ---------- */

#[cfg(test)]
mod test {
    use crate::Present;

    #[test]
    fn part1() {
        let p = Present::from("2x3x4");
        assert_eq!(58, p.paper_amount());

        let p = Present::from("1x1x10");
        assert_eq!(p.paper_amount(), 43);
    }

    #[test]
    fn part2() {
        let p = Present::from("2x3x4");
        assert_eq!(p.ribbon_amount(), 34);

        let p = Present::from("1x1x10");
        assert_eq!(p.ribbon_amount(), 14);
    }
}
