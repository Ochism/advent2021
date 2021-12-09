use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::hash_map::RandomState;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Readings {
    sequences: Vec<HashSet<char, RandomState>>,
    output: Vec<HashSet<char, RandomState>>,
}

#[aoc_generator(day8)]
pub fn day8_gen(input: &str) -> Vec<Readings> {
    input
        .lines()
        .map(|l| {
            let (start, end) = l.split_once(" | ").unwrap();
            Readings {
                sequences: start
                    .split(' ')
                    .map(|s| HashSet::from_iter(s.chars()))
                    .collect(),
                output: end
                    .split(' ')
                    .map(|s| HashSet::from_iter(s.chars()))
                    .collect(),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[Readings]) -> usize {
    input
        .iter()
        .flat_map(|r| r.output.iter())
        .filter(|&letter| matches!(letter.len(), 2 | 3 | 4 | 7))
        .count()
}

fn find_by_len(values: &[HashSet<char, RandomState>], len: usize) -> &HashSet<char, RandomState> {
    values.iter().find(|&item| item.len() == len).unwrap()
}

fn find_by<P>(sets: &[HashSet<char, RandomState>], pred: P) -> &HashSet<char, RandomState>
where
    P: FnMut(&&HashSet<char>) -> bool,
{
    let mut filtered = sets.iter().filter(pred);
    let out = filtered.next();
    let dup = filtered.next();
    if dup != None {
        dbg!(out);
        dbg!(dup);
    }
    assert_eq!(dup, None);
    out.unwrap()
}

fn solve_output(reading: &Readings) -> usize {
    let one = find_by_len(&reading.sequences, 2);
    let seven = find_by_len(&reading.sequences, 3);
    let four = find_by_len(&reading.sequences, 4);
    let eight = find_by_len(&reading.sequences, 7);

    let three = find_by(&reading.sequences, |&set| {
        set.len() == 5 && set.is_superset(one)
    });
    let nine = find_by(&reading.sequences, |&set| {
        set.intersection(three).count() == 5 && set.len() == 6
    });
    let zero = find_by(&reading.sequences, |&set| {
        set.len() == 6 && set != nine && set.intersection(one).count() == 2
    });
    let six = find_by(&reading.sequences, |&set| {
        set.len() == 6 && set != nine && set.intersection(one).count() == 1
    });

    let two = find_by(&reading.sequences, |&set| {
        set.len() == 5 && set.intersection(nine).count() == 4
    });

    let five = find_by(&reading.sequences, |&set| {
        set.len() == 5 && set.intersection(nine).count() == 5 && set.intersection(one).count() == 1
    });

    let lookup = &vec![
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ];

    let out = reading
        .output
        .iter()
        .map(|number| {
            lookup
                .iter()
                .find(|(set, _)| *set == number)
                .map(|(_, value)| value)
                .unwrap_or_else(|| panic!("failed to find n: {n:?}", n = number))
        })
        .rev()
        .enumerate()
        .fold(0usize, |acc, (power, &value)| {
            acc + value * (10usize.pow(power as u32))
        });
    out
}

#[aoc(day8, part2)]
pub fn part2(input: &[Readings]) -> usize {
    input.iter().map(solve_output).sum()
}

mod test {
    #[test]
    fn one_line() {
        use crate::day8::*;
        let input = day8_gen(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        assert_eq!(input.len(), 1);
        let input = input.get(0).unwrap();
        assert_eq!(input.sequences.len(), 10);
        assert_eq!(input.output.len(), 4);
        assert_eq!(part2(&[input.clone()]), 5353);
    }

    #[test]
    fn example() {
        use crate::day8::*;
        assert_eq!(part1(&day8_gen(include_str!("example.txt"))), 26);
    }

    #[test]
    fn example2() {
        use crate::day8::*;
        assert_eq!(part2(&day8_gen(include_str!("example.txt"))), 61229);
    }
}
