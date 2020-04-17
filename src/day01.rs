use std::fs;

fn part_one() -> i32 {
    let mut fuel_total = 0;
    let masses = get_input();
    for mass in masses.lines() {
        let parsed_mass = mass.parse::<i32>().unwrap();
        let fuel = parsed_mass/3 - 2;
        fuel_total += fuel;
    }

   fuel_total
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
