#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // Part 1:
    let races = get_races_vec(&input);
    let mut product = 1.0;
    for race in races {
        product *= process_race(race);
    }
    println!("Part 1: {}", product);

    // Part 2:
    let distance: String = input
        .lines()
        .nth(1)
        .expect("Missing second line for Distance")
        .split(":")
        .nth(1)
        .expect("Missing : in Time Line")
        .split_whitespace()
        .collect();

    let time: String = input
        .lines()
        .nth(0)
        .expect("Missing first line for time")
        .split(":")
        .nth(1)
        .expect("Missing : in Distance Line")
        .split_whitespace()
        .collect();

    let solutions = process_race(Race {
        time: time.parse().unwrap(),
        distance: distance.parse().unwrap(),
    });
    println!("Part 2: {}", solutions);
}

fn process_race(race: Race) -> f64 {
    let time = race.time as f64;
    let distance = race.distance as f64;

    // distance = (time - charge_time) * velocity (substitute velocity with charge_time = velocity)
    // <=> distance = (time - charge_time) * charge_time
    // <=> 0 = charge_time^2 - time * charge_time + distance
    // quadratic formula: x = (-b +- sqrt(b^2 - 4ac)) / 2a
    // -> a=1, x=charge_time, b=-time, c=distance
    // we solve for x=charge_time to match the record distance
    let root_part = (time.powi(2) - 4.0 * distance).sqrt();
    let min_charge_time = (((time - root_part) / 2.0) + 1.0).floor();
    let max_charge_time = time - min_charge_time;
    let nr_solutions = max_charge_time - min_charge_time + 1.0;
    nr_solutions
}
fn get_races_vec(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();
    let time_data = lines[0].split(":").nth(1).unwrap().split_whitespace();
    let distance_data = lines[1].split(":").nth(1).unwrap().split_whitespace();

    let time_numbers: Vec<u64> = time_data.map(|s| s.parse().unwrap()).collect();
    let distance_numbers: Vec<u64> = distance_data.map(|s| s.parse().unwrap()).collect();

    time_numbers
        .iter()
        .zip(distance_numbers.iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect()
}
