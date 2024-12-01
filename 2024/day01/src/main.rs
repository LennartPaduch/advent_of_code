fn main() {
    let data = std::fs::read_to_string("input.txt").expect("Failed to read file");
    
    let lines: Vec<&str> = data.lines().collect();
    
    let mut list1: Vec<i32> = lines.iter().map(|line| {
        line.split_whitespace()
            .next()
            .unwrap()
            .parse()
            .expect("Failed to parse first number")
    }).collect();
    
    let mut list2: Vec<i32> = lines.iter().map(|line| {
        line.split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .expect("Failed to parse second number")
    }).collect();

    list1.sort();
    list2.sort();

    let sum = list1.iter().zip(list2.iter()).fold(0, |acc, (x1, x2)| acc + (x1 - x2).abs());
    println!("Sum: {}", sum);

    // part 2
    let mut count_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    
    for &num in &list2 {
        *count_map.entry(num).or_insert(0) += 1;
    }
    
    let similarity_score = list1.iter().fold(0, |acc, x| {
        if let Some(count) = count_map.get(x){
           return acc + x*count
        }
        acc
    });

    println!("Similarity score: {}", similarity_score);
}
