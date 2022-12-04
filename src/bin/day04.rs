fn to_array<T>(a: (T, T)) -> [T; 2] { [a.0, a.1] }
fn overlaps(a: [i32; 2], b: [i32; 2]) -> bool { a[0] >= b[0] && a[0] <= b[1] }
fn is_subrange(a: [i32; 2], b: [i32; 2]) -> bool { a[0] >= b[0] && a[1] <= b[1] }

fn main() {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();
    let vec = input
        .lines()
        .map(|l| {
            to_array(l.trim().split_once(',').unwrap())
                .map(|s| to_array(s.split_once('-').unwrap()).map(|v| v.parse::<i32>().unwrap()))
        })
        .collect::<Vec<_>>();
    let part1 = vec.iter().copied().map(|[a, b]| (is_subrange(a, b) || is_subrange(b, a)) as i32)
        .sum::<i32>();
    println!("{}", part1);
    let part2 = vec.iter().copied().map(|[a, b]| (overlaps(a, b) || overlaps(b, a)) as i32)
        .sum::<i32>();
    println!("{}", part2);
}

