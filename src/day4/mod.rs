use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct BingoBoard {
    pub squares: Vec<Square>,
    pub observed: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Square {
    number: u8,
    called: bool,
}

impl BingoBoard {
    fn number_called(&mut self, called: u8) {
        for mut square in self.squares.iter_mut() {
            if square.number == called {
                square.called = true;
            }
        }
    }

    fn check_for_win(&mut self) -> bool {
        for mut row in &self.squares.iter().chunks(5) {
            if row.all(|square| square.called) {
                self.observed = true;
                return true;
            }
        }
        for col_offset in 0..5 {
            if self
                .squares
                .iter()
                .skip(col_offset)
                .step_by(5)
                .all(|square| square.called)
            {
                self.observed = true;
                return true;
            }
        }

        false
    }

    fn not_selected(&self) -> impl Iterator<Item = u8> + '_ {
        self.squares
            .iter()
            .filter(|square| !square.called)
            .map(|square| square.number)
    }
}

#[aoc_generator(day4)]
pub fn day4_gen(input: &str) -> (Vec<u8>, Vec<BingoBoard>) {
    let mut lines = input.lines();
    let calls = lines
        .next()
        .unwrap()
        .split(',')
        .flat_map(|num| num.parse())
        .collect();

    let mut boards = Vec::new();

    lines.next(); //blank
    for chunk in &lines.chunks(6) {
        // 5 lines for the boards, 1 spacer

        boards.push(BingoBoard {
            squares: chunk
                .flat_map(|c| c.lines())
                .flat_map(str::split_ascii_whitespace)
                .flat_map(str::parse::<u8>)
                .map(|number| Square {
                    number,
                    called: false,
                })
                .collect(),
            observed: false,
        })
    }

    (calls, boards)
}

#[aoc(day4, part1)]
pub fn part1(input: &(Vec<u8>, Vec<BingoBoard>)) -> usize {
    let mut boards = input.1.clone();
    let mut calls = input.0.iter();
    let (winner, called) = 'outer: loop {
        let &call = calls.next().expect("failed to converge");
        for board in boards.iter_mut() {
            board.number_called(call);
            if board.check_for_win() {
                break 'outer (board, call);
            }
        }
    };

    winner.not_selected().fold(0usize, |a, n| a + n as usize) * called as usize
}

#[aoc(day4, part2)]
pub fn part2(input: &(Vec<u8>, Vec<BingoBoard>)) -> usize {
    let mut boards = input.1.clone();
    let mut last_winner = None;
    for &call in &input.0 {
        for board in boards.iter_mut() {
            board.number_called(call);
            if !board.observed && board.check_for_win() {
                last_winner =
                    Some(board.not_selected().fold(0usize, |a, n| a + n as usize) * call as usize);
            }
        }
    }

    last_winner.unwrap()
}

mod test {

    #[test]
    fn gen() {
        use crate::day4::day4_gen;
        let input = day4_gen(include_str!("basic.txt"));
        assert!(input.0.len() > 0);
        assert!(input.1.len() > 0);
    }

    #[test]
    fn basic() {
        use crate::day4::*;
        assert_eq!(part1(&day4_gen(include_str!("basic.txt"))), 4512);
    }
    #[test]
    fn part2() {
        use crate::day4::*;
        assert_eq!(part2(&day4_gen(include_str!("basic.txt"))), 1924);
    }
}
