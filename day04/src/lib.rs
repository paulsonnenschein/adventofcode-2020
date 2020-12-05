use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FieldType {
    Byr, // (Birth Year)
    Iyr, // (Issue Year)
    Eyr, // (Expiration Year)
    Hgt, // (Height)
    Hcl, // (Hair Color)
    Ecl, // (Eye Color)
    Pid, // (Passport ID)
    Cid, // (Country ID)
}

impl FromStr for FieldType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "byr" => Ok(Self::Byr),
            "iyr" => Ok(Self::Iyr),
            "eyr" => Ok(Self::Eyr),
            "hgt" => Ok(Self::Hgt),
            "hcl" => Ok(Self::Hcl),
            "ecl" => Ok(Self::Ecl),
            "pid" => Ok(Self::Pid),
            "cid" => Ok(Self::Cid),
            _ => Err(()),
        }
    }
}

impl FieldType {
    fn is_valid(&self, value: &str) -> bool {
        match self {
            Self::Byr => {
                let parsed = value.parse::<i32>().unwrap_or(0);
                (1920..=2002).contains(&parsed)
            }
            Self::Iyr => {
                let parsed = value.parse::<i32>().unwrap_or(0);
                (2010..=2020).contains(&parsed)
            }
            Self::Eyr => {
                let parsed = value.parse::<i32>().unwrap_or(0);
                (2020..=2030).contains(&parsed)
            }
            Self::Hgt => {
                let (height, unit) = value.split_at(value.len() - 2);
                match unit {
                    "cm" => {
                        let parsed = height.parse::<i32>().unwrap_or(0);
                        (150..=193).contains(&parsed)
                    }
                    "in" => {
                        let parsed = height.parse::<i32>().unwrap_or(0);
                        (59..=76).contains(&parsed)
                    }
                    _ => false,
                }
            }
            Self::Hcl => {
                value.len() == 7 && {
                    let mut chars = value.chars();
                    chars.next().unwrap() == '#' && chars.all(|c| c.is_ascii_hexdigit())
                }
            }
            Self::Ecl => matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
            Self::Pid => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
            Self::Cid => true,
        }
    }
}

pub fn parse(input: &str) -> Vec<HashMap<FieldType, String>> {
    input
        .split("\n\n")
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|item| {
                    let mut split = item.split(':');
                    (
                        split.next().unwrap().parse::<FieldType>().unwrap(),
                        split.next().unwrap().to_string(),
                    )
                })
                .collect::<HashMap<FieldType, String>>()
        })
        .collect()
}

pub fn part1(input: &[HashMap<FieldType, String>]) -> usize {
    input
        .iter()
        .map(|id| {
            if id.contains_key(&FieldType::Cid) {
                id.len() - 1
            } else {
                id.len()
            }
        })
        .filter(|&valid_keys| valid_keys == 7)
        .count()
}

pub fn part2(input: &[HashMap<FieldType, String>]) -> usize {
    input
        .iter()
        .filter(|&id| {
            let all_valid = id
                .iter()
                .all(|(field_type, content)| field_type.is_valid(content));

            if id.contains_key(&FieldType::Cid) {
                (id.len() - 1) == 7 && all_valid
            } else {
                id.len() == 7 && all_valid
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run04() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
