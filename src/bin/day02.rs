use aoc_2024::read_input;

const DAY: u8 = 2;

fn main() {
    let input = read_input(DAY);
    let input = parse(&input);

    println!("Answer [Part 1]: {}", part1(&input));
    println!("Answer [Part 2]: {}", part2(&input));
}

fn parse(s: &str) -> Vec<Vec<u64>> {
    let mut out = Vec::new();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut row = Vec::new();
        for s in line.split_whitespace() {
            row.push(s.parse::<u64>().unwrap());
        }
        out.push(row);
    }
    out
}

fn check(vals: &[u64], skip: Option<u64>) -> bool {
    let skip = skip.map(|s| s as usize);
    let mut prev: Option<u64> = None;
    // 0 = unknown, 1 = increasing, -1 = decreasing
    let mut flag: i8 = 0;

    for (idx, &curr) in vals.iter().enumerate() {
        if Some(idx) == skip {
            continue;
        }
        if let Some(prev) = prev {
            if curr == prev {
                return false;
            }
            let step = curr.abs_diff(prev);
            if step < 1 || step > 3 {
                return false;
            }
            if flag == 0 {
                flag = if curr > prev { 1 } else { -1 };
            } else if (flag == 1 && curr <= prev) || (flag == -1 && curr >= prev) {
                return false;
            }
        }
        prev = Some(curr);
    }
    true
}

fn part1(rows: &[Vec<u64>]) -> u64 {
    let mut count: u64 = 0;

    for values in rows {
        if check(values, None) {
            count += 1;
        }
    }
    count
}

fn part2(rows: &[Vec<u64>]) -> u64 {
    let mut count: u64 = 0;

    for values in rows {
        if check(values, None) {
            count += 1;
            continue;
        }
        let mut flag = false;
        for i in 0..values.len() {
            if check(values, Some(i as u64)) {
                flag = true;
                break;
            }
        }
        if flag {
            count += 1;
        }
    }
    count
}
