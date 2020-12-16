use regex::Regex;
use std::ops::RangeInclusive;

pub type Rule = (String, RangeInclusive<u32>, RangeInclusive<u32>);
#[derive(Debug)]
pub struct Ticket(Vec<u32>);

impl From<&str> for Ticket {
    fn from(s: &str) -> Self {
        Ticket(s.split(',').map(|str| str.parse().unwrap()).collect())
    }
}

#[derive(Debug)]
pub struct Input {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

pub fn parse(input: &str) -> Input {
    let rule_pattern = Regex::new(r"(?m)([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let mut parts = input.split("\n\n");
    let rules = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let captures = rule_pattern.captures(line).unwrap();

            (
                captures[1].to_owned(),
                captures[2].parse().unwrap()..=captures[3].parse().unwrap(),
                captures[4].parse().unwrap()..=captures[5].parse().unwrap(),
            )
        })
        .collect();
    let my_ticket = parts
        .next()
        .unwrap()
        .strip_prefix("your ticket:\n")
        .unwrap()
        .into();
    let nearby_tickets = parts
        .next()
        .unwrap()
        .strip_prefix("nearby tickets:\n")
        .unwrap()
        .lines()
        .map(Ticket::from)
        .collect();

    Input {
        rules,
        my_ticket,
        nearby_tickets,
    }
}

pub fn part1(input: &Input) -> u32 {
    input
        .nearby_tickets
        .iter()
        .map(|ticket| {
            ticket
                .0
                .iter()
                .filter(|&&n| !matches_a_rule(n, &input.rules))
                .sum::<u32>()
        })
        .sum()
}

#[allow(clippy::needless_range_loop)]
pub fn part2(input: &Input) -> u64 {
    let filtered = input
        .nearby_tickets
        .iter()
        .filter(|&ticket| ticket.0.iter().all(|&n| matches_a_rule(n, &input.rules)))
        .collect::<Vec<_>>();
    let mut current_rules = input.rules.iter().cloned().enumerate().collect::<Vec<_>>();
    let mut assigned_rules = vec![usize::max_value(); input.rules.len()];

    while !current_rules.is_empty() {
        'outer: for i in 0..input.my_ticket.0.len() {
            let mut found_rule = None;
            for (i_current_rules, (_, (_, first, second))) in current_rules.iter().enumerate() {
                if filtered
                    .iter()
                    .all(|&ticket| first.contains(&ticket.0[i]) || second.contains(&ticket.0[i]))
                {
                    if found_rule == None {
                        found_rule = Some(i_current_rules);
                    } else {
                        continue 'outer; // skip this field, it is not uniquely assignable
                    }
                }
            }
            if let Some(i_rule) = found_rule {
                assigned_rules[i] = current_rules[i_rule].0;
                current_rules.swap_remove(i_rule);
            }
        }
    }

    assigned_rules
        .iter()
        .enumerate()
        .filter(|(_, &rule_i)| input.rules[rule_i].0.starts_with("departure"))
        .map(|(ticket_i, _)| input.my_ticket.0[ticket_i] as u64)
        .product()
}

fn matches_a_rule(i: u32, rules: &[Rule]) -> bool {
    rules
        .iter()
        .any(|rule| rule.1.contains(&i) || rule.2.contains(&i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        let parsed = parse(input);

        assert_eq!(71, part1(&parsed));
    }

    #[test]
    fn run16() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
