use crate::util::input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl<'a> From<&'a str> for RPS {
    fn from(s: &'a str) -> Self {
        match s {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => unreachable!(),
        }
    }
}

fn parse() -> Vec<(RPS, RPS)> {
    input()
        .lines()
        .map(|s| {
            let mut s = s.split_whitespace();
            (s.next().unwrap().into(), s.next().unwrap().into())
        })
        .collect()
}

fn score1((a, you): (RPS, RPS)) -> i32 {
    (match (you, a) {
        (RPS::Rock, RPS::Rock) => 3,
        (RPS::Rock, RPS::Paper) => 0,
        (RPS::Rock, RPS::Scissors) => 6,
        (RPS::Paper, RPS::Rock) => 6,
        (RPS::Paper, RPS::Paper) => 3,
        (RPS::Paper, RPS::Scissors) => 0,
        (RPS::Scissors, RPS::Rock) => 0,
        (RPS::Scissors, RPS::Paper) => 6,
        (RPS::Scissors, RPS::Scissors) => 3,
    }) + you as i32
}

pub fn solution1() {
    let score: i32 = parse().into_iter().map(score1).sum();
    println!("{score}")
}

fn score2((a, you): (RPS, RPS)) -> i32 {
    // X (Rock): lose
    // Y (Paper): draw
    // Z (Scissors): win
    let you = match (you, a) {
        (RPS::Rock, RPS::Rock) => RPS::Scissors,
        (RPS::Rock, RPS::Paper) => RPS::Rock,
        (RPS::Rock, RPS::Scissors) => RPS::Paper,
        (RPS::Paper, RPS::Rock) => RPS::Rock,
        (RPS::Paper, RPS::Paper) => RPS::Paper,
        (RPS::Paper, RPS::Scissors) => RPS::Scissors,
        (RPS::Scissors, RPS::Rock) => RPS::Paper,
        (RPS::Scissors, RPS::Paper) => RPS::Scissors,
        (RPS::Scissors, RPS::Scissors) => RPS::Rock,
    };
    score1((a, you))
}

pub fn solution2() {
    let score: i32 = parse().into_iter().map(score2).sum();
    println!("{score}")
}
