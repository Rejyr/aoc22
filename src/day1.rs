use crate::util::input;

fn calories() -> Vec<i32> {
    let mut calories: Vec<i32> = input()
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<i32>().unwrap()).sum())
        .collect();
    calories.sort_unstable();
    calories.reverse();
    calories
}

pub fn solution1() {
    println!("{}", calories()[0]);
}

pub fn solution2() {
    println!("{}", calories().iter().take(3).sum::<i32>());
}
