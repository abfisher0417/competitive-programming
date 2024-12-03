use std::collections::HashMap;
use std::fs;

fn read_input() -> (Vec<i32>, Vec<i32>) {
    // FIXME: Handle hardcoding of input.txt
    let input = fs::read_to_string("input_data/day1.txt").unwrap();
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        if let (Some(num1), Some(num2)) = (numbers.next(), numbers.next()) {
            vec1.push(num1.parse().unwrap());
            vec2.push(num2.parse().unwrap());
        }
    }

    (vec1, vec2)
}

fn list_frequency(vec: &Vec<i32>) -> HashMap<i32, i32> {
    let mut frequency_map = HashMap::new();
    for &num in vec.iter() {
        *frequency_map.entry(num).or_insert(0) += 1;
    }
    frequency_map
}

fn main() {
    let (mut vec1, mut vec2) = read_input();

    vec1.sort();
    vec2.sort();

    let frequency_map2 = list_frequency(&vec2);

    let mut total_distances: i32 = 0;
    let mut similarity_score: i32 = 0;

    for (i, num1) in vec1.iter().enumerate() {
        let num2 = vec2[i];
        total_distances += (num1 - num2).abs();
        similarity_score += num1 * frequency_map2.get(&num1).unwrap_or(&0);
    }

    println!("Total distances: {}", total_distances);
    println!("Similarity score: {}", similarity_score);
}
