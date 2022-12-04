use std::io::Read;
use anyhow::{Ok, Result};

fn modulo(a: i8, modulus: i8) -> i8 {
    let b = a % modulus;
    let half = modulus / 2;
    if b < -half {
        b + modulus
    } else if b > half {
        b - modulus
    } else {
        b
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let pairs = input.lines().map(|l| l.trim().split_once(' ').unwrap()).map(|(a, b)| ((a.as_bytes().first().unwrap() - b'A') as i8, (b.as_bytes().first().unwrap() - b'X') as i8)).collect::<Vec<_>>();
    let score = |(a, b)| 3 + 3 * modulo(b - a, 3) as i64 + (b + 1) as i64;
    let part1 = pairs.iter().map(|pair| score(*pair)).sum::<i64>();
    println!("Part 1: {}", part1);
    let part2: i64 = pairs.iter().map(|(a, b)| (3 + 3 * (b - 1) + (a + b - 1).rem_euclid(3) + 1) as i64).sum();
    println!("Part 2: {}", part2);

    Ok(())
}
