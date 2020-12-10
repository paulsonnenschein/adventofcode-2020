pub fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn part1(input: &[u32]) -> u32 {
    let mut current = 0;
    let mut incr = [0; 4];

    for jolt in input {
        let diff = (jolt - current) as usize;
        match diff {
            3 | 2 | 1 | 0 => incr[diff] += 1,
            _ => unreachable!(),
        }
        current = *jolt;
    }

    incr[3] += 1;

    incr[1] * incr[3]
}

pub fn part2(input: &[u32]) -> u64 {
    let mut counters = vec![0u64; 10_000];

    counters[0] = 1; // start has one combination

    for jolt in input {
        let total = calculate_combinations(&counters, *jolt);

        counters[*jolt as usize] = total;
    }

    calculate_combinations(&counters, input[input.len() - 1] + 3)
}

fn calculate_combinations(counters: &[u64], jolt: u32) -> u64 {
    let jolt = jolt as usize;
    let mut total = 0;
    if counters[jolt] > 0 {
        total += counters[jolt];
    }
    if jolt >= 1 && counters[jolt - 1] > 0 {
        total += counters[jolt - 1];
    }
    if jolt >= 2 && counters[jolt - 2] > 0 {
        total += counters[jolt - 2];
    }
    if jolt >= 3 && counters[jolt - 3] > 0 {
        total += counters[jolt - 3];
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run10() {
        let input = include_str!("./input.txt");
        let mut parsed = parse(input);
        parsed.sort_unstable();
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
