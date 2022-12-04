use anyhow::{Ok, Result};
fn main() -> Result<()> {
    let input = std::fs::read_to_string("input")?;
    let groups = input.split("\n\n").map(|s| s.trim().lines().map(|l| l.parse::<i64>().unwrap()).sum());
    let mut vec: Vec<i64> = groups.collect();
    let max: i64 = vec.iter().copied().max().unwrap();
    vec.as_mut_slice().sort_unstable_by(|a, b| b.cmp(a));
    let top3: i64 = vec.into_iter().take(3).sum();
    println!("{}", max);
    println!("{}", top3);
    Ok(())
}
