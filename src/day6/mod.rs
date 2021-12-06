use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn day6_gen(input: &str) -> [usize; 9] {
    let mut result = [0; 9];
    for timer in input.trim().split(',').flat_map(str::parse::<usize>) {
        result[timer] += 1;
    }
    result
}

fn run_day6_times(input: &[usize; 9], times: u32) -> usize {
    let mut timers = *input;

    for _ in 0..times {
        let mut next = [0; 9];

        next[..8].clone_from_slice(&timers[1..9]);

        next[8] = timers[0];
        next[6] += timers[0];

        timers = next;
    }

    timers.iter().sum()
}
#[aoc(day6, part1)]
pub fn part1(input: &[usize; 9]) -> usize {
    run_day6_times(input, 80)
}

#[aoc(day6, part2)]
pub fn part2(input: &[usize; 9]) -> usize {
    run_day6_times(input, 256)
}

mod test {

    #[test]
    fn input() {
        use crate::day6::day6_gen;
        assert_eq!(day6_gen("3,4,3,1,2"), [0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn basic() {
        use crate::day6::{day6_gen, part1};
        assert_eq!(part1(&day6_gen("3,4,3,1,2")), 5934);
    }

    #[test]
    fn basic2() {
        use crate::day6::{day6_gen, part2};
        assert_eq!(part2(&day6_gen("3,4,3,1,2")), 26984457539);
    }
}
