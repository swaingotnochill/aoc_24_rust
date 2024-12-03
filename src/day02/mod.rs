use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn solve() {
    println!("Day 02 Solution:");

    let mut safe_reports = 0;
    let mut tolerance_safe_reports = 0;

    if let Ok(lines) = read_lines("./src/input/day02.txt") {
        for line in lines.flatten() {
            let numbers: Vec<i32> = line
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            if is_safe(&numbers) {
                safe_reports += 1;
            }

            // Part 2.
            if is_safe_with_tolerance(&numbers) {
                tolerance_safe_reports += 1;
            }
        }
    }
    println!("No. of safe reports {safe_reports}");
    println!("No. of safe reports with tolerance {tolerance_safe_reports}");
}

pub fn is_safe_with_tolerance(nums: &Vec<i32>) -> bool {
    if nums.len() < 2 {
        return true;
    }

    if is_safe(&nums) {
        return true;
    }

    // Each number can be potentially be removed to make it safe.
    for i in 0..nums.len() {
        let mut new_nums = nums.clone();
        new_nums.remove(i);
        if is_safe(&new_nums) {
            return true;
        }
    }
    false
}

pub fn is_safe(nums: &Vec<i32>) -> bool {
    if nums.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..nums.len() {
        let diff = nums[i] - nums[i - 1];
        if diff > 0 {
            decreasing = false;
        }
        if diff < 0 {
            increasing = false;
        }

        if !increasing && !decreasing {
            return false;
        }

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }
    true
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
