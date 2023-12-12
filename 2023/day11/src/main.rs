fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (row_has_galaxy, col_has_galaxy, galaxy_cords) = parse_input(&input);
    let mut sum = travel(&galaxy_cords, &row_has_galaxy, &col_has_galaxy, 1);
    println!("Part 1: {}", sum);
    sum = travel(&galaxy_cords, &row_has_galaxy, &col_has_galaxy, 999_999);
    println!("Part 2: {}", sum);
}

fn get_expansion_vecs(
    row_has_galaxy: &Vec<bool>,
    col_has_galaxy: &Vec<bool>,
) -> (Vec<u64>, Vec<u64>) {
    let mut sum: u64 = 0;
    let empty_rows_till_row =
        row_has_galaxy
            .iter()
            .fold(Vec::new(), |mut acc: Vec<u64>, &has_galaxy| {
                if !has_galaxy {
                    sum += 1;
                }
                acc.push(sum);
                acc
            });
    let empty_cols_till_col =
        col_has_galaxy
            .iter()
            .fold(Vec::new(), |mut acc: Vec<u64>, &has_galaxy| {
                if !has_galaxy {
                    sum += 1;
                }
                acc.push(sum);
                acc
            });
    (empty_rows_till_row, empty_cols_till_col)
}

fn travel(
    galaxy_cords: &Vec<(usize, usize)>,
    row_has_galaxy: &Vec<bool>,
    col_has_galaxy: &Vec<bool>,
    expansion_factor: u64,
) -> u64 {
    let (empty_rows_till_row, empty_cols_till_col) =
        get_expansion_vecs(row_has_galaxy, col_has_galaxy);
    let mut sum = 0;
    for (i, &(y1, x1)) in galaxy_cords.iter().enumerate() {
        for &(y2, x2) in &galaxy_cords[i + 1..] {
            let expanded_rows = empty_rows_till_row[y1.max(y2)] - empty_rows_till_row[y1.min(y2)];
            let expanded_cols = empty_cols_till_col[x1.max(x2)] - empty_cols_till_col[x1.min(x2)];
            let dx = x1.abs_diff(x2) as u64 + expanded_rows * expansion_factor;
            let dy = y1.abs_diff(y2) as u64 + expanded_cols * expansion_factor;
            sum += dx + dy;
        }
    }
    sum
}

fn parse_input(input: &str) -> (Vec<bool>, Vec<bool>, Vec<(usize, usize)>) {
    let mut row_has_galaxy: Vec<bool> = vec![false; input.lines().count()];
    let mut col_has_galaxy: Vec<bool> = vec![false; input.lines().next().unwrap().chars().count()];
    let mut galaxy_cords: Vec<(usize, usize)> = Vec::new();

    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(k, c)| {
            if c == '#' {
                galaxy_cords.push((i, k));
                row_has_galaxy[i] = true;
                col_has_galaxy[k] = true;
            }
        });
    });
    (row_has_galaxy, col_has_galaxy, galaxy_cords)
}
