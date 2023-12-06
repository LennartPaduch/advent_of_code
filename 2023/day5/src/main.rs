use std::collections::HashMap;

#[derive(Debug, Clone)]
struct MapItem {
    to: String,
    range_map: Vec<RangeItem>,
}

#[derive(Debug, Clone)]
struct RangeItem {
    src: u64,
    dest: u64,
    range: u64,
}

#[derive(Debug, Clone)]
struct Seed {
    start: u64,
    range: u64,
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let seeds: Vec<u64> = get_seeds(&input);
    let sections: Vec<(&str, &str, Vec<Vec<u64>>)> = get_sections(&input);
    let mut map: HashMap<String, MapItem> = get_map(sections);
    let mut lowest_location = std::u64::MAX;

    // Part 1:
    for seed in seeds.iter() {
        lowest_location = std::cmp::min(lowest_location, walk(*seed, "seed", &map));
    }
    println!("Part 1: {}", lowest_location);

    // Part 2:
    let ranged_seeds: Vec<Seed> = get_ranged_seeds(seeds);
    lowest_location = std::u64::MAX;

    for p in map.iter_mut() {
        p.1.range_map = insert_empty_ranges(p.1.range_map.clone());
    }

    for seed in ranged_seeds {
        let mut remaining = seed.range;
        let mut start = seed.start;
        while remaining > 0 {
            let (start_location, consumed) = walk_with_ranges(start, remaining, "seed", &map);
            start += consumed;
            remaining -= consumed;
            lowest_location = std::cmp::min(lowest_location, start_location);
        }
    }
    println!("Part 2: {}", lowest_location);
}

fn walk_with_ranges(
    value: u64,
    range: u64,
    from: &str,
    map: &HashMap<String, MapItem>,
) -> (u64, u64) {
    if let Some(map_item) = map.get(from) {
        if let Some(range_item) = map_item
            .range_map
            .iter()
            .find(|range_item| value >= range_item.src && value < range_item.src + range_item.range)
        {
            let consumed = value - range_item.src;
            let new_val = range_item.dest + consumed;
            return walk_with_ranges(
                new_val,
                std::cmp::min(range_item.range - consumed, range),
                &map_item.to,
                map,
            );
        }
        return walk_with_ranges(value, 1, &map_item.to, map);
    }
    (value, range)
}
fn get_ranged_seeds(seeds: Vec<u64>) -> Vec<Seed> {
    let mut ranged_seeds: Vec<Seed> = Vec::new();
    for (idx, _) in seeds.iter().enumerate() {
        if idx % 2 == 0 {
            ranged_seeds.push(Seed {
                start: seeds[idx],
                range: seeds[idx + 1],
            });
        }
    }
    ranged_seeds
}

fn insert_empty_ranges(mut ranges: Vec<RangeItem>) -> Vec<RangeItem> {
    ranges.sort_by(|a, b| a.src.cmp(&b.src));

    let mut start = 0;
    let mut i = 0;
    while i < ranges.len() {
        let range = ranges[i].clone();
        if range.src > start {
            ranges.insert(
                i,
                RangeItem {
                    src: start,
                    dest: start,
                    range: range.src - start,
                },
            );
            i += 1;
        }
        start = range.src + range.range;
        i += 1;
    }
    ranges
}

fn walk(value: u64, from: &str, map: &HashMap<String, MapItem>) -> u64 {
    if let Some(map_item) = map.get(from) {
        let new_from = map_item.to.clone();
        if let Some(range_item) = map_item
            .range_map
            .iter()
            .find(|range_item| value >= range_item.src && value < range_item.src + range_item.range)
        {
            let new_val = value - range_item.src + range_item.dest;
            return walk(new_val, &new_from, map);
        }
        return walk(value, &new_from, map);
    }
    value
}

fn get_map(sections: Vec<(&str, &str, Vec<Vec<u64>>)>) -> HashMap<String, MapItem> {
    let mut map: HashMap<String, MapItem> = HashMap::new();
    for section in sections.iter() {
        let to = section.1.to_string();
        let from = section.0.to_string();
        let values = section.2.clone();
        let range_items = values
            .iter()
            .map(|value| RangeItem {
                src: value[1],
                dest: value[0],
                range: value[2],
            })
            .collect::<Vec<RangeItem>>();

        map.insert(
            from,
            MapItem {
                to,
                range_map: range_items,
            },
        );
    }
    map
}

fn get_sections(input: &str) -> Vec<(&str, &str, Vec<Vec<u64>>)> {
    input
        .split("\n\n")
        .skip(1) // Skip the first line containing seeds
        .map(|section| {
            let mut lines = section.lines();
            let header_line = lines.next().expect("No header found!");
            let header_parts: Vec<&str> = header_line
                .split_whitespace()
                .next()
                .expect("Invalid header format")
                .split("-")
                .collect();

            let from = header_parts[0];
            let to = header_parts[2];

            let values = lines
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.parse::<u64>().expect("Invalid number format"))
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<Vec<u64>>>();
            (from, to, values)
        })
        .collect()
}

fn get_seeds(input: &str) -> Vec<u64> {
    let seeds: Vec<u64> = input
        .lines()
        .nth(0)
        .expect("No input data!")
        .split(":")
        .nth(1)
        .expect("No seeds found!")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    seeds
}
