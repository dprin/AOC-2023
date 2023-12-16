use day_macro::{solution, execute_day};

#[allow(unused)]
const TEST_INPUT: &str = r"
asdasd";

#[solution(Year = 2023, Day = 1, Part = 1, Test = TEST_INPUT, Answer = 0)]
fn something(data: &String) -> u32 {
    0
}

#[solution(Year = 2023, Day = 1, Part = 2, Test = TEST_INPUT, Answer = 0)]
fn something_else(data: &String) -> u32 {
    0
}

execute_day!(2023, 1);
