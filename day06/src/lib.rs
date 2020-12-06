use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

pub fn part1(input: &[Vec<HashSet<char>>]) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .flat_map(|set| set.iter())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

pub fn part2(input: &[Vec<HashSet<char>>]) -> usize {
    input
        .iter()
        .map(|group| {
            let mut iter = group.iter();
            iter.next()
                .map(|set| iter.fold(set.clone(), |set1, set2| &set1 & set2).len())
                .unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run06() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        //println!("{:?}", parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
