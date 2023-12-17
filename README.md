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

An example usage of the solution macro is as follows:
```rust
// Year = 2023
// Day = 1
// Part = 2
// Test = TEST_INPUT
// Answer = 123
#[solution(2023, 1, 2, TEST_INPUT, 123)]
fn something(data: &String) -> u32 {
    unimplemented!()
}
```

# Execute Macro
The execute macro generates a main function so that the functions that were inputted into the solution macro can be executed (along with their output being shown).

It takes input:
- Year: year of the problem to execute
- Day: day of the problem to execute

An example usage of the execute macro is:
```rust
execute_day!(2023, 1);
```
