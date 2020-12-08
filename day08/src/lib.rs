use std::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let instr = split.next();
        let value = split.next().and_then(|v| v.parse::<i32>().ok());
        match (instr, value) {
            (Some("nop"), Some(v)) => Ok(Instruction::Nop(v)),
            (Some("acc"), Some(v)) => Ok(Instruction::Acc(v)),
            (Some("jmp"), Some(v)) => Ok(Instruction::Jmp(v)),
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
    if let Err(v) = execute(input) {
        v
    } else {
        unreachable!()
    }
}

fn execute(input: &[Instruction]) -> Result<i32, i32> {
    let mut acc = 0;
    let mut current_instruction: i32 = 0;
    let mut visited = vec![false; input.len()];

    while current_instruction < input.len() as i32 && !visited[current_instruction as usize] {
        visited[current_instruction as usize] = true;
        match input[current_instruction as usize] {
            Instruction::Nop(_) => {
                current_instruction += 1;
            }
            Instruction::Acc(v) => {
                acc += v;
                current_instruction += 1;
            }
            Instruction::Jmp(v) => {
                current_instruction += v;
            }
        }
    }

    if current_instruction < input.len() as i32 {
        Err(acc)
    } else {
        Ok(acc)
    }
}

pub fn part2(mut input: Vec<Instruction>) -> i32 {
    for i in 0..input.len() {
        match input[i] {
            Instruction::Nop(v) => {
                input[i] = Instruction::Jmp(v);
                if let Ok(v) = execute(&input) {
                    return v;
                }
                input[i] = Instruction::Nop(v);
            }
            Instruction::Jmp(v) => {
                input[i] = Instruction::Nop(v);
                if let Ok(v) = execute(&input) {
                    return v;
                }
                input[i] = Instruction::Jmp(v);
            }
            Instruction::Acc(_) => {} // ignore
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run08() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed));
    }
}
