use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fetch_data()?;
    let sum1 = part1(&input);
    let sum2 = part2(&input);
    println!("Sum for Part 1: {}", sum1);
    println!("Sum for Part 2: {}", sum2);
    Ok(())
}

fn part2(input: &[String]) -> u32 {
    let mut sum = 0;
    let mut tickets: Vec<usize> = vec![1; input.len()];
    for (index, row) in input.iter().enumerate() {
        let splitted_row = split_row(row);
        let winning_numbers = extract_numbers(&splitted_row, 1);
        let our_numbers = extract_numbers(&splitted_row, 2);
        let mut matches: u32 = 1;
        for number in our_numbers.iter() {
            if winning_numbers.contains(number) {
                tickets[index + matches as usize] += tickets[index];
                matches += 1;
            }
        }
        sum += tickets[index] as u32;
    }
    sum
}

fn split_row(row: &str) -> Vec<&str> {
    let splitted_row: Vec<&str> = row.split(|c| c == ':' || c == '|').collect();
    splitted_row
}

fn extract_numbers(splitted_row: &[&str], index: usize) -> Vec<i32> {
    let numbers = splitted_row[index]
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>();
    numbers
}

fn part1(input: &[String]) -> u32 {
    let mut sum = 0;
    for row in input.iter() {
        let splitted_row = split_row(row);
        let winning_numbers = extract_numbers(&splitted_row, 1);
        let our_numbers = extract_numbers(&splitted_row, 2);
        let mut matches: u32 = 0;
        for number in our_numbers.iter() {
            if winning_numbers.contains(number) {
                matches += 1;
            }
        }
        if matches > 0 {
            sum += 2u32.pow(matches - 1);
        }
    }
    sum
}

fn fetch_data() -> Result<Vec<String>, Box<dyn Error>> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let input = io::BufReader::new(file).lines();
    input.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}
