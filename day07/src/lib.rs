#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
pub struct Rule {
    bag: Bag,
    must_contain: Vec<(u32, Bag)>,
}

pub type Bag = (String, String);

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^(\w+ \w+ bags) contain (no other bags|(?:\d+ \w+ \w+ bags?(?:, )?)+)\.$"
            )
            .unwrap();
        }

        let captures = RE.captures(s).unwrap();
        let bag = {
            let mut parts = captures[1].split_ascii_whitespace();
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().to_string(),
            )
        };
        let must_contain = {
            let str = &captures[2];
            if str == "no other bags" {
                vec![]
            } else {
                str.split(", ")
                    .map(|s| {
                        let mut parts = s.split_ascii_whitespace();
                        (
                            parts.next().unwrap().parse().unwrap(),
                            (
                                parts.next().unwrap().to_string(),
                                parts.next().unwrap().to_string(),
                            ),
                        )
                    })
                    .collect::<Vec<(u32, Bag)>>()
            }
        };
        Ok(Rule { bag, must_contain })
    }
}

pub fn parse(input: &str) -> Vec<Rule> {
    input
        .lines()
        .map(|line| Rule::from_str(line).unwrap())
        .collect()
}

pub fn part1(input: &[Rule]) -> usize {
    let my_bag: Bag = ("shiny".to_string(), "gold".to_string());
    let mut seen = HashSet::new();
    let mut todo = VecDeque::new();

    todo.push_back(&my_bag);

    while let Some(inspecton) = todo.pop_front() {
        if !seen.contains(inspecton) {
            seen.insert(inspecton);
            for Rule { bag, must_contain } in input {
                if must_contain.iter().any(|(_, d)| d == inspecton) {
                    todo.push_back(bag);
                }
            }
        }
    }

    seen.len() - 1
}

pub fn part2(input: &[Rule]) -> usize {
    let map = input
        .iter()
        .map(|Rule { bag, must_contain }| (bag, must_contain))
        .collect::<HashMap<_, _>>();

    number_of_bags(&("shiny".to_string(), "gold".to_string()), &map) - 1
}

fn number_of_bags(current: &Bag, rules: &HashMap<&Bag, &Vec<(u32, Bag)>>) -> usize {
    rules[current]
        .iter()
        .map(|(amount, bag)| *amount as usize * number_of_bags(bag, rules))
        .sum::<usize>()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run07() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
