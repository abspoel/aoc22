use std::io::Read;
use anyhow::{Ok, Result};

fn prio(ch: u8) -> i8 {
    let o = ch as i8 - b'a' as i8;
    if o < 0 { o + b'a' as i8 - b'A' as i8 + 26 } else { o }
}

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut sum = 0;
    for (a, b) in input.lines().map(str::trim).map(|l| l.split_at(l.len() / 2)) {
        let mut arr = [0u32; 52];
        for &ch in a.as_bytes().iter() {
            arr[prio(ch) as usize] += 1;
        }
        let dup = b.as_bytes().iter().copied().find(|&ch| arr[prio(ch) as usize] > 0).unwrap();
        sum += prio(dup) as i64 + 1;
    }
    println!("{}", sum);
    Ok(())
}
