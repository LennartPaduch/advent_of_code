fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let patterns: Vec<&str> = extract_patterns(&input);
    let total_sum = patterns
        .iter()
        .fold(0, |sum, pattern| sum + calc_sum(&pattern, 1));
    println!("Part 1 - Total sum: {}", total_sum);
    let total_sum = patterns
        .iter()
        .fold(0, |sum, pattern| sum + calc_sum(&pattern, 2));
    println!("Part 2 - Total sum: {}", total_sum);
}

fn transpose_pattern(lines: &Vec<&str>) -> Vec<String> {
    let mut transposed_pattern = Vec::new();
    for i in 0..lines[0].len() {
        let mut row = String::new();
        for line in lines {
            row.push(line.chars().nth(i).unwrap());
        }
        transposed_pattern.push(row);
    }
    transposed_pattern
}

fn calc_sum(pattern: &str, part: usize) -> usize {
    let lines = pattern.lines().collect();
    let mut sum = find_mirror_idx(&lines, part) * 100;
    let transposed_vec = transpose_pattern(&lines);
    let transposed_lines: Vec<&str> = transposed_vec.iter().map(AsRef::as_ref).collect();
    sum += find_mirror_idx(&transposed_lines, part);
    sum
}

fn find_mirror_idx(lines: &Vec<&str>, part: usize) -> usize {
    for i in 1..lines.len() {
        let (above_slice, mut below) = lines.split_at(i);

        let mut above: Vec<&str> = above_slice.to_vec();
        above.reverse();

        let smaller_arr_len = above.len().min(below.len());
        let above = &above[..smaller_arr_len];
        below = &below[..smaller_arr_len];

        if part == 2 {
            let mut sum = 0;
            for i in 0..smaller_arr_len {
                let above_chars = above[i].chars().collect::<Vec<char>>();
                let below_chars = below[i].chars().collect::<Vec<char>>();
                for c in 0..above[i].len() {
                    if above_chars[c] != below_chars[c] {
                        sum += 1;
                    }
                }
            }
            if sum == 1 {
                return i;
            }
        } else {
            if above == below {
                return i;
            }
        }
    }
    0
}

fn extract_patterns(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}
