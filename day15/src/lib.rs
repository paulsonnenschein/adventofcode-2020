pub fn parse(input: &str) -> Vec<usize> {
    input.split(',').flat_map(|n| n.parse().ok()).collect()
}

pub fn part1(initial: &[usize]) -> usize {
    calculate(initial, 2020)
}
pub fn part2(initial: &[usize]) -> usize {
    calculate(initial, 30000000)
}

fn calculate(initial: &[usize], target: usize) -> usize {
    let mut iteration = 1usize;

    let expected_range = 100_000_000;
    let mut seen = vec![false; expected_range];
    let mut last_seen = vec![0usize; expected_range];
    let mut number_val = vec![0usize; expected_range];

    for x in initial {
        number_val[*x] = 0;
        seen[*x] = true;
        last_seen[*x] = iteration;
        iteration += 1;
    }

    let mut last = initial[initial.len() - 1];

    while iteration <= target {
        let val = if seen[last] { number_val[last] } else { 0 };
        number_val[val] = if seen[val] {
            iteration - last_seen[val]
        } else {
            0
        };
        seen[val] = true;
        last_seen[val] = iteration;
        last = val;

        iteration += 1;
    }

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(436, part1(&vec![0, 3, 6]));
        assert_eq!(1, part1(&vec![1, 3, 2]));
        assert_eq!(10, part1(&vec![2, 1, 3]));
        assert_eq!(27, part1(&vec![1, 2, 3]));
    }

    #[test]
    fn run15() {
        let input = "0,12,6,13,20,1,17";
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
