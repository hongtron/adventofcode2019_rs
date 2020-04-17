use std::fs;

fn part_one() -> i32 {
    get_input()
        .lines()
        .map(|m| m.parse::<i32>().unwrap())
        .map(|m| m / 3 - 2)
        .sum()
}

fn get_input() -> String {
    fs::read_to_string("inputs/day01.txt").unwrap()
}

fn calculate_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        return 0
    } else {
        return fuel + calculate_fuel(fuel)
    }
}

fn part_two() -> i32 {
    get_input()
        .lines()
        .map(|m| m.parse::<i32>().unwrap())
        .map(|m| calculate_fuel(m))
        .sum()
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

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 4687331);
    }
}
