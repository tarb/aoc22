fn main() {
    let input = include_str!("input.txt");
    println!("Part1 {}", part1(input));
    println!("Part2 {}", part2::<3>(input));
}

fn cals(input: &str) -> impl Iterator<Item = u32> + '_  {
    input
    .split("\n\n")
    .map(|s| {
        s.split('\n')
            .filter_map(|s| s.parse::<u32>().ok())
            .sum::<u32>()
    })
}

fn part1(input: &str) -> u32 {
    cals(input).max().unwrap_or(0)
}

fn part2<const N: usize>(input: &str) -> u32 {
    cals(input).max_n::<N>().iter().sum()
}

trait MaxNExt: Iterator {
    fn max_n<const N: usize>(&mut self) -> [Self::Item; N]
    where
        Self::Item: Ord + Default + Copy,
        Self: Sized,
    {
        self.fold([Self::Item::default(); N], |mut a, v| {
            if v > a[0] {
                let i = (a.len() - 1) - a.iter().rev().position(|p| v > *p).unwrap();
                if i != 0 {
                    a[0..=i].rotate_left(1);
                }
                a[i] = v;
            }
            a
        })
    }
}

impl<I: Iterator> MaxNExt for I {}
