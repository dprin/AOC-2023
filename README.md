# Advent Of Code 2023
For now this is Advent Of Code 2023, but I would like to expand it to work with any year.

The whole purpose of this project is to let me learn rust more and specifically macros, as they are usually a pain to work with.

# Solution Macro
The solution macro is placed on top of the function that is a solution to a problem and it takes the following data:

- Day: day of the problem
- Year: year of the problem
- Part: since there are two parts in a day normally
- Test: the test input that is given on the website, as a `&str`
- Answer: the answer to the test input

A example usage of the program is as follows:
```rust
#[solution(Year = 2023, Day = 1, Part = 1, Test = TEST_INPUT, Answer = 123)]
fn something(data: &String) -> u32 {
    unimplemented!()
}
```
