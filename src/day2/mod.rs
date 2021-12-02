use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
pub struct Command {
    dir: Direction,
    distance: isize,
}

#[aoc_generator(day2)]
pub fn day2_gen(input: &str) -> Vec<Command> {
    input
        .lines()
        .flat_map(|l| l.split_once(' '))
        .map(|(dir, dist)| {
            let dir = match dir {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => unreachable!(),
            };
            Command {
                dir,
                distance: dist.parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day2, part1, iter)]
pub fn part1(input: &[Command]) -> isize {
    let (end_depth, end_dist) = input
        .iter()
        .fold((0, 0), |(depth, dist), comm| match comm.dir {
            Direction::Forward => (depth, dist + comm.distance),
            Direction::Up => (depth - comm.distance, dist),
            Direction::Down => (depth + comm.distance, dist),
        });
    end_depth * end_dist
}

#[aoc(day2, part1, for)]
pub fn part1_for(input: &[Command]) -> isize {
    let mut depth = 0;
    let mut dist = 0;
    for comm in input.iter() {
        match comm.dir {
            Direction::Forward => {
                dist = dist + comm.distance;
            }
            Direction::Up => {
                depth = depth - comm.distance;
            }
            Direction::Down => {
                depth = depth + comm.distance;
            }
        }
    }
    depth * dist
}

#[aoc(day2, part2)]
pub fn part2(input: &[Command]) -> isize {
    let (end_depth, end_dist, _) =
        input
            .iter()
            .fold((0, 0, 0), |(depth, dist, aim), comm| match comm.dir {
                Direction::Forward => (depth + (aim * comm.distance), dist + comm.distance, aim),
                Direction::Up => (depth, dist, aim - comm.distance),
                Direction::Down => (depth, dist, aim + comm.distance),
            });
    end_depth * end_dist
}

mod test {

    #[test]
    fn basic() {
        use crate::day2::{day2_gen, part1};
        assert_eq!(dbg!(part1(&day2_gen(include_str!("basic.txt")))), 150);
    }

    #[test]
    fn basic2() {
        use crate::day2::{day2_gen, part2};
        assert_eq!(dbg!(part2(&day2_gen(include_str!("basic.txt")))), 900);
    }
}
