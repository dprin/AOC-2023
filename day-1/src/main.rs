use day_macro::{solution, execute_day};

#[allow(unused)]
const PART_1: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

#[solution(2023, 1, 1, PART_1, 142)]
fn something(data: &String) -> u32 {
    data.lines()
        .map(|line| {
            let mut nums = line.chars().filter_map(|char| char.to_digit(10));

            let first = nums.next().unwrap();
            let last = nums.last().unwrap_or(first);
            
            (first * 10) + last
        }).sum()
}

execute_day!(2023, 1);
