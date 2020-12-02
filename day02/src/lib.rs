use regex::Regex;

type Password = (usize, usize, char, String);

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

pub fn part1(input: &[Password]) -> usize {
    input.iter().filter(|&pw| is_valid(pw)).count()
}

fn is_valid(pw: &Password) -> bool {
    let count = pw.3.chars().filter(|&c| c == pw.2).count();

    ((pw.0)..=(pw.1)).contains(&count)
}

pub fn part2(input: &[Password]) -> usize {
    input.iter().filter(|&pw| is_valid2(pw)).count()
}

fn is_valid2(pw: &Password) -> bool {
    let f = pw.3.chars().nth(pw.0 - 1).map_or(false, |c| c == pw.2);
    let s = pw.3.chars().nth(pw.1 - 1).map_or(false, |c| c == pw.2);

    f && !s || !f && s
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
        println!("{:?}", part2(&parsed));
    }
}
