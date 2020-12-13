use modinverse::modinverse;

pub fn parse(input: &str) -> (i32, Vec<Option<i32>>) {
    let mut lines = input.lines();
    let time = lines
        .next()
        .and_then(|line| line.parse::<i32>().ok())
        .unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|c| match c {
            "x" => None,
            v => Some(v.parse::<i32>().unwrap()),
        })
        .collect();

    (time, busses)
}

pub fn part1(time: i32, busses: &[Option<i32>]) -> i32 {
    busses
        .iter()
        .flatten()
        .map(|&bus_time| {
            (
                bus_time,
                (time as f32 / bus_time as f32).ceil() as i32 * bus_time,
            )
        })
        .min_by_key(|(_, time)| *time)
        .map(|(bus, closest)| bus * (closest - time))
        .unwrap()
}

// brute force is way too slow...
// stolen from here: https://www.reddit.com/r/adventofcode/comments/kc4njx/2020_day_13_solutions/gfov7ib/
pub fn part2(busses: &[Option<i32>]) -> i64 {
    let mapped_busses = busses
        .iter()
        .enumerate()
        .flat_map(|(i, time)| match time {
            None => None,
            Some(t) => Some((i as i64, *t as i64)),
        })
        .collect::<Vec<(i64, i64)>>();

    let product = mapped_busses
        .iter()
        .map(|(_offset, time)| time)
        .product::<i64>();

    mapped_busses
        .iter()
        .map(|(offset, time)| {
            let product_without_id = product / time;
            let modular_inverse = modinverse(product_without_id, *time).unwrap();
            (time - offset % time) * product_without_id * modular_inverse
        })
        .sum::<i64>()
        % product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_part2() {
        let busses = vec![Some(17), None, Some(13), Some(19)];
        assert_eq!(3417, part2(&busses));
        let busses = vec![Some(67), Some(7), Some(59), Some(61)];
        assert_eq!(754018, part2(&busses));
        let busses = vec![Some(67), None, Some(7), Some(59), Some(61)];
        assert_eq!(779210, part2(&busses));
        let busses = vec![Some(67), Some(7), None, Some(59), Some(61)];
        assert_eq!(1261476, part2(&busses));
        let busses = vec![Some(1789), Some(37), Some(47), Some(1889)];
        assert_eq!(1202161486, part2(&busses));
    }

    #[test]
    fn run13() {
        let input = include_str!("./input.txt");
        let (time, busses) = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(time, &busses));
        println!("{:?}", part2(&busses));
    }
}
