use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn day7_gen(input: &str) -> Vec<usize> {
    let crabs: Vec<_> = input
        .lines()
        .flat_map(|l| l.split(','))
        .flat_map(|l| l.parse::<usize>())
        .collect();
    let &max = crabs.iter().max().unwrap();
    let mut crab_counts = vec![0; max + 1];
    for crab_distance in crabs {
        crab_counts[crab_distance] += 1;
    }
    crab_counts
}

fn fuel_costs<'a, F: 'a>(input: &'a [usize], cost: F) -> impl Iterator<Item = usize> + 'a
where
    F: Fn(usize, usize, usize) -> usize,
{
    (0..input.len()).map(move |middle| {
        input
            .iter()
            .enumerate()
            .map(|(i, &crab)| cost(i, middle, crab))
            .sum()
    })
}

#[aoc(day7, part1)]
pub fn part1(input: &[usize]) -> usize {
    fuel_costs(input, |i, middle, crab| {
        (middle as isize - i as isize).unsigned_abs() * crab
    })
    .min()
    .unwrap()
}

#[aoc(day7, part2)]
pub fn part2(input: &[usize]) -> usize {
    fuel_costs(input, |i, middle, crab| {
        let distance_moved = (middle as isize - i as isize).unsigned_abs();
        (distance_moved * (distance_moved + 1)) * crab / 2
    })
    .min()
    .unwrap()
}

mod test {

    #[test]
    fn input() {
        use crate::day7::*;
        let input = day7_gen("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(input.len(), 17);
    }

    #[test]
    fn example() {
        use crate::day7::*;
        assert_eq!(part1(&day7_gen("16,1,2,0,4,2,7,1,2,14")), 37);
    }

    #[test]
    fn example2() {
        use crate::day7::*;
        assert_eq!(part2(&day7_gen("16,1,2,0,4,2,7,1,2,14")), 168);
    }
}
