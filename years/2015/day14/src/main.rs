const TRAVEL_TIME: u32 = 2503;

/* ---------- */

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    flight_time: u32,
    rest_time: u32
}

impl Reindeer {
    fn new(speed_str: &str, ft_str: &str, rt_str: &str) -> Self {
        Self {
            speed: speed_str.parse().expect("failed to parse speed"),
            flight_time: ft_str.parse().expect("failed to parse flight time"),
            rest_time: rt_str.parse().expect("failed to parse rest time")
        }
    }

    fn distance_at(&self, time: u32) -> u32 {
        let current_phase = time % (self.flight_time + self.rest_time);
        let total_phases = time / (self.flight_time + self.rest_time);
        let additional_flight_time = if current_phase > self.flight_time { self.flight_time } else { current_phase };

        (total_phases * self.flight_time * self.speed) + (additional_flight_time * self.speed)
    }
}

/* ---------- */

fn main() {
    let max = utils::inputs_str!("1").split('\n')
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let reindeer = Reindeer::new(parts[3], parts[6], parts[13]);

            reindeer.distance_at(TRAVEL_TIME)
        })
        .max()
        .expect("couldn't find max distance");

    println!("result = {}", max);
}
