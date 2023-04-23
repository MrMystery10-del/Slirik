use std::ops::{Add, Div, Mul, Rem, Sub};
use std::string::ToString;

use crate::Statement;

static mut CLASS: &str = "MAIN";
static mut DIRECTORY: String = String::new();
static mut OPERATION: String = String::new();
static mut VARIABLE_TYPE: String = String::new();
static mut LOADED_VARIABLE: String = String::new();
static mut VARIABLES: Vec<(&str, String, String)> = Vec::new();
// class, dir, var
static mut VARIABLE_VALUE: Vec<(String, Storage)> = Vec::new();

struct Storage {
    value_type: String,
    value: String,
}

pub unsafe fn execute(statement: Statement) {
    match statement.identifier.as_str() {
        "dir" => DIRECTORY = statement.value,
        "var" => {
            let value = statement.value;
            if !VARIABLES.contains(&(CLASS, DIRECTORY.clone(), value.clone())) {
                VARIABLES.push((CLASS, DIRECTORY.clone(), value.clone()));
                VARIABLE_VALUE.push((value.clone(), Storage {
                    value_type: VARIABLE_TYPE.parse().unwrap(),
                    value: "".to_string(),
                }))
            } else {
                for storage in VARIABLE_VALUE.iter_mut().filter(|(var_name, _)| var_name == &value).map(|(_, storage)| storage) {
                    storage.value_type = VARIABLE_TYPE.parse().unwrap();
                    storage.value = "".to_string();
                }
            }
        }
        "type" => VARIABLE_TYPE = statement.value,
        "load" => {
            if VARIABLES.contains(&(CLASS, DIRECTORY.clone(), statement.value.clone())) {
                LOADED_VARIABLE = statement.value;
            } else {
                print_error(String::from("ERROR"), String::from("Undefined variable name"), statement.value);
            }
        }
        "set" => {
            for storage in VARIABLE_VALUE.iter_mut().filter(|(var_name, _)| var_name == &LOADED_VARIABLE).map(|(_, storage)| storage) {
                storage.value = statement.value.clone();
            }
        }
        "add" => {
            match VARIABLE_TYPE.as_str() {
                "int" | "float" => add_value(statement.value),
                _ => print_error(String::from("WARNING"), String::from("The statement 'add' is not allowed for"), VARIABLE_TYPE.clone())
            }
        }
        "get" => {
            if VARIABLES.contains(&(CLASS, DIRECTORY.clone(), statement.value.clone())) {
                for storage in VARIABLE_VALUE.iter_mut().filter(|(var_name, _)| var_name == &statement.value).map(|(_, storage)| storage) {
                    add_value(storage.value.clone());
                }
            } else if (statement.value == "true") || (statement.value == "false") {
                for storage in VARIABLE_VALUE.iter_mut().filter(|(var_name, _)| var_name == &LOADED_VARIABLE).map(|(_, storage)| storage) {
                    storage.value = statement.value.clone();
                }
            } else {
                print_error(String::from("ERROR"), String::from("Undefined variable name"), statement.value.clone());
            }
        }
        "op" => OPERATION = statement.value,
        _ => print_error(String::from("WARNING"), String::from("Undefined statement"), statement.identifier)
    }
}

unsafe fn add_value(value: String) {
    for storage in VARIABLE_VALUE.iter_mut().filter(|(var_name, _)| var_name == &LOADED_VARIABLE).map(|(_, storage)| storage) {
        if storage.value_type == "int" {
            let current_value = storage.value.clone().parse::<f64>().unwrap();
            let result = calculate(current_value, value.clone().parse::<f64>().unwrap());
            storage.value = result.to_string();
        } else if storage.value_type == "float" {
            let current_value = storage.value.clone().parse::<f64>().unwrap();
            let result = calculate(current_value, value.clone().parse::<f64>().unwrap());
            storage.value = result.to_string();
        }
    }
}

unsafe fn calculate(x: f64, y: f64) -> f64 {
    match OPERATION.as_str() {
        "+" => add(x, y),
        "-" => sub(x, y),
        "*" => mul(x, y),
        "/" => div(x, y),
        "^" => pow(x, y),
        "#" => x,
        _ => {
            print_error(String::from("ERROR"), String::from("Unexpected operator"), OPERATION.clone());
            panic!();
        }
    }
}

fn add(x: f64, y: f64) -> f64 {
    return x + y;
}

fn sub(x: f64, y: f64) -> f64 {
    return x - y;
}

fn mul(x: f64, y: f64) -> f64 {
    return x * y;
}

fn div(x: f64, y: f64) -> f64 {
    return x / y;
}

fn pow(x: f64, y: f64) -> f64 {
    let mut result = x;
    let mut index = 1;

    while index < y as i64 {
        result *= x;
        index += 1;
    }
    return result;
}

pub unsafe fn print_variables(){
    for val in &VARIABLE_VALUE {
        println!("{} {} {}", val.1.value_type, val.0, val.1.value);
    }
}

fn print_error(level: String, message: String, fail: String) {
    println!("{} | {}: {}", level, message, fail);
}