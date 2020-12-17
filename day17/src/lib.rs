use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

pub fn part1(input: &[Vec<bool>]) -> usize {
    let mut current_set = HashSet::<(i32, i32, i32)>::new();

    for (y, row) in input.iter().enumerate() {
        for (x, &is_set) in row.iter().enumerate() {
            if is_set {
                current_set.insert((x as i32, y as i32, 0));
            }
        }
    }

    for _ in 0..6 {
        //println!("filled: {}", current_set.len());
        //debug_print(&current_set);
        current_set = calculate_iteration(&current_set);
    }
    current_set.len()
}

fn calculate_iteration(last_state: &HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let init = (
        i32::max_value(),
        i32::min_value(),
        i32::max_value(),
        i32::min_value(),
        i32::max_value(),
        i32::min_value(),
    );
    let (min_x, max_x, min_y, max_y, min_z, max_z) = last_state.iter().fold(
        init,
        |(min_x, max_x, min_y, max_y, min_z, max_z), (x, y, z)| {
            (
                min_x.min(*x),
                max_x.max(*x),
                min_y.min(*y),
                max_y.max(*y),
                min_z.min(*z),
                max_z.max(*z),
            )
        },
    );

    let mut next_set = HashSet::<(i32, i32, i32)>::new();

    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            for z in (min_z - 1)..=(max_z + 1) {
                match (
                    last_state.contains(&(x, y, z)),
                    count_neighbors(x, y, z, last_state),
                ) {
                    (true, 2) | (true, 3) | (false, 3) => {
                        next_set.insert((x, y, z));
                    }
                    _ => {}
                };
            }
        }
    }

    next_set
}

fn count_neighbors(x: i32, y: i32, z: i32, last_state: &HashSet<(i32, i32, i32)>) -> u32 {
    let mut counter = 0;

    for x_iter in (x - 1)..=(x + 1) {
        for y_iter in (y - 1)..=(y + 1) {
            for z_iter in (z - 1)..=(z + 1) {
                if (x_iter != x || y_iter != y || z_iter != z)
                    && last_state.contains(&(x_iter, y_iter, z_iter))
                {
                    counter += 1;
                }
            }
        }
    }

    counter
}

pub fn part2(input: &[Vec<bool>]) -> usize {
    let mut current_set = HashSet::<(i32, i32, i32, i32)>::new();

    for (y, row) in input.iter().enumerate() {
        for (x, &is_set) in row.iter().enumerate() {
            if is_set {
                current_set.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    for _ in 0..6 {
        //println!("filled: {}", current_set.len());
        //debug_print(&current_set);
        current_set = calculate_iteration_4d(&current_set);
    }
    current_set.len()
}

fn calculate_iteration_4d(
    last_state: &HashSet<(i32, i32, i32, i32)>,
) -> HashSet<(i32, i32, i32, i32)> {
    let init = (
        i32::max_value(),
        i32::min_value(),
        i32::max_value(),
        i32::min_value(),
        i32::max_value(),
        i32::min_value(),
        i32::max_value(),
        i32::min_value(),
    );
    let (min_x, max_x, min_y, max_y, min_z, max_z, min_w, max_w) = last_state.iter().fold(
        init,
        |(min_x, max_x, min_y, max_y, min_z, max_z, min_w, max_w), (x, y, z, w)| {
            (
                min_x.min(*x),
                max_x.max(*x),
                min_y.min(*y),
                max_y.max(*y),
                min_z.min(*z),
                max_z.max(*z),
                min_w.min(*w),
                max_w.max(*w),
            )
        },
    );

    let mut next_set = HashSet::<(i32, i32, i32, i32)>::new();

    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            for z in (min_z - 1)..=(max_z + 1) {
                for w in (min_w - 1)..=(max_w + 1) {
                    match (
                        last_state.contains(&(x, y, z, w)),
                        count_neighbors_4d(x, y, z, w, last_state),
                    ) {
                        (true, 2) | (true, 3) | (false, 3) => {
                            next_set.insert((x, y, z, w));
                        }
                        _ => {}
                    };
                }
            }
        }
    }

    next_set
}

fn count_neighbors_4d(
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    last_state: &HashSet<(i32, i32, i32, i32)>,
) -> u32 {
    let mut counter = 0;

    for x_iter in (x - 1)..=(x + 1) {
        for y_iter in (y - 1)..=(y + 1) {
            for z_iter in (z - 1)..=(z + 1) {
                for w_iter in (w - 1)..=(w + 1) {
                    if (x_iter != x || y_iter != y || z_iter != z || w_iter != w)
                        && last_state.contains(&(x_iter, y_iter, z_iter, w_iter))
                    {
                        counter += 1;
                    }
                }
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = ".#.
..#
###";
        let parsed = parse(input);
        assert_eq!(112, part1(&parsed));
    }

    #[test]
    fn run17() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
