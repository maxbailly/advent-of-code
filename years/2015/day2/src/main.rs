fn area_from_str(line: &str) -> u32 {
    let mut dim = [0; 3];

    line.split('x')
        .map(|val| val.parse::<u32>().expect("failed to parse"))
        .enumerate()
        .for_each(|(i, val)| if i < 3 { dim[i] = val });

    let side_areas = [
        dim[0] * dim[1],
        dim[1] * dim[2],
        dim[0] * dim[2],
    ];

    let min_area = side_areas.iter().min().expect("failed to find minimum area");

    side_areas[0] + side_areas[1]
        + side_areas[2]
        + min_area
}

fn main() {
    let res: u32 = utils::input_str!("1")
        .split_whitespace()
        .map(area_from_str)
        .sum();

    println!("{}", res)
}
