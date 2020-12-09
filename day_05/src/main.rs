use std::collections::HashSet;
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
    println!("{:?}", p2);
    Ok(())
}

fn part_01(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'F' | 'L' => '0',
                    'B' | 'R' => '1',
                    _ => 'x',
                })
                .collect::<String>()
        })
        .map(|bytes| usize::from_str_radix(&bytes, 2).unwrap())
        .max()
        .unwrap()
}

fn part_02(input: &Vec<String>) -> usize {
    let ids = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'F' | 'L' => '0',
                    'B' | 'R' => '1',
                    _ => 'x',
                })
                .collect::<String>()
        })
        .map(|bytes| usize::from_str_radix(&bytes, 2).unwrap())
        .collect::<HashSet<usize>>();
    *ids.iter()
        .cloned()
        .filter_map(
            |id| match ids.contains(&(id + 2)) && !ids.contains(&(id + 1)) {
                true => Some(id + 1),
                false => None,
            },
        )
        .collect::<Vec<usize>>()
        .first()
        .unwrap()
}
