use std::vec;

use crate::util::input;

pub fn solution1() {
    let input = input();

    let most_calories = input
        .lines()
        .map(|s| s.parse().unwrap_or(0))
        // (max, sum)
        .fold((0, 0), |acc, x| {
            if x == 0 {
                return (std::cmp::max(acc.1, acc.0), 0);
            }

            (acc.0, acc.1 + x)
        })
        .0;
    println!("{most_calories}")
}

pub fn solution2() {
    let input = input();
    let mut calories = vec![];

    let last = input
        .lines()
        .map(|s| s.parse().unwrap_or(0))
        .fold(0, |acc, x| {
            if x == 0 {
                calories.push(acc);
                0
            } else {
                acc + x
            }
        });
    calories.push(last);
    calories.sort_unstable();
    let calories: i32 = calories.iter().rev().take(3).sum();
    println!("{calories}");
}
