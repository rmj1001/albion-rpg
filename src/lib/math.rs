use std::{num::ParseIntError, str::FromStr};

use super::input::{self, prompt_input};

pub enum Operation<T> {
    Add(T),
    Subtract(T),
    Multiply(T),
    Divide(T),
    None,
}

/// Allowed input:
/// +num, -num, *num, /num
pub fn generic_calculator<T>() -> Operation<T>
where
    T: FromStr<Err = std::num::ParseIntError>,
{
    let mut calculation = prompt_input("Enter operation. (Ex. +1, -1, *1, /1)");

    // Remove all whitespace for easier parsing
    calculation = calculation.replace(' ', "").to_string().trim().to_string();

    let chars: Vec<char> = calculation.chars().collect();
    let operator = chars[0];

    match operator {
        '+' => {}
        '-' => {}
        '*' => {}
        '/' => {}
        invalid => {
            input::invalid_input(Some(&invalid.to_string()), Some("+, -, *, or /"), true);
            return Operation::None;
        }
    }

    let number_string: String = String::from_iter(chars[1..].iter());
    let number_result: Result<T, ParseIntError> = number_string.trim().parse();

    if number_result.is_err() {
        input::invalid_input(Some(&number_string), Some("integer"), true);
        return Operation::None;
    }

    let number: T = number_result.unwrap();

    let operation: Operation<T> = match operator {
        '+' => Operation::Add(number),
        '-' => Operation::Subtract(number),
        '*' => Operation::Multiply(number),
        '/' => Operation::Divide(number),
        _ => Operation::None,
    };

    operation
}

pub fn usize_calculator() -> Operation<usize> {
    generic_calculator::<usize>()
}

pub fn isize_calculator() -> Operation<isize> {
    generic_calculator::<isize>()
}
