use crate::essentials::State;
use crate::executor_helper::clear_condition;

pub fn calculate_condition(state: &mut State, num1: f64, num2: f64) -> bool {
    let operator = state.condition.1.as_ref().unwrap().as_str();

    match operator {
        "<" => {
            clear_condition(state);
            num1 < num2
        }
        ">" => {
            clear_condition(state);
            num1 > num2
        }
        "<=" => {
            clear_condition(state);
            num1 <= num2
        }
        ">=" => {
            clear_condition(state);
            num1 <= num2
        }
        "==" => {
            clear_condition(state);
            num1 == num2
        }
        _ => panic!("Invalid operator {}", operator)
    }
}

pub fn calculate(operation: &String, x: f64, y: f64) -> f64 {
    match operation.as_str() {
        "+" => add(x, y),
        "-" => sub(x, y),
        "*" => mul(x, y),
        "/" => div(x, y),
        "%" => rem(x, y),
        "^" => pow(x, y),
        "#" => root(x, y),
        _ => panic!("ERROR | Unexpected operator")
    }
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn sub(x: f64, y: f64) -> f64 {
    x - y
}

fn mul(x: f64, y: f64) -> f64 {
    x * y
}

fn div(x: f64, y: f64) -> f64 {
    x / y
}

fn rem(x: f64, y: f64) -> f64 {
    x % y
}

fn pow(x: f64, y: f64) -> f64 {
    return x.powf(y);
}

fn root(x: f64, y: f64) -> f64 {
    return y.powf(1.0 / x);
}