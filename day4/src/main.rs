#![feature(iter_array_chunks)]

use std::cmp::{max, min};

const INPUT_DELIMS: [char; 2] = ['-', ','];

fn main() {
    let input = include_str!("input.txt");
    println!("Part One {}", part1(input));
    println!("Part Two {}", part2(input));
}

fn nums(input: &str) -> impl Iterator<Item = [u32; 4]> + '_ {
    input
        .lines()
        .flat_map(|s| s.splitn(4, &INPUT_DELIMS[..]))
        .map(|s| s.parse::<u32>().unwrap())
        .array_chunks::<4>()
}

fn part1(input: &str) -> u32 {
    nums(input)
        .map(|[a1, a2, b1, b2]| ((a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2)) as u32)
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    nums(input)
        .map(|[a1, a2, b1, b2]| (max(a1, b1) <= min(a2, b2)) as u32)
        .sum::<u32>()
}
