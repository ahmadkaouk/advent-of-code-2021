#![feature(test)]
#![feature(array_windows)]

extern crate test;
const INPUT: &str = include_str!("../inputs/input01.txt");

fn part1() -> u32 {
    INPUT
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>()
        .array_windows()
        .filter(|[a, b]| a < b)
        .count() as u32
}

fn part2() -> u32 {
    INPUT
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>()
        .array_windows()
        .map(|[a, b, c]| a + b + c)
        .collect::<Vec<i32>>()
        .array_windows()
        .filter(|[a, b]| a < b)
        .count() as u32
}

pub fn main() {
    println!("Part 1: Answer {}", part1());
    println!("Part 2: Answer {} ", part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 1581);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 1618);
    }
    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(|| part1());
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(|| part2());
    }
}
