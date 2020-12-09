#![feature(iterator_fold_self)]

use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let input_path = Path::new("input.txt");
    let input = read_to_string(&input_path)?;
    let p1 = part_01(&input);
    println!("{}", p1);
    let p2 = part_02(&input);
    println!("{}", p2);
    Ok(())
}

fn part_01(input: &str) -> usize {
    let entries: Vec<&str> = input.split("\n\n").collect();
    entries
        .iter()
        .map(|entry| {
            entry
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

fn part_02(input: &str) -> usize {
    let entries: Vec<&str> = input.split("\n\n").collect();
    entries
        .iter()
        .map(|entry| {
            entry
                .split_whitespace()
                .map(|line| {
                    line.chars()
                        .filter(|c| !c.is_whitespace())
                        .collect::<HashSet<char>>()
                })
                .fold_first(|set, x| set.intersection(&x).cloned().collect())
                .unwrap()
                .len()
        })
        .sum()
}
