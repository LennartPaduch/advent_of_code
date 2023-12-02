use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

#[tokio::main]
async fn main() {
    let data = fetch_data().await.unwrap();
    let sum1 = part1(&data);
    let sum2 = part2(&data);
    println!("Sum for Part 1: {}", sum1);
    println!("Sum for Part 2: {}", sum2);
}

fn update_counts_and_highscores(counts: &mut [i32; 3], index: usize, value: i32, highscores: &mut [i32; 3]){
    counts[index] += value;
    if counts[index] > highscores[index]{
        highscores[index] = counts[index];
    }
}

fn part2(lines: &[String]) -> i32{
    let mut cube_power = 0;
    for line in lines.iter(){
        let numbers = extract_numbers(line);
        let colors = extract_colors(line);
        let mut counts = [0,0,0]; // rgb
        let mut highscores = [0,0,0]; // rgb
        let mut index = 1;
        for color in colors{
            if color == ";"{
                counts = [0,0,0];
                continue;
            }
            let value = numbers[index];
            index += 1;
            match color.as_str(){
                "red" => {
                    update_counts_and_highscores(&mut counts, 0, value, &mut highscores)
                },
                "green" => {
                    update_counts_and_highscores(&mut counts, 1, value, &mut highscores)
                },
                "blue" => {
                    update_counts_and_highscores(&mut counts, 2, value, &mut highscores)
                },
                _ => {}
            }
        }
        cube_power += highscores[0] * highscores[1] * highscores[2];
    }
    cube_power
}

fn part1(lines: &[String]) -> i32 {
    let limits = [12, 13, 14]; // rgb
    let mut sum = 0;
    'lines: for line in lines.iter() { 
        let numbers = extract_numbers(line);
        let colors = extract_colors(line);
        let mut counts = [0, 0, 0]; // rgb
        let mut index = 1;
        for color in colors {
            if color == ";" {
                counts = [0, 0, 0];
                continue;
            }
            let value = numbers[index];
            index += 1;
            match color.as_str() {
                "red" => {
                    counts[0] += value;
                    if counts[0] > limits[0] {
                        continue 'lines;  
                    }
                },
                "green" => {
                    counts[1] += value;
                    if counts[1] > limits[1] {
                        continue 'lines; 
                    }
                },
                "blue" => {
                    counts[2] += value;
                    if counts[2] > limits[2] {
                        continue 'lines; 
                    }
                },
                _ => {}
            }
        }
        sum += numbers[0];
    }
    sum
}


fn extract_colors(line: &str) -> Vec<String> {
    let color_re = Regex::new(r"\b(red|blue|green)\b|;").unwrap();
    let colors: Vec<String> = color_re
        .find_iter(line)
        .map(|mat| mat.as_str().to_string())
        .collect();
    colors
}

fn extract_numbers(line: &str) -> Vec<i32> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.find_iter(line)
        .filter_map(|mat| mat.as_str().parse::<i32>().ok())
        .collect()
}
async fn fetch_data() -> Result<Vec<String>, Box<dyn Error>> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let lines = io::BufReader::new(file).lines();
    let result: Result<Vec<String>, io::Error> = lines.collect();
    result.map_err(|e| e.into())
}
