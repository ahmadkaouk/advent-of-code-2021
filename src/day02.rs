#![feature(test)]
#![feature(array_windows)]

extern crate test;

const INPUT: &str = include_str!("../inputs/input02.txt");

fn part1() -> i32 {
    let (x, y) = INPUT
        .lines()
        .map(|cmd_str| cmd_str.split_once(' ').unwrap())
        .fold((0, 0), |(x, y), (c, i)| {
            match (c, i.parse::<i32>().unwrap()) {
                ("forward", i) => (x + i, y),
                ("down", i) => (x, y + i),
                ("up", i) => (x, y - i),
                _ => unreachable!(),
            }
        });
    x * y
}

fn part2() -> i32 {
    let (x, y, a) = INPUT
        .lines()
        .map(|cmd_str| cmd_str.split_once(' ').unwrap())
        .fold((0, 0, 0), |(x, y, a), (c, i)| {
            match (c, i.parse::<i32>().unwrap()) {
                ("forward", i) => (x + i, y + a * i, a),
                ("down", i) => (x, y, a + i),
                ("up", i) => (x, y, a - i),
                _ => unreachable!(),
            }
        });
    x * y
}

pub fn main() {
    println!("Part 1 Answer: {}", part1());
    println!("Part 2 Answer: {}", part2());
}

mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 1507611);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 1880593125);
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
