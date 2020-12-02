use regex::Regex;

type Password = (u32, u32, char, String);

pub fn parse(input: &str) -> Vec<Password> {
    let pattern = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            (
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].chars().next().unwrap(),
                captures[4].to_string(),
            )
        })
        .collect()
}

pub fn part1(input: &[Password]) -> u32 {
    input.iter().filter(|&pw| is_valid(pw)).count() as u32
}

fn is_valid(pw: &Password) -> bool {
    let count = pw.3.chars().filter(|&c| c == pw.2).count();

    ((pw.0)..=(pw.1)).contains(&(count as u32))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run02() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        //println!("{:?}", part2(&parsed));
    }
}
