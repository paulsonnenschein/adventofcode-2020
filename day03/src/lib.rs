pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u64 {
    calculate(input, 3, 1)
}

pub fn part2(input: &[&str]) -> u64 {
    calculate(input, 1, 1)
        * calculate(input, 3, 1)
        * calculate(input, 5, 1)
        * calculate(input, 7, 1)
        * calculate(input, 1, 2)
}

fn calculate(input: &[&str], right: usize, down: usize) -> u64 {
    input
        .iter()
        .enumerate()
        .filter(|(i, &_str)| i % down == 0)
        .enumerate()
        .map(|(i, (_, &str))| {
            if str.chars().nth((i * right) % str.len()).unwrap_or(' ') == '#' {
                1
            } else {
                0
            }
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run03() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
