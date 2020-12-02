pub fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

pub fn part1(input: &[i32]) -> i32 {
    for first in input {
        for second in input {
            if first + second == 2020 {
                return first * second;
            }
        }
    }
    unreachable!()
}

pub fn part2(input: &[i32]) -> i32 {
    for first in input {
        for second in input {
            for third in input {
                if first + second + third == 2020 {
                    return first * second * third;
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run01() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
