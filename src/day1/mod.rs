use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn day1_gen(input: &str) -> Vec<u32> {
    input.lines().flat_map(|l| l.parse::<u32>()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> usize {
    input.iter().tuple_windows().filter(|(f, l)| l > f).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(n1, n2, n3)| n1 + n2 + n3)
        .tuple_windows()
        .filter(|(f, l)| l > f)
        .count()
}
