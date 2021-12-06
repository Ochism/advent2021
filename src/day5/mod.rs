use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    start: (u16, u16),
    end: (u16, u16),
}

impl Line {
    fn straight_points(&self) -> Box<dyn Iterator<Item = (u16, u16)>> {
        if self.start.0 == self.end.0 {
            let x = self.start.0;
            let min = self.start.1.min(self.end.1);
            let max = self.start.1.max(self.end.1);

            Box::new((min..=max).map(move |y| (x, y)))
        } else if self.start.1 == self.end.1 {
            let y = self.start.1;
            let min = self.start.0.min(self.end.0);
            let max = self.start.0.max(self.end.0);
            Box::new((min..=max).map(move |x| (x, y)))
        } else {
            Box::new(std::iter::empty())
        }
    }

    fn points(&self) -> Box<dyn Iterator<Item = (u16, u16)>> {
        if self.start.0 == self.end.0 || self.start.1 == self.end.1 {
            self.straight_points()
        } else {
            let min_x = self.start.0.min(self.end.0);
            let max_x = self.start.0.max(self.end.0);
            let start_x = self.start.0 as i16;
            let start_y = self.start.1 as i16;

            let dist = (max_x - min_x) as i16;
            let step_x = (self.end.0 as i16 - self.start.0 as i16) / dist;
            let step_y = (self.end.1 as i16 - self.start.1 as i16) / dist;
            Box::new((0i16..=dist).map(move |mag| {
                (
                    (start_x + step_x * mag) as u16,
                    (start_y + step_y * mag) as u16,
                )
            }))
        }
    }
}

#[aoc_generator(day5)]
pub fn day5_gen(input: &str) -> Vec<Line> {
    input
        .lines()
        .flat_map(|l| l.split(" -> "))
        .flat_map(|p| p.split(','))
        .flat_map(str::parse)
        .tuples()
        .map(|(start_x, start_y, end_x, end_y)| Line {
            start: (start_x, start_y),
            end: (end_x, end_y),
        })
        .collect()
}

fn count_2_overlaps<I>(input: I) -> usize
where
    I: Iterator<Item = (u16, u16)>,
{
    let mut points = HashMap::new();
    for point in input {
        *points.entry(point).or_insert(0usize) += 1;
    }
    points.values().filter(|&&count| count > 1usize).count()
}

#[aoc(day5, part1)]
pub fn part1(input: &[Line]) -> usize {
    count_2_overlaps(input.iter().flat_map(|line| line.straight_points()))
}

#[aoc(day5, part2)]
pub fn part2(input: &[Line]) -> usize {
    count_2_overlaps(input.iter().flat_map(|line| line.points()))
}

mod test {

    #[test]
    fn straight_lines() {
        use crate::day5::Line;
        let line = Line {
            start: (0, 0),
            end: (0, 3),
        };
        assert_eq!(
            line.straight_points().collect::<Vec<_>>(),
            vec![(0, 0), (0, 1), (0, 2), (0, 3)]
        )
    }

    #[test]
    fn diagonal() {
        use crate::day5::Line;
        let line = Line {
            start: (0, 0),
            end: (2, 2),
        };
        assert_eq!(
            line.points().collect::<Vec<_>>(),
            vec![(0, 0), (1, 1), (2, 2)]
        );
        let line = Line {
            start: (2, 2),
            end: (0, 0),
        };
        assert_eq!(
            line.points().collect::<Vec<_>>(),
            vec![(2, 2), (1, 1), (0, 0)]
        );
        let line = Line {
            start: (0, 2),
            end: (2, 0),
        };
        assert_eq!(
            line.points().collect::<Vec<_>>(),
            vec![(0, 2), (1, 1), (2, 0)]
        );
        let line = Line {
            start: (2, 0),
            end: (0, 2),
        };
        assert_eq!(
            line.points().collect::<Vec<_>>(),
            vec![(2, 0), (1, 1), (0, 2)]
        );
    }

    #[test]
    fn basic_input() {
        use crate::day5::*;
        let input = day5_gen(include_str!("basic.txt"));
        assert_eq!(input.len(), 10);
        assert_eq!(input.iter().flat_map(Line::straight_points).count(), 26);
        assert_eq!(input.iter().flat_map(Line::points).count(), 53);
    }

    #[test]
    fn basic() {
        use crate::day5::*;
        assert_eq!(part1(&day5_gen(include_str!("basic.txt"))), 5);
    }

    #[test]
    fn basic2() {
        use crate::day5::*;
        assert_eq!(part2(&day5_gen(include_str!("basic.txt"))), 12);
    }
}
