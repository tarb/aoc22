#![feature(iter_array_chunks)]

fn main() {
    let input = include_str!("input.txt");
    println!("Part One {}", part1(input));
    println!("Part Two {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input.lines()
        .map(|s| s.split_at(s.len()/2))
        .map(|(a, b)| items_set(a) & items_set(b))
        .map(|a| score(a))
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    input.lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| items_set(a) & items_set(b) & items_set(c))
        .map(|a| score(a))
        .sum::<u32>()
}

fn items_set(s: &str) -> u64 {
    s.bytes().fold(0u64, |set, b| {
        match b {
            b'a'..=b'z' => set | 1<<b-96,
            b'A'..=b'Z' => set | 1<<b-38,
            _ => set,
        }
    })
}

fn score(n: u64) -> u32 {
    (1..=52).fold(0,|v, i| {
        if n & (1<<i) > 0 {
            v + i
        } else {
            v
        }
    })
}
