use crate::Instruction::{SetMask, SetMemory};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Instruction {
    SetMask(String),
    SetMemory(usize, u64),
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" = ");
            let left = split.next().unwrap();
            let right = split.next().unwrap();
            match left {
                "mask" => SetMask(right.to_owned()),
                _ => {
                    let addr = left
                        .strip_prefix("mem[")
                        .and_then(|l| l.strip_suffix("]"))
                        .and_then(|l| l.parse::<usize>().ok());
                    SetMemory(addr.unwrap(), right.parse().unwrap())
                }
            }
        })
        .collect()
}

pub fn part1(input: &[Instruction]) -> u64 {
    let max_assign = *input
        .iter()
        .flat_map(|i| match i {
            SetMemory(addr, _) => Some(addr),
            _ => None,
        })
        .max()
        .unwrap();

    let mut current_mask = String::new();
    let mut memory = vec![0u64; max_assign + 1];

    for instruction in input.iter().cloned() {
        match instruction {
            SetMask(mask) => current_mask = mask,
            SetMemory(addr, val) => memory[addr] = apply_mask(val, &current_mask),
        }
    }

    memory.iter().sum()
}

pub fn apply_mask(val: u64, mask: &str) -> u64 {
    let mut copy = 0;
    for (i, c) in mask.chars().enumerate() {
        match c {
            'X' => {
                copy <<= 1;
                let is_set = (val & (1u64 << (mask.len() - i - 1))) > 0;
                if is_set {
                    copy |= 1; // set lowest bit
                }
            }
            '1' => {
                copy <<= 1;
                copy |= 1; // set lowest bit
            }
            '0' => {
                copy <<= 1;
            }
            _ => unreachable!(),
        };
    }
    copy
}

pub fn part2(input: &[Instruction]) -> u64 {
    let mut current_mask = String::new();
    let mut memory = HashMap::<usize, u64>::new();

    for instruction in input.iter().cloned() {
        match instruction {
            SetMask(mask) => current_mask = mask,
            SetMemory(addr, val) => {
                for masked in mask_addresses(addr, &current_mask) {
                    memory.insert(masked, val);
                }
            }
        }
    }

    memory.values().sum()
}

fn mask_addresses(val: usize, mask: &str) -> impl IntoIterator<Item = usize> {
    let mut addrs = vec![0usize];

    for (i, c) in mask.chars().enumerate() {
        let mut to_add = Vec::new();
        for addr in addrs.iter_mut() {
            match c {
                'X' => {
                    *addr <<= 1;
                    to_add.push(*addr); // add version without lowest bit set to queue

                    *addr |= 1; // set lowest bit
                }
                '1' => {
                    *addr <<= 1;
                    *addr |= 1; // set lowest bit
                }
                '0' => {
                    *addr <<= 1;
                    let is_set = (val & (1usize << (mask.len() - i - 1))) > 0;
                    if is_set {
                        *addr |= 1; // set lowest bit
                    }
                }
                _ => unreachable!(),
            };
        }

        addrs.extend(to_add.into_iter());
    }
    addrs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        assert_eq!(73, apply_mask(11, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
        assert_eq!(101, apply_mask(101, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
        assert_eq!(64, apply_mask(0, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
    }

    #[test]
    fn run14() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
