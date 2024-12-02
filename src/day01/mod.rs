use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn solve() {
    println!("Day 01 Solution:");

    let mut _list_one: Vec<u32> = Vec::new();
    let mut _list_two: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("./src/input/day01.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let numbers: Vec<u32> = line
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            assert_eq!(numbers.len(), 2);
            _list_one.push(numbers[0]);
            _list_two.push(numbers[1]);
        }
    }

    _list_one.sort();
    _list_two.sort();

    assert_eq!(_list_one.len(), _list_two.len());

    let mut sum = 0;

    for i in 0.._list_one.len() {
        sum += _list_one[i].abs_diff(_list_two[i]);
    }

    println!("Part 1: Solution is {}", sum);

    // Part 2.
    let mut similarity_sum = 0;
    for i in _list_one {
        similarity_sum += i * _list_two.iter().filter(|&s| *s == i).count() as u32;
    }

    println!("Part 2: Solution is {}", similarity_sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
