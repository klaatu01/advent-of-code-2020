use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let input_path = Path::new("input.txt");
    let reader = BufReader::new(File::open(&input_path)?);
    let mut input: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().parse())
        .flatten()
        .collect();
    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);
    let p1 = part_01(&mut input);
    println!("{}", p1);
    let p2 = part_02(&input);
    println!("{:?}", p2);
    Ok(())
}

fn recurse(head: usize, tail: &[usize], ones: usize, threes: usize) -> usize {
    match tail.first() {
        Some(num) => match num - head {
            1 => recurse(*num, &tail[1..], ones + 1, threes),
            3 => recurse(*num, &tail[1..], ones, threes + 1),
            _ => recurse(*num, &tail[1..], ones, threes),
        },
        None => ones * threes,
    }
}

fn part_01(input: &Vec<usize>) -> usize {
    recurse(0, &input[..], 0, 0)
}

fn part_02(input: &Vec<usize>) -> usize {
    let mut map = HashMap::<usize, usize>::new();
    let len = input.len();
    map.insert(input[len - 2], 1);
    map.insert(input[len - 3], 1);
    for i in (0..len - 3).rev() {
        if input[i + 3] - input[i] <= 3 {
            map.insert(
                input[i],
                *map.get(&input[i + 1]).unwrap()
                    + *map.get(&input[i + 2]).unwrap()
                    + *map.get(&input[i + 3]).unwrap(),
            );
        } else if input[i + 2] - input[i] <= 3 {
            map.insert(
                input[i],
                *map.get(&input[i + 1]).unwrap() + *map.get(&input[i + 2]).unwrap(),
            );
        } else {
            map.insert(input[i], *map.get(&input[i + 1]).unwrap());
        }
    }
    *map.get(&0).unwrap()
}
