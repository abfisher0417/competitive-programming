use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

// A more Rust idiomatic solution can be found at

fn is_vec_increasing(vec: &Vec<i32>, skip_idx: Option<usize>) -> bool {
    let mut prev: Option<i32> = None;
    for i in 0..vec.len() {
        if let Some(skip) = skip_idx {
            if skip == i {
                continue;
            }
        }
        if let Some(p) = prev {
            if p >= vec[i] || vec[i] - p > 3 {
                return false;
            }
        }
        prev = Some(vec[i]);
    }
    true
}

fn is_vec_decreasing(vec: &Vec<i32>, skip_idx: Option<usize>) -> bool {
    let mut prev: Option<i32> = None;
    for i in 0..vec.len() {
        if let Some(skip) = skip_idx {
            if skip == i {
                continue;
            }
        }
        if let Some(p) = prev {
            if p <= vec[i] || p - vec[i] > 3 {
                return false;
            }
        }
        prev = Some(vec[i]);
    }
    true
}

fn are_levels_safe(levels: &Vec<i32>) -> bool {
    for i in 0..levels.len() {
        if is_vec_increasing(levels, Some(i)) || is_vec_decreasing(levels, Some(i)) {
            return true;
        }
    }
    is_vec_increasing(levels, None) || is_vec_decreasing(levels, None)
}

fn main() -> io::Result<()> {
    let f = File::open("input_data/day2.txt")?;
    let reader = BufReader::new(f);
    let mut safe_reports = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let levels_arr = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                if are_levels_safe(&levels_arr) {
                    safe_reports += 1;
                }
            }
            Err(e) => return Err(e),
        }
    }
    println!("Safe reports: {}", safe_reports);
    Ok(())
}
