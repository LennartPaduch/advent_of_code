fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sum = quick_maths(&input, false);
    println!("Part 1: {}", sum);
    sum = quick_maths(&input, true);
    println!("Part 2: {}", sum);
}

fn quick_maths(input: &str, reverse: bool) -> i32 {
    let mut sum = 0;
    let lines = input.lines();
    for line in lines {
        let mut values: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        if reverse {
            values.reverse();
        }
        sum += extrapolate(&values, *values.last().expect("Expected last vec element"));
    }
    sum
}

fn extrapolate(values: &Vec<i32>, mut sum: i32) -> i32 {
    let mut vec: Vec<i32> = Vec::new();
    if values.iter().all(|&x| x == 0) {
        return sum;
    }
    for i in 0..values.len() - 1 {
        vec.push(values[i + 1] - values[i]);
    }
    sum += vec.last().expect("Expected last vec element");
    return extrapolate(&vec, sum);
}
