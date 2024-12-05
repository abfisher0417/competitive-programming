use std::{cmp::Ordering, collections::HashMap, fs};

fn parse_ordering_rules(line: &str, ordering_rules: &mut HashMap<usize, Vec<usize>>) {
    if let Some((key_str, val_str)) = line.split_once("|") {
        let key = key_str.parse::<usize>().expect("Invalid key format");
        let val = val_str.parse::<usize>().expect("Invalid page format");
        ordering_rules.entry(key).or_insert_with(Vec::new).push(val);
    }
}

fn parse_updates(line: &str) -> Vec<usize> {
    line.split(',')
        .map(|x| x.parse::<usize>().expect("Invalid page format"))
        .collect()
}

fn mid_value(arr: &[usize]) -> usize {
    let len = arr.len();
    arr[len / 2]
}

fn is_update_valid(update: &Vec<usize>, ordering_rules: &HashMap<usize, Vec<usize>>) -> bool {
    let update_len = update.len();
    for (i, page) in update.iter().rev().enumerate() {
        // Are there any pages preceeding this page that should be printed after this page?
        if let Some(pages) = ordering_rules.get(page) {
            for j in (0..update_len - i).rev() {
                if pages.contains(&update[j]) {
                    return false;
                }
            }
        }
    }
    true
}

fn reorder_update(update: &Vec<usize>, ordering_rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut new_update = update.clone();
    new_update.sort_unstable_by(|a, b| {
        if let Some(pages) = ordering_rules.get(a) {
            if pages.contains(b) {
                return Ordering::Less; // a should come before b
            }
        }
        if let Some(pages) = ordering_rules.get(b) {
            if pages.contains(a) {
                return Ordering::Greater; // b should come before a
            }
        }
        a.cmp(b) // fallback to natural order if no specific rule
    });
    new_update
}

fn main() {
    let mut ordering_rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();
    let input = fs::read_to_string("input_data/day5.txt").expect("Could not read file");

    for line in input.lines() {
        if line.contains("|") {
            parse_ordering_rules(line, &mut ordering_rules);
        } else if !line.is_empty() {
            updates.push(parse_updates(line));
        }
    }

    let mut p1_result = 0;
    let mut p2_result = 0;
    for update in updates.iter() {
        if is_update_valid(update, &ordering_rules) {
            p1_result += mid_value(update);
        } else {
            let new_update = reorder_update(update, &ordering_rules);
            p2_result += mid_value(&new_update);
        }
    }

    println!("Part 1 result: {}", p1_result);
    println!("Part 2 result: {}", p2_result);
}
