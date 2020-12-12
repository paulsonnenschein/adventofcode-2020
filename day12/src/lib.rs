use std::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    TurnLeft(i32),
    TurnRight(i32),
    Forward(i32),
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn forward(&self, x: &mut i32, y: &mut i32, amount: i32) {
        match self {
            Direction::North => *y += amount,
            Direction::East => *x += amount,
            Direction::South => *y -= amount,
            Direction::West => *x -= amount,
        };
    }

    fn turned(&self, angle: i32) -> Self {
        let current = match self {
            Direction::North => 0,
            Direction::East => 90,
            Direction::South => 180,
            Direction::West => 270,
        };

        match (current + angle + 360) % 360 {
            0 => Direction::North,
            90 => Direction::East,
            180 => Direction::South,
            270 => Direction::West,
            a => unreachable!("invalid angle {}", a),
        }
    }

    fn turn_waypoint(x: &mut i32, y: &mut i32, amount: i32) {
        match (amount + 360) % 360 {
            0 => {}
            90 => {
                let new_y = -*x;
                let new_x = *y;
                *x = new_x;
                *y = new_y;
            }
            180 => {
                let new_y = -*y;
                let new_x = -*x;
                *x = new_x;
                *y = new_y;
            }
            270 => {
                let new_y = *x;
                let new_x = -*y;
                *x = new_x;
                *y = new_y;
            }
            a => unreachable!("invalid angle {}", a),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ins, amount) = s.split_at(1);
        let i = amount.parse::<i32>().map_err(|_| ())?;
        match ins {
            "N" => Ok(Instruction::North(i)),
            "S" => Ok(Instruction::South(i)),
            "E" => Ok(Instruction::East(i)),
            "W" => Ok(Instruction::West(i)),
            "L" => Ok(Instruction::TurnLeft(i)),
            "R" => Ok(Instruction::TurnRight(i)),
            "F" => Ok(Instruction::Forward(i)),
            _ => Err(()),
        }
    }
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect()
}

pub fn part1(input: &[Instruction]) -> i32 {
    let mut x = 0i32;
    let mut y = 0i32;

    let mut direction = Direction::East;

    for i in input {
        match i {
            Instruction::North(i) => y += i,
            Instruction::South(i) => y -= i,
            Instruction::East(i) => x += i,
            Instruction::West(i) => x -= i,
            Instruction::TurnLeft(i) => direction = direction.turned(-*i),
            Instruction::TurnRight(i) => direction = direction.turned(*i),
            Instruction::Forward(i) => direction.forward(&mut x, &mut y, *i),
        };
    }

    x.abs() + y.abs()
}
pub fn part2(input: &[Instruction]) -> i32 {
    let mut x = 0i32;
    let mut y = 0i32;

    let mut delta_x = 10i32;
    let mut delta_y = 1i32;

    for i in input {
        match i {
            Instruction::North(i) => delta_y += i,
            Instruction::South(i) => delta_y -= i,
            Instruction::East(i) => delta_x += i,
            Instruction::West(i) => delta_x -= i,
            Instruction::TurnLeft(i) => Direction::turn_waypoint(&mut delta_x, &mut delta_y, -*i),
            Instruction::TurnRight(i) => Direction::turn_waypoint(&mut delta_x, &mut delta_y, *i),
            Instruction::Forward(i) => {
                x += delta_x * i;
                y += delta_y * i;
            }
        };
    }

    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run12() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
