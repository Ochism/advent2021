use aoc_runner_derive::{aoc, aoc_generator};
use bitvec::prelude::*;
use bitvec::ptr::{BitRef, Const};
use std::cmp::Ordering;

#[aoc_generator(day3)]
pub fn day3_gen(input: &str) -> Vec<BitVec> {
    input
        .lines()
        .map(|l| BitVec::from_iter(l.chars().map(|c| c == '1')))
        .collect()
}

fn transpose(input: &[BitVec]) -> Vec<BitVec> {
    let empty = BitVec::from_iter((0..input.len()).map(|_| false));
    let mut out = vec![empty; input.get(0).map_or(0, BitVec::len)];
    input
        .iter()
        .enumerate()
        .for_each(|(j, bv)| bv.iter().enumerate().for_each(|(i, bf)| out[i].set(j, *bf)));

    out
}

fn value(input: &BitSlice) -> usize {
    input
        .iter()
        .rev()
        .enumerate()
        .filter(|(_, b)| **b)
        .map(|(i, _)| 2_usize.pow(i as u32))
        .sum()
}

#[aoc(day3, part1)]
pub fn part1(input: &[BitVec]) -> usize {
    let transposed = transpose(input);
    let total = transposed.get(0).map_or(0, BitVec::len);
    let set_bits = BitVec::from_iter(transposed.iter().map(|bv| bv.count_ones() > total / 2));
    let gamma = value(set_bits.as_bitslice());
    let epsilon: usize = value(
        BitVec::from_iter(set_bits.iter().map(|bf: BitRef<'_, Const, Lsb0>| !bf)).as_bitslice(),
    );
    gamma * epsilon
}
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum MostCommon {
    One,
    Zero,
    Neither,
}

fn find_rating<F>(input: &[BitVec], criteria: F) -> usize
where
    F: Fn(MostCommon, BitRef<Const>) -> bool,
{
    let mut data: Vec<_> = input.iter().collect();
    let bits = input.get(0).map_or(0, BitVec::len);

    for i in 0..bits {
        let count = data
            .iter()
            .flat_map(|bv| bv.get(i))
            .filter(|br| **br)
            .count();

        let mc = match count.cmp(&(data.len() - count)) {
            Ordering::Greater => MostCommon::One,
            Ordering::Less => MostCommon::Zero,
            Ordering::Equal => MostCommon::Neither,
        };
        data = data
            .iter()
            .filter(|&bv| criteria(mc, bv.get(i).unwrap()))
            .copied()
            .collect();
        if data.len() == 1 {
            return value(data.get(0).unwrap());
        }
    }

    unreachable!("failed to converge")
}

#[aoc(day3, part2)]
pub fn part2(input: &[BitVec]) -> usize {
    let oxy = |mc, br: BitRef<Const>| match mc {
        MostCommon::Zero => !*br,
        MostCommon::One => *br,
        MostCommon::Neither => *br,
    };
    let co2 = |mc, br: BitRef<Const>| match mc {
        MostCommon::Zero => *br,
        MostCommon::One => !*br,
        MostCommon::Neither => !*br,
    };
    find_rating(input, oxy) * find_rating(input, co2)
}

mod test {

    #[test]
    fn simple_input() {
        use crate::day3::*;
        use bitvec::bits;
        assert_eq!(&day3_gen("0101100"), &vec![bits![0, 1, 0, 1, 1, 0, 0]]);
    }
    #[test]
    fn transpose() {
        use crate::day3::*;
        use bitvec::bits;
        assert_eq!(
            &transpose(&day3_gen("10\n10")),
            &vec![bits![1, 1], bits![0, 0]]
        );
    }
    #[test]
    fn basic() {
        use crate::day3::*;
        assert_eq!(part1(&day3_gen(include_str!("basic.txt"))), 198);
    }
    #[test]
    fn simple2() {
        use crate::day3::*;
        assert_eq!(part2(&day3_gen("10\n01")), 2);
    }
    #[test]
    fn basic2() {
        use crate::day3::*;
        assert_eq!(part2(&day3_gen(include_str!("basic.txt"))), 230);
    }
}
