extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let input_path = Path::new("input.txt");
    let reader = BufReader::new(File::open(&input_path)?);
    let input = reader.lines().flatten().collect();
    let p1 = part_01(&input);
    println!("{}", p1);
    let p2 = part_02(&input);
    println!("{}", p2);
    Ok(())
}

fn part_01(input: &Vec<String>) -> usize {
    let splitter = regex::Regex::new(r"([\d]+)\-([\d]+)\s([A-z]):\s([A-z]+)").unwrap();
    let policies: Vec<(usize, usize, char, String)> = input
        .iter()
        .map(|line| {
            for cap in splitter.captures_iter(line) {
                return Some((
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                    cap[3].to_string().chars().next().unwrap(),
                    cap[4].to_string(),
                ));
            }
            None
        })
        .flatten()
        .collect();
    policies
        .iter()
        .filter(|policy| {
            let count = policy.3.chars().filter(|&c| c == policy.2).count();
            count >= policy.0 && count <= policy.1
        })
        .count()
}

fn part_02(input: &Vec<String>) -> usize {
    let splitter = regex::Regex::new(r"([\d]+)\-([\d]+)\s([A-z]):\s([A-z]+)").unwrap();
    let policies: Vec<(usize, usize, char, String)> = input
        .iter()
        .map(|line| {
            for cap in splitter.captures_iter(line) {
                return Some((
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                    cap[3].to_string().chars().next().unwrap(),
                    cap[4].to_string(),
                ));
            }
            None
        })
        .flatten()
        .collect();
    policies
        .iter()
        .filter(|policy| {
            let chars: Vec<char> = policy.3.chars().collect();
            (chars[policy.0 - 1] == policy.2 && chars[policy.1 - 1] != policy.2)
                || (chars[policy.0 - 1] != policy.2 && chars[policy.1 - 1] == policy.2)
        })
        .count()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = vec!["1-3 a: aab".to_string()];
        let count = super::part_01(&input);
        assert_eq!(count, 1);
    }

    #[test]
    fn part_02() {
        let input = vec!["1-3 a: aab".to_string()];
        let count = super::part_02(&input);
        assert_eq!(count, 1);
    }
}
