use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let input_path = Path::new("input.txt");
    let reader = BufReader::new(File::open(&input_path)?);
    let input: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().parse())
        .flatten()
        .collect();
    let p1 = part_01(&input);
    println!("{}", p1);
    let p2 = part_02(&input, p1);
    println!("{:?}", p2);
    Ok(())
}

fn part_01(input: &Vec<usize>) -> usize {
    *input
        .as_slice()
        .windows(26)
        .find_map(|window| {
            let set = window[..25].into_iter().collect::<HashSet<&usize>>();
            for w in window[..25].into_iter() {
                match window.last().unwrap().checked_sub(*w) {
                    Some(num) => {
                        if set.contains(&(num)) && num != *w {
                            return None;
                        }
                    }
                    _ => (),
                }
            }
            Some(window.last().unwrap())
        })
        .unwrap()
}

fn part_02(input: &Vec<usize>, target: usize) -> usize {
    for i in 0..input.len() - 1 {
        for x in i + 1..(input.len() - 1) {
            let slice = &input[i..x];
            let sum: usize = slice.clone().iter().sum::<usize>();
            if sum == target {
                return slice.clone().iter().min().unwrap()
                    + slice.clone().iter().max().cloned().unwrap();
            }
        }
    }
    0
}
