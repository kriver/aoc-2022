pub fn part1() -> String {
    "world".to_string()
}

#[cfg(test)]
mod tests {
    use crate::day1::part1;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "world");
    }
}
