
pub fn count_l(input: &str) -> i32 {
    input.chars().filter(|c| *c == 'l').count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run() {
        let input = include_str!("./input.txt");
        println!("{}", input);
        assert_eq!(count_l(input), 3)
    }
}
