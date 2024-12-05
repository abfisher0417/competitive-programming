use std::fs;

// Found this which is more elegant than my solution: https://github.com/siddhu33/advent2024/blob/master/src/day4/mod.rs

fn part1(input: &Vec<Vec<char>>) -> usize {
    let direction_tuples = [
        [(-1, 0), (-2, 0), (-3, 0)],
        [(1, 0), (2, 0), (3, 0)],
        [(0, -1), (0, -2), (0, -3)],
        [(0, 1), (0, 2), (0, 3)],
        [(-1, -1), (-2, -2), (-3, -3)],
        [(1, 1), (2, 2), (3, 3)],
        [(-1, 1), (-2, 2), (-3, 3)],
        [(1, -1), (2, -2), (3, -3)]
    ];

    let mut num_xmas = 0;
    let grid_y_len = input.len() as isize;
    let grid_x_len = input[0].len() as isize;

    for i in 0..grid_y_len { // y-axis of input
        for j in 0..grid_x_len { // x-axis of input
            if input[i as usize][j as usize] != 'X' {
                continue;
            }
            for k in 0..direction_tuples.len() { // iterate through each direction tuple list
                for l in 0..direction_tuples[k].len() { // iterate through each direction tuple
                    let (x, y) = direction_tuples[k][l];
                    let new_i = i + y;
                    let new_j = j + x;
                    // FIXME: Simplify this logic
                    if new_i >= 0 && new_i < grid_y_len && new_j >= 0 && new_j < grid_x_len {
                        if l == 0 && input[new_i as usize][new_j as usize] != 'M' {
                            break;
                        }
                        if l == 1 && input[new_i as usize][new_j as usize] != 'A' {
                            break;
                        }
                        if l == 2 && input[new_i as usize][new_j as usize] == 'S' {
                            num_xmas += 1;
                        }
                    }
                }
            }
        }
    }

    num_xmas
}

fn part2(input: &Vec<Vec<char>>) -> usize {
    let grid_y_len: isize = input.len() as isize;
    let grid_x_len = input[0].len() as isize;
    let offsets = vec![(-1, 1), (1, -1), (-1, -1), (1, 1)];
    let mut matches = 0;
    for i in 0..grid_y_len { // y-axis of input
        for j in 0..grid_x_len { // x-axis of input
            if input[i as usize][j as usize] == 'A' {
                let mut other_chars: [i32; 4] = [-2, -2, -2, -2];
                for (idx, (x, y)) in offsets.iter().enumerate() {
                    let new_i = i + y;
                    let new_j = j + x;
                    if new_i >= 0 && new_i < grid_y_len && new_j >= 0 && new_j < grid_x_len {
                        let offset_char = input[new_i as usize][new_j as usize];
                        if offset_char == 'M' {
                            other_chars[idx] = 1;
                        } else if offset_char == 'S' {
                            other_chars[idx] = -1
                        }
                    }
                }
                if other_chars[0] + other_chars[1] == 0 && other_chars[2] + other_chars[3] == 0 {
                    matches += 1
                }
            }
        }
    }
    matches
}

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("input_data/day4.txt")
        .expect("Could not read file")
        .lines()
        .map(|line| {
            line.chars()
                .collect()
        })
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
