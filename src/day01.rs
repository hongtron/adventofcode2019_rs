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
        // we can't chain like get_input().lines().collect() because lines() produces slices, and
        // the underlying string (i.e. the string referenced by the slices) gets freed if its not
        // persisted in a variable
        let input = get_input();
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(lines[0], "62371");
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 3126794);
    }
}
