struct Beam {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Beam {
    fn move_forward(&mut self) {
        match self.direction {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
        self.move_forward();
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Part 1:
    let sum = calc_energized_tiles(&grid, 0, 0, Direction::Right);
    println!("Part 1: {}", sum);

    // Part 2:
    let mut max = 0;
    for i in 0..grid.len() {
        max = max.max(calc_energized_tiles(&grid, 0, i as isize, Direction::Right));
        max = max.max(calc_energized_tiles(
            &grid,
            (grid[0].len() - 1) as isize,
            i as isize,
            Direction::Left,
        ));
    }
    for i in 0..grid[0].len() {
        max = max.max(calc_energized_tiles(&grid, i as isize, 0, Direction::Down));
        max = max.max(calc_energized_tiles(
            &grid,
            i as isize,
            (grid.len() - 1) as isize,
            Direction::Up,
        ));
    }
    println!("Part 2: {}", max);
}

fn calc_energized_tiles(
    grid: &Vec<Vec<char>>,
    starting_x: isize,
    starting_y: isize,
    starting_direction: Direction,
) -> usize {
    let mut visited_grid: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; grid[0].len()]; grid.len()];
    walk(
        &grid,
        Beam {
            x: starting_x,
            y: starting_y,
            direction: starting_direction,
        },
        &mut visited_grid,
    );
    visited_grid.iter().fold(0, |sum, row| {
        row.iter().fold(sum, |acc, col| {
            if col.len() > 0 {
                return acc + 1;
            }
            acc
        })
    })
}

fn walk(grid: &Vec<Vec<char>>, mut beam: Beam, visited_grid: &mut Vec<Vec<Vec<Direction>>>) {
    if let Some(curr) = grid
        .get((beam.y) as usize)
        .and_then(|row| row.get((beam.x) as usize))
    {
        if visited_grid[beam.y as usize][beam.x as usize].contains(&beam.direction) {
            return;
        }
        visited_grid[beam.y as usize][beam.x as usize].push(beam.direction);
        mirror_beam(grid, visited_grid, curr, &mut beam);
        walk(grid, beam, visited_grid);
    }
}

fn mirror_beam(
    grid: &Vec<Vec<char>>,
    visited_grid: &mut Vec<Vec<Vec<Direction>>>,
    mirror_type: &char,
    beam: &mut Beam,
) {
    match mirror_type {
        '.' => beam.move_forward(),
        '/' => match beam.direction {
            Direction::Right => beam.change_direction(Direction::Up),
            Direction::Left => beam.change_direction(Direction::Down),
            Direction::Up => beam.change_direction(Direction::Right),
            Direction::Down => beam.change_direction(Direction::Left),
        },
        '\\' => match beam.direction {
            Direction::Right => beam.change_direction(Direction::Down),
            Direction::Left => beam.change_direction(Direction::Up),
            Direction::Up => beam.change_direction(Direction::Left),
            Direction::Down => beam.change_direction(Direction::Right),
        },
        '|' => match beam.direction {
            Direction::Up => beam.move_forward(),
            Direction::Down => beam.move_forward(),
            Direction::Left => {
                beam.change_direction(Direction::Up);
                walk(
                    grid,
                    Beam {
                        x: beam.x,
                        y: beam.y + 1,
                        direction: Direction::Down,
                    },
                    visited_grid,
                )
            }
            Direction::Right => {
                beam.change_direction(Direction::Down);
                walk(
                    grid,
                    Beam {
                        x: beam.x,
                        y: beam.y - 1,
                        direction: Direction::Up,
                    },
                    visited_grid,
                )
            }
        },
        '-' => match beam.direction {
            Direction::Up => {
                beam.direction = Direction::Right;
                beam.x += 1;
                walk(
                    grid,
                    Beam {
                        x: beam.x - 1,
                        y: beam.y,
                        direction: Direction::Left,
                    },
                    visited_grid,
                )
            }
            Direction::Down => {
                beam.change_direction(Direction::Left);
                walk(
                    grid,
                    Beam {
                        x: beam.x + 1,
                        y: beam.y,
                        direction: Direction::Right,
                    },
                    visited_grid,
                )
            }
            Direction::Left => beam.move_forward(),
            Direction::Right => beam.move_forward(),
        },
        _ => unreachable!(),
    }
}
