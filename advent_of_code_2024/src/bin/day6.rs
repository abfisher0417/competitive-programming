use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

// Up, Right, Down, Left
const DIRECTION_ARRAY: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn load_grid() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let file = File::open("input_data/day6.txt")?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let char_vec: Vec<char> = line.chars().collect();
        grid.push(char_vec);
    }
    Ok(grid)
}

fn starting_position(grid: &Vec<Vec<char>>) -> (isize, isize) {
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x] == '^' {
                return (x as isize, y as isize);
            }
        }
    }
    return (0, 0);
}

fn will_loop(grid: &Vec<Vec<char>>) -> bool {
    let (mut x, mut y) = starting_position(&grid);
    let grid_y_len = grid.len() as isize;
    let grid_x_len = grid[0].len() as isize;
    let mut visited_with_direction: HashSet<(isize, isize, usize)> = HashSet::new();
    let mut direction: usize = 0 as usize;
    loop {
        let cur_char: char = grid[y as usize][x as usize];
        if cur_char != '#' {
            visited_with_direction.insert((x, y, direction));
        }
        let next_x = x + DIRECTION_ARRAY[direction].0;
        let next_y = y + DIRECTION_ARRAY[direction].1;
        if visited_with_direction.contains(&(next_x, next_y, direction)) {
            return true;
        } else if next_x < 0 || next_x >= grid_x_len || next_y < 0 || next_y >= grid_y_len {
            break;
        } else if grid[next_y as usize][next_x as usize] == '#' {
            direction = (direction + 1) % 4;
        } else {
            x = next_x;
            y = next_y;
        }
    }
    return false;
}

fn main() {
    let mut grid = load_grid().unwrap();
    let (mut x, mut y) = starting_position(&grid);
    let grid_y_len = grid.len() as isize;
    let grid_x_len = grid[0].len() as isize;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut direction: usize = 0 as usize;
    loop {
        let cur_char: char = grid[y as usize][x as usize];
        if cur_char != '#' {
            visited.insert((x, y));
        }
        let next_x = x + DIRECTION_ARRAY[direction].0;
        let next_y = y + DIRECTION_ARRAY[direction].1;
        if next_x < 0 || next_x >= grid_x_len || next_y < 0 || next_y >= grid_y_len {
            break;
        } else if grid[next_y as usize][next_x as usize] == '#' {
            direction = (direction + 1) % 4;
        } else {
            x = next_x;
            y = next_y;
        }
    }
    println!("Part 1: {}", visited.len());

    let mut part2 = 0;
    for point in visited.iter() {
        let prev_grid_val = grid[point.1 as usize][point.0 as usize];
        if prev_grid_val == '.' {
            grid[point.1 as usize][point.0 as usize] = '#';
            if will_loop(&grid) {
                part2 += 1;
            }
            grid[point.1 as usize][point.0 as usize] = prev_grid_val;
        }
    }
    println!("Part 2: {}", part2);
}
