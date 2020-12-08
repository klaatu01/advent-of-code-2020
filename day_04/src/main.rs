use std::collections::HashMap;
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
            Passport::from(
                entry
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|field| {
                        let s = field.split(":").collect::<Vec<&str>>();
                        (s[0].to_string(), s[1].to_string())
                    })
                    .collect::<HashMap<String, String>>(),
            )
        })
        .filter(|passport| passport.validate_p01())
        .count()
}

fn part_02(input: &str) -> usize {
    let entries: Vec<&str> = input.split("\n\n").collect();
    entries
        .iter()
        .map(|entry| {
            Passport::from(
                entry
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|field| {
                        let s = field.split(":").collect::<Vec<&str>>();
                        (s[0].to_string(), s[1].to_string())
                    })
                    .collect::<HashMap<String, String>>(),
            )
        })
        .filter(|passport| passport.validate_p02())
        .count()
}

#[derive(Debug)]
enum Height {
    CM(usize),
    IN(usize),
    Other(usize),
}

#[derive(Debug)]
enum Color {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
    Unkown,
}

#[derive(Debug)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<Color>,
    pid: Option<String>,
    cid: Option<usize>,
}

impl Passport {
    pub fn validate_p01(&self) -> bool {
        match self {
            Passport {
                byr: Some(_),
                iyr: Some(_),
                eyr: Some(_),
                hgt: Some(_),
                hcl: Some(_),
                ecl: Some(_),
                pid: Some(_),
                cid: _,
            } => true,
            _ => false,
        }
    }

    pub fn validate_p02(&self) -> bool {
        match self {
            Passport {
                byr: Some(byr_val),
                iyr: Some(iyr_val),
                eyr: Some(eyr_val),
                hgt: Some(hgt_val),
                hcl: Some(hcl_val),
                ecl: Some(ecl_val),
                pid: Some(pid_val),
                cid: _,
            } => {
                let years = match (byr_val, iyr_val, eyr_val) {
                    (1920..=2002, 2010..=2020, 2020..=2030) => true,
                    _ => false,
                };
                let height = match hgt_val {
                    Height::CM(n) => match n {
                        150..=193 => true,
                        _ => false,
                    },
                    Height::IN(n) => match n {
                        59..=76 => true,
                        _ => false,
                    },
                    _ => false,
                };
                let eyes = match ecl_val {
                    Color::Unkown => false,
                    _ => true,
                };

                let hair = match &hcl_val[..1] {
                    "#" => {
                        let s = &hcl_val[1..hcl_val.len()];
                        s.len() == 6
                            && s.chars().all(|c| match c {
                                'a'..='f' => true,
                                '0'..='9' => true,
                                _ => false,
                            })
                    }
                    _ => false,
                };

                let passport_id = pid_val.len() == 9 && pid_val.chars().all(|c| c.is_numeric());

                years && height && eyes && hair && passport_id
            }
            _ => false,
        }
    }
}

impl From<HashMap<String, String>> for Passport {
    fn from(map: HashMap<String, String>) -> Passport {
        Passport {
            byr: map.get("byr").map(|f| f.parse().unwrap()),
            iyr: map.get("iyr").map(|f| f.parse().unwrap()),
            eyr: map.get("eyr").map(|f| f.parse().unwrap()),
            hgt: match map.get("hgt") {
                Some(f) => {
                    let num: Option<usize> = f
                        .chars()
                        .filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse()
                        .ok();
                    let followed = f.chars().filter(|c| c.is_alphabetic()).collect::<String>();
                    match (num, followed.as_str()) {
                        (Some(n), "cm") => Some(Height::CM(n)),
                        (Some(n), "in") => Some(Height::IN(n)),
                        (Some(n), _) => Some(Height::Other(n)),
                        (_, _) => None,
                    }
                }
                _ => None,
            },
            hcl: map.get("hcl").cloned(),
            ecl: match map.get("ecl").map(|f| f.as_str()) {
                Some("amb") => Some(Color::Amber),
                Some("brn") => Some(Color::Brown),
                Some("blu") => Some(Color::Blue),
                Some("gry") => Some(Color::Grey),
                Some("grn") => Some(Color::Green),
                Some("hzl") => Some(Color::Hazel),
                Some("oth") => Some(Color::Other),
                Some(_) => Some(Color::Unkown),
                None => None,
            },
            pid: map.get("pid").cloned(),
            cid: map.get("cid").map(|f| f.parse().unwrap()),
        }
    }
}
