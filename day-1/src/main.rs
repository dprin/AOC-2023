use day_macro::{solution, execute_day};

#[allow(unused)]
const TEST_INPUT: &str = r"
asdasd";

#[solution(2023, 1, 1, TEST_INPUT, 0)]
fn something(data: &String) -> u32 {
    0
}

#[solution(2023, 1, 2, TEST_INPUT, 0)]
fn something_else(data: &String) -> u32 {
    0
}

execute_day!(2023, 1);
