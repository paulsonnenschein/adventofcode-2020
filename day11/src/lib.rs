use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SeatState {
    Floor,
    Empty,
    Taken,
}

impl TryFrom<char> for SeatState {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(SeatState::Floor),
            'L' => Ok(SeatState::Empty),
            '#' => Ok(SeatState::Taken),
            _ => Err(()),
        }
    }
}

pub fn parse(input: &str) -> Vec<Vec<SeatState>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| SeatState::try_from(c).unwrap())
                .collect()
        })
        .collect()
}

pub fn part(input: &[Vec<SeatState>], seat_iter: SeatIter) -> usize {
    let mut old = input.to_owned();

    loop {
        let next = next_iter(&old, seat_iter);
        if next == old {
            break;
        }
        old = next;
    }

    count_taken(&old)
}

pub type SeatIter = fn(input: &[Vec<SeatState>], x: usize, y: usize) -> SeatState;

fn next_iter(input: &[Vec<SeatState>], iter: SeatIter) -> Vec<Vec<SeatState>> {
    let mut result = input.to_owned();

    for (y, row) in input.iter().enumerate() {
        for (x, _seat) in row.iter().enumerate() {
            result[y][x] = iter(input, x, y);
        }
    }

    result
}

fn count_taken(input: &[Vec<SeatState>]) -> usize {
    input
        .iter()
        .flat_map(|row| row.iter().filter(|&seat| seat == &SeatState::Taken))
        .count()
}

pub fn next_state_part1(input: &[Vec<SeatState>], x: usize, y: usize) -> SeatState {
    let mut occupied = 0;

    if x > 0 && y > 0 && input[y - 1][x - 1] == SeatState::Taken {
        occupied += 1;
    }
    if x > 0 && input[y][x - 1] == SeatState::Taken {
        occupied += 1;
    }
    if x > 0 && y + 1 < input.len() && input[y + 1][x - 1] == SeatState::Taken {
        occupied += 1;
    }
    if y + 1 < input.len() && input[y + 1][x] == SeatState::Taken {
        occupied += 1;
    }
    if x + 1 < input[0].len() && y + 1 < input.len() && input[y + 1][x + 1] == SeatState::Taken {
        occupied += 1;
    }
    if x + 1 < input[0].len() && input[y][x + 1] == SeatState::Taken {
        occupied += 1;
    }
    if x + 1 < input[0].len() && y > 0 && input[y - 1][x + 1] == SeatState::Taken {
        occupied += 1;
    }
    if y > 0 && input[y - 1][x] == SeatState::Taken {
        occupied += 1;
    }

    match input[y][x] {
        SeatState::Empty if occupied == 0 => SeatState::Taken,
        SeatState::Taken if occupied >= 4 => SeatState::Empty,
        s => s,
    }
}

pub fn next_state_part2(input: &[Vec<SeatState>], x: usize, y: usize) -> SeatState {
    let mut occupied = 0;
    let mut x_ = x;
    let mut y_ = y;

    // todo calculate lookup map once (map that stores where to look at for each coordinate)
    // top-left
    while y_ > 0 && x_ > 0 {
        y_ -= 1;
        x_ -= 1;
        if input[y_][x_] == SeatState::Taken {
            occupied += 1;
            break;
        }
        if input[y_][x_] == SeatState::Empty {
            break;
        }
    }
    x_ = x;
    y_ = y;
    // left
    while x_ > 0 {
        x_ -= 1;
        if input[y_][x_] == SeatState::Taken {
            occupied += 1;
            break;
        }
        if input[y_][x_] == SeatState::Empty {
            break;
        }
    }
    x_ = x;
    y_ = y;
    // bottom-left
    while x_ > 0 && y_ + 1 < input.len() {
        x_ -= 1;
        y_ += 1;
        if input[y_][x_] == SeatState::Taken {
            occupied += 1;
            break;
        }
        if input[y_][x_] == SeatState::Empty {
            break;
        }
    }
    x_ = x;
    y_ = y;
    // bottom
    while y_ + 1 < input.len() {
        y_ += 1;
        if input[y_][x_] == SeatState::Taken {
            occupied += 1;
            break;
        }
        if input[y_][x_] == SeatState::Empty {
            break;
        }
    }
    x_ = x;
    y_ = y;
    // bottom-right
    while x_ + 1 < input[0].len() && y_ + 1 < input.len() {
        x_ += 1;
        y_ += 1;
        if input[y_][x_] == SeatState::Taken {
            occupied += 1;
            break;
        }
        if input[y_][x_] == SeatState::Empty {
            break;
        }
    }
    x_ = x;
    y_ = y;
    // right
    while x_ + 1 < input[0].len() {
        x_ += 1;
        if input[y_][x_] == SeatState::Taken {
            occupied += 1;
            break;
        }
        if input[y_][x_] == SeatState::Empty {
            break;
        }
    }
    x_ = x;
    y_ = y;
    // top-right
    while x_ + 1 < input[0].len() && y_ > 0 {
        x_ += 1;
        y_ -= 1;
        if input[y_][x_] == SeatState::Taken {
            occupied += 1;
            break;
        }
        if input[y_][x_] == SeatState::Empty {
            break;
        }
    }
    x_ = x;
    y_ = y;
    // top
    while y_ > 0 {
        y_ -= 1;
        if input[y_][x_] == SeatState::Taken {
            occupied += 1;
            break;
        }
        if input[y_][x_] == SeatState::Empty {
            break;
        }
    }

    match input[y][x] {
        SeatState::Empty if occupied == 0 => SeatState::Taken,
        SeatState::Taken if occupied >= 5 => SeatState::Empty,
        s => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run11() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part(&parsed, next_state_part1));
        println!("{:?}", part(&parsed, next_state_part2));
    }
}
