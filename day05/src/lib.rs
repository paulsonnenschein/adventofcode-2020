pub type Pass = Vec<bool>;

pub fn parse(input: &str) -> Vec<Pass> {
    input.lines().map(parse_row).collect()
}

pub fn parse_row(row: &str) -> Pass {
    row.chars().map(|c| matches!(c, 'B' | 'R')).collect()
}

pub fn identify_seat(pass: &Pass) -> (u32, u32) {
    (binary(&pass[0..7], 0, 127), binary(&pass[7..10], 0, 7))
}

fn binary(directions: &[bool], lower: u32, upper: u32) -> u32 {
    if directions.is_empty() {
        lower
    } else if directions[0] {
        binary(&directions[1..], (lower + upper + 1) / 2, upper)
    } else {
        binary(&directions[1..], lower, (lower + upper) / 2)
    }
}

pub fn part1(input: &[Pass]) -> u32 {
    input
        .iter()
        .map(identify_seat)
        .map(|(row, column)| row * 8 + column)
        .max()
        .unwrap()
}

pub fn part2(input: &[Pass]) -> u32 {
    let mut seats: Vec<_> = input
        .iter()
        .map(identify_seat)
        .map(|(row, column)| row * 8 + column)
        .collect();

    seats.sort_unstable();

    let mut peekable = seats.iter().peekable();

    while let Some(next) = peekable.next() {
        if Some(next + 2).as_ref().as_ref() == peekable.peek() {
            return next + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            vec![false, true, false, true, true, false, false, true, false, true],
            parse_row("FBFBBFFRLR")
        )
    }

    #[test]
    fn test_identify_seat() {
        assert_eq!(identify_seat(&parse_row("FBFBBFFRLR")), (44, 5))
    }

    #[test]
    fn run05() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
