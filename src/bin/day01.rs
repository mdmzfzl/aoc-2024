use aoc_2024::read_input;
use itertools::Itertools;
use std::{collections::HashMap, iter};

const DAY: u8 = 1;

fn main() {
    let input = read_input(DAY);

    println!("Answer [Part 1]: {}", part1(parse(&input)));
    println!("Answer [Part 2]: {}", part2(parse(&input)));
}

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let pairs: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let values: Vec<u64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (values[0], values[1])
        })
        .collect();

    pairs.into_iter().unzip()
}

/// Part 1 Solution
fn part1((mut left, mut right): (Vec<u64>, Vec<u64>)) -> u64 {
    left.sort_unstable();
    right.sort_unstable();

    // left.iter()
    //     .zip(right.iter())
    //     .map(|(l, r)| l.abs_diff(*r))
    //     .sum()
    iter::zip(left, right).map(|(l, r)| l.abs_diff(r)).sum()
}

/// Part 2 Solution
fn part2((left, right): (Vec<u64>, Vec<u64>)) -> u64 {
    // let mut right_freq_table = HashMap::new();
    // for num in right {
    //     *right_freq_table.entry(num).or_insert(0) += 1;
    // }
    let right_count = right.into_iter().counts();

    // left.iter()
    //     .map(|&a| {
    //         let count = right_count.get(&a).unwrap_or(&0);
    //         a * (*count as u64)
    //     })
    //     .sum()

    left.iter()
        .filter_map(|l| right_count.get(l).map(|count| l * (*count as u64)))
        .sum()
}
