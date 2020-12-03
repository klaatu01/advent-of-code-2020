use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let input_path = Path::new("input.txt");
    let reader = BufReader::new(File::open(&input_path)?);
    let input = reader
        .lines()
        .map(|line| line.unwrap().parse())
        .flatten()
        .collect();
    let p1 = part_01(&input);
    let p2 = part_02(&input);
    println!("{}", p1.unwrap());
    println!("{}", p2.unwrap());
    Ok(())
}

fn part_01(input: &Vec<i32>) -> Option<i32> {
    let numbers = HashSet::<i32>::from_iter(input.iter().cloned());
    numbers
        .iter()
        .find_map(|a| numbers.get(&(2020 - a)).map(|b| a * b))
}

fn part_02(input: &Vec<i32>) -> Option<i32> {
    let numbers = HashSet::<i32>::from_iter(input.iter().cloned());
    numbers.iter().find_map(|a| {
        numbers
            .iter()
            .filter(|&b| a != b)
            .find_map(|b| numbers.get(&(2020 - a - b)).map(|c| a * b * c))
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = vec![151, 1020, 1000, 500, 1500, 20];
        let val = super::part_01(&input);
        assert_eq!(val.unwrap(), 1000 * 1020);
    }

    #[test]
    fn part_02() {
        let input = vec![151, 1020, 1000, 300, 440, 580];
        let val = super::part_02(&input);
        assert_eq!(val.unwrap(), 1000 * 440 * 580);
    }
}
