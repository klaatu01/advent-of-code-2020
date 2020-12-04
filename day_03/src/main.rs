use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let input_path = Path::new("input.txt");
    let reader = BufReader::new(File::open(&input_path)?);
    let input = reader.lines().map(|s| s.unwrap()).collect();
    let p1 = part_01(&input);
    println!("{}", p1);
    let p2 = part_02(&input);
    println!("{}", p2);
    Ok(())
}

fn tree_count(map: &Vec<bool>, height: usize, width: usize, down: usize, right: usize) -> usize {
    let mut count = 0;
    let mut y = 0;
    loop {
        let x = (y / down) * right;
        let index = (width * y) + (x % width);
        if map[index] {
            count += 1;
        }
        y += down;
        if y >= height {
            break;
        }
    }
    count
}

fn part_01(input: &Vec<String>) -> usize {
    let width = input[0].len();
    let height = input.len();

    let flat: Vec<bool> = input
        .iter()
        .flat_map(|s| s.chars())
        .filter_map(|c| match c {
            '#' => Some(true),
            '.' => Some(false),
            _ => None,
        })
        .collect();
    tree_count(&flat, height, width, 1, 3)
}

fn part_02(input: &Vec<String>) -> usize {
    let width = input[0].len();
    let height = input.len();

    let flat: Vec<bool> = input
        .iter()
        .flat_map(|s| s.chars())
        .filter_map(|c| match c {
            '#' => Some(true),
            '.' => Some(false),
            _ => None,
        })
        .collect();
    tree_count(&flat, height, width, 1, 1)
        * tree_count(&flat, height, width, 1, 3)
        * tree_count(&flat, height, width, 1, 5)
        * tree_count(&flat, height, width, 1, 7)
        * tree_count(&flat, height, width, 2, 1)
}

#[cfg(test)]
mod tests {}
