use std::ops::{Add, Div, Mul, Rem, Sub};
use std::string::ToString;
use crate::calculator::calculate;

use crate::Statement;

pub struct State {
    pub class: String,
    pub directory: String,
    pub operation: String,
    pub variable_type: Option<String>,
    pub loaded_variable: Option<String>,
    pub variables: Vec<(String, String, String)>,
    pub variable_value: Vec<(String, Storage)>,
}

pub struct Storage {
    pub value_type: String,
    pub value: String,
}

pub fn execute(mut state: &mut State, statement: Statement) -> &mut State {
    match statement.identifier.as_str() {
        "dir" => state.directory = statement.value,
        "var" => {
            let value = &statement.value;
            if !state.variables.contains(&(state.class.clone(), state.directory.clone(), value.clone())) {
                state.variables.push((state.class.clone(), state.directory.clone(), value.clone()));
                if let Some(variable_type) = &state.variable_type {
                    state.variable_value.push((value.clone(), Storage {
                        value_type: variable_type.clone(),
                        value: String::new(),
                    }))
                }
            } else if let Some(variable_type) = &state.variable_type {
                for storage in state.variable_value.iter_mut().filter(|(var_name, _)| var_name == value).map(|(_, storage)| storage) {
                    storage.value_type = variable_type.clone();
                    storage.value = String::new();
                }
            }
        }
        "type" => state.variable_type = Some(statement.value),
        "load" => {
            if state.variables.contains(&(state.class.clone(), state.directory.clone(), statement.value.clone())) {
                state.loaded_variable = Some(statement.value.clone());
            } else if state.directory == "local" && state.variables.contains(&(state.class.clone(), "global".to_string(), statement.value.clone())) {
                state.loaded_variable = Some(statement.value.clone());
            } else {
                print_error(String::from("ERROR"), format!("Undefined variable name: {}", &statement.value));
            }
        }
        "set" => {
            if let Some(loaded_variable) = &state.loaded_variable {
                for storage in state.variable_value.iter_mut().filter(|(var_name, _)| var_name == loaded_variable).map(|(_, storage)| storage) {
                    storage.value = statement.value.clone();
                }
            }
        }
        "add" => {
            if let Some(variable_type) = &state.variable_type {
                match variable_type.as_str() {
                    "int" | "float" => add_value(state, statement.value),
                    _ => print_error(String::from("WARNING"), format!("The statement 'add' is not allowed for {}", variable_type))
                }
            }
        }
        "get" => {
            match &state.loaded_variable {
                Some(..) => {
                    if let Some(storage) = state.variable_value.iter().find(|(var_name, _)| var_name == &statement.value).map(|(_, storage)| storage) {
                        add_value(state, storage.value.clone());
                    }
                }
                None => print_error(String::from("ERROR"), format!("No variable is loaded to get: {} {}", statement.identifier, statement.value))
            }
        }
        "op" => {
            state.operation = statement.value;
        }
        _ => print_error(String::from("WARNING"), format!("Undefined statement: {}", &statement.identifier)),
    }

    state
}

fn add_value(state: &mut State, value: String) {
    if let Some(loaded_variable) = &state.loaded_variable {
        for storage in state.variable_value.iter_mut().filter(|(var_name, _)| var_name == loaded_variable).map(|(_, storage)| storage) {
            if storage.value_type == "int" {
                let current_value = storage.value.clone().parse::<f64>().unwrap();
                let result = calculate(&state.operation, current_value, value.clone().parse::<f64>().unwrap());
                storage.value = result.to_string();
            } else if storage.value_type == "float" {
                let current_value = storage.value.clone().parse::<f64>().unwrap();
                let result = calculate(&state.operation, current_value, value.clone().parse::<f64>().unwrap());
                storage.value = result.to_string();
            } else {
                print_error(String::from("ERROR"), format!("The variable type '{}' does not support the 'add' statement", storage.value_type));
            }
        }
    }
}

pub fn print_variables(state: State) {
    for val in state.variable_value {
        println!("{} {} {}", val.1.value_type, val.0, val.1.value);
    }
}

fn print_error(level: String, message: String) {
    println!("{} | {}", level, message);
}