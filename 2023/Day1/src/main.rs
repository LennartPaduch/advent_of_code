use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let lines = fetch_data().await?;
    let sum1 = part1(&lines);
    let sum2 = part2(&lines);
    println!("Sum for Part 1: {}", sum1);
    println!("Sum for Part 2: {}", sum2);
    Ok(())
}

fn extract_digits(line: &str) -> Vec<i32> {
    line.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect()
}

fn part1(lines: &[String]) -> i32 {
    lines
        .iter()
        .map(|line| {
            let digits = extract_digits(line);
            digits.get(0).unwrap_or(&0) * 10 + digits.last().unwrap_or(&0)
        })
        .sum()
}

fn part2(lines: &[String]) -> i32 {
    let spelled_numbers = build_spelled_numbers_map();

    let mut sum: i32 = 0;
    for line in lines.iter() {
        let mut digits: Vec<i8> = Vec::new();
        let mut i = 0;
        while i < line.len() {
            for (word, &digit) in spelled_numbers.iter() {
                if line[i..].starts_with(word) {
                    digits.push(digit);
                    break;
                }
            }
            i += 1;
        }
        let number = digits[0] * 10 + digits[digits.len() - 1];
        sum += number as i32;
    }
    sum
}

fn build_spelled_numbers_map() -> HashMap<&'static str, i8> {
    let spelled_numbers: HashMap<&str, i8> = [
        ("nine", 9),
        ("eight", 8),
        ("seven", 7),
        ("six", 6),
        ("five", 5),
        ("four", 4),
        ("three", 3),
        ("two", 2),
        ("one", 1),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]
    .iter()
    .cloned()
    .collect();
    spelled_numbers
}

async fn fetch_data() -> Result<Vec<String>, Box<dyn Error>> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let lines = io::BufReader::new(file).lines();
    let result: Result<Vec<String>, io::Error> = lines.collect();
    result.map_err(|e| e.into())
}
