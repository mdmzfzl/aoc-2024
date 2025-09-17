use aoc_2024::read_input;

const DAY: u8 = 0;

fn main() {
    let input = read_input(DAY);

    println!("Answer [Part 1]: {}", part1(&input));
    println!("Answer [Part 2]: {}", part2(&input));
}

/// Part 1 Solution
fn part1(input: &str) -> i32 {
    0
}

/// Part 2 Solution
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
