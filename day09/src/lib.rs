pub fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part1(input: &[u64]) -> u64 {
    solve(input, 25)
}

pub fn solve(input: &[u64], preamble_size: usize) -> u64 {
    input
        .windows(preamble_size + 1)
        .find(|window| !is_valid_window(&window[0..preamble_size], window[preamble_size]))
        .unwrap()[preamble_size]
}

fn is_valid_window(input: &[u64], target: u64) -> bool {
    for (i, v1) in input.iter().enumerate() {
        for v2 in input[i..].iter() {
            if v1 + v2 == target {
                return true;
            }
        }
    }
    false
}

pub fn part2(input: &[u64], part1_sol: u64) -> u64 {
    for i in 0..input.len() {
        let mut sum = 0u64;
        let mut min = u64::max_value();
        let mut max = u64::min_value();
        for (ii, value) in input[i..].iter().enumerate() {
            sum += value;
            min = min.min(*value);
            max = max.max(*value);
            if sum == part1_sol && ii >= 1 {
                return min + max;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = include_str!("./sample.txt");
        let parsed = parse(input);
        assert_eq!(127, solve(&parsed, 5));
        assert_eq!(62, part2(&parsed, 127));
    }

    #[test]
    fn run09() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        let solution = part1(&parsed);
        println!("{:?}", solution);
        println!("{:?}", part2(&parsed, solution));
    }
}
