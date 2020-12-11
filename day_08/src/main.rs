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

enum Cmd {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl From<&String> for Cmd {
    fn from(line: &String) -> Self {
        let split: Vec<&str> = line.split(' ').collect();
        let cmd = split[0];
        let num: isize = split[1].parse().unwrap();
        match cmd {
            "acc" => Cmd::Acc(num),
            "jmp" => Cmd::Jmp(num),
            _ => Cmd::Nop(num),
        }
    }
}

fn boot_sequence(cmds: &Vec<Cmd>) -> (bool, isize) {
    let mut index: usize = 0;
    let mut acc: isize = 0;
    let mut visited = std::collections::HashSet::new();
    visited.insert(index);
    while index < cmds.len() {
        let cmd = &cmds[index];
        match cmd {
            Cmd::Acc(num) => {
                acc += num;
                index += 1;
            }
            Cmd::Jmp(num) => {
                let i = if num.is_negative() {
                    index - num.abs() as usize
                } else {
                    index + num.abs() as usize
                };

                if !visited.contains(&i) {
                    index = i;
                } else {
                    return (false, acc);
                }
            }
            _ => {
                index += 1;
            }
        }
        visited.insert(index);
    }
    (true, acc)
}

fn part_01(lines: &Vec<String>) -> isize {
    let cmds: Vec<Cmd> = lines.iter().map(|line| Cmd::from(line)).collect();
    boot_sequence(&cmds).1
}

fn part_02(lines: &Vec<String>) -> isize {
    let mut cmds: Vec<Cmd> = lines.iter().map(|line| Cmd::from(line)).collect();
    let mut index: usize = 0;
    let mut acc: isize = 0;
    let mut visited = std::collections::HashSet::new();
    let mut jmp_nop = Vec::new();
    visited.insert(index);
    while index < cmds.len() {
        let cmd = &cmds[index];
        match cmd {
            Cmd::Acc(num) => {
                acc += num;
                index += 1;
            }
            Cmd::Jmp(num) => {
                let i = if num.is_negative() {
                    index - num.abs() as usize
                } else {
                    index + num.abs() as usize
                };
                jmp_nop.push(index);
                if !visited.contains(&i) {
                    index = i;
                } else {
                    break;
                }
            }
            _ => {
                index += 1;
                jmp_nop.push(index);
            }
        }
        visited.insert(index);
    }

    while !jmp_nop.is_empty() {
        let id = jmp_nop.pop().unwrap();

        match cmds[id] {
            Cmd::Nop(num) => cmds[id] = Cmd::Jmp(num),
            Cmd::Jmp(num) => cmds[id] = Cmd::Nop(num),
            _ => (),
        }

        let (finished, acc) = boot_sequence(&cmds);
        if finished {
            return acc;
        }

        match cmds[id] {
            Cmd::Nop(num) => cmds[id] = Cmd::Jmp(num),
            Cmd::Jmp(num) => cmds[id] = Cmd::Nop(num),
            _ => (),
        }
    }
    0
}
