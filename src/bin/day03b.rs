use std::io::Read;
use itertools::Itertools;
use anyhow::{Ok, Result};

fn prio(ch: u8) -> i8 {
    let o = ch as i8 - b'a' as i8;
    if o < 0 { o + b'a' as i8 - b'A' as i8 + 26 } else { o }
}

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut sum = 0;
    for chunk in &input.lines().map(str::trim).chunks(3) {
        let mut arr = [0u8; 52];
        for (i, l) in chunk.enumerate() {
            for ch in l.as_bytes() {
                arr[prio(*ch) as usize] |= 1 << i;
            }
        }
        sum += arr.iter().copied().enumerate().find(|(_, x)| *x == 7).unwrap().0 + 1;
    }
    println!("{}", sum);
    Ok(())
}
