const TRAVEL_TIME: u32 = 2503;

/* ---------- */

#[derive(Debug)]
struct Reindeer {
    _name: String,
    speed: u32,
    flight_time: u32,
    rest_time: u32,
    points: u32,
    travelled_distance: u32,
}

impl Reindeer {
    fn new(name: &str, speed_str: &str, ft_str: &str, rt_str: &str) -> Self {
        Self {
            _name: String::from(name),
            speed: speed_str.parse().expect("failed to parse speed"),
            flight_time: ft_str.parse().expect("failed to parse flight time"),
            rest_time: rt_str.parse().expect("failed to parse rest time"),
            points: 0,
            travelled_distance: 0,
        }
    }

    fn distance_at(&self, time: u32) -> u32 {
        let current_phase = time % (self.flight_time + self.rest_time);
        let total_phases = time / (self.flight_time + self.rest_time);
        let additional_flight_time = if current_phase > self.flight_time {
            self.flight_time
        } else {
            current_phase
        };

        (total_phases * self.flight_time * self.speed) + (additional_flight_time * self.speed)
    }

    fn inc_pts(&mut self) {
        self.points += 1
    }

    fn points(&self) -> u32 {
        self.points
    }

    fn update_distance(&mut self, time: u32) -> u32 {
        self.travelled_distance = self.distance_at(time);
        self.travelled_distance
    }

    fn travelled_distance(&self) -> u32 {
        self.travelled_distance
    }
}

/* ---------- */

fn part1(reindeers: &[Reindeer]) -> u32 {
    reindeers
        .iter()
        .map(|reindeer| reindeer.distance_at(TRAVEL_TIME))
        .max()
        .expect("couldn't find max distance")
}

/* ---------- */

fn part2(reindeers: &mut Vec<Reindeer>) -> u32 {
    for time in 1..TRAVEL_TIME {
        // println!("at {time} seconds");

        let max_distance = reindeers
            .iter_mut()
            .map(|reindeer| reindeer.update_distance(time))
            .max()
            .expect("to find the reindeer that travelled a greater distance");

        // println!("\twinner distance = {max_distance}");

        reindeers
            .iter_mut()
            // .inspect(|r| println!("\t{r:?}"))
            .filter(|reindeer| max_distance == reindeer.travelled_distance())
            .for_each(|reindeer| {
                // println!("\t{reindeer:?}");
                reindeer.inc_pts();
            })
    }

    reindeers
        .iter()
        .map(|reindeer| reindeer.points())
        .max()
        .expect("to find the maximum travelled distance")
}

/* ---------- */

fn main() {
    let mut reindeers = utils::input_str!()
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            Reindeer::new(parts[0], parts[3], parts[6], parts[13])
        })
        .collect::<Vec<Reindeer>>();

    println!("[PART 1] Answer = {}", part1(&reindeers));
    println!("[PART 2] Answer = {}", part2(&mut reindeers));
}
