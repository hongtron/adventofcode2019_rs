use std::fs;

fn part_one() -> i32 {
    get_input()
        .lines()
        .map(|m| m.parse::<i32>().unwrap() / 3 - 2)
        .sum()
}

fn get_input() -> String {
    fs::read_to_string("inputs/day01.txt").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        assert_eq!(get_input().lines().next().unwrap(), "62371");
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 3126794);
    }
}
