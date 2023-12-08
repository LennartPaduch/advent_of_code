use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (instructions, node_map, start_nodes, end_nodes) = parse_input(&input);
    let count = calc_distance("AAA", "ZZZ", &instructions, &node_map);
    println!("Part 1: {}", count);
    let distances = find_paths(&node_map, start_nodes, end_nodes, instructions);
    let final_distance: u64 = lcm(distances);
    println!("Part 2: {}", final_distance);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(distances: Vec<u64>) -> u64 {
    let mut lcm = distances[0];
    for i in 1..distances.len() {
        let gcd = gcd(lcm, distances[i] as u64);
        lcm = (lcm * distances[i] as u64) / gcd;
    }
    lcm
}

fn find_paths(
    node_map: &HashMap<&str, (&str, &str)>,
    start_nodes: Vec<&str>,
    end_nodes: Vec<&str>,
    instructions: Vec<u16>,
) -> Vec<u64> {
    let mut distances: Vec<u64> = Vec::new();
    for start in start_nodes {
        for end in end_nodes.iter() {
            if let Some(_reachable) = is_node_reachable(&node_map, start, end) {
                distances.push(calc_distance(start, end, &instructions, &node_map));
                break;
            }
        }
    }
    distances
}

fn is_node_reachable(
    node_map: &HashMap<&str, (&str, &str)>,
    start: &str,
    end: &str,
) -> Option<bool> {
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();
    queue.push_back(start);
    visited.insert(start);

    while let Some(curr_node) = queue.pop_front() {
        if curr_node == end {
            return Some(true);
        }
        if let Some(&(left, right)) = node_map.get(curr_node) {
            let mut target_node = left;
            if !visited.contains(target_node) {
                queue.push_back(target_node);
                visited.insert(target_node);
            }
            target_node = right;
            if !visited.contains(target_node) {
                queue.push_back(target_node);
                visited.insert(target_node);
            }
        }
    }

    None
}

fn calc_distance(
    start: &str,
    end: &str,
    instructions: &Vec<u16>,
    node_map: &HashMap<&str, (&str, &str)>,
) -> u64 {
    let mut distance: usize = 0;
    let mut curr_node = start;
    let nr_instructions = instructions.len();
    while curr_node != end {
        let direction: u16 = instructions[distance % nr_instructions];
        let (left, right) = node_map.get(curr_node).unwrap();
        curr_node = if direction == 0 { left } else { right };
        distance += 1;
    }
    distance as u64
}

fn parse_input(input: &str) -> (Vec<u16>, HashMap<&str, (&str, &str)>, Vec<&str>, Vec<&str>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let instructions: Vec<u16> = parts[0]
        .trim()
        .chars()
        .map(|c| if c == 'L' { return 0 } else { return 1 })
        .collect();
    let nodes: Vec<&str> = parts[1].split("\n").collect();
    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut start_nodes: Vec<&str> = Vec::new();
    let mut end_nodes: Vec<&str> = Vec::new();
    for node in nodes.iter() {
        if node == &"" {
            continue;
        }
        let parts: Vec<&str> = node.split(" ").map(|s| s.trim()).collect();
        let node_name = parts[0];
        let left_node = parts[2]
            .split(",")
            .nth(0)
            .unwrap()
            .split("(")
            .nth(1)
            .unwrap();
        let right_node = parts[3].split(")").nth(0).unwrap();
        if node_name.chars().last().unwrap() == 'A' {
            start_nodes.push(node_name);
        }
        if node_name.chars().last().unwrap() == 'Z' {
            end_nodes.push(node_name);
        }

        node_map.insert(node_name, (left_node, right_node));
    }
    (instructions, node_map, start_nodes, end_nodes)
}
