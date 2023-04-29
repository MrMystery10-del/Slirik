use std::collections::HashMap;
use std::string::ToString;

use crate::calculator::calculate;
use crate::executor_helper::{add_value, check_condition, clear_condition, print_warning};
use crate::Statement;

pub struct State {
    pub class: String,
    pub directory: String,
    pub operation: String,
    pub variable_type: Option<String>,
    pub loaded_variable: Option<String>,
    pub condition: (Option<String>, Option<String>, Option<String>),
    pub variables: Vec<(String, String, String)>,
    pub variable_value: HashMap<String, Storage>,
}

pub struct Storage {
    pub value_type: String,
    pub value: String,
}

pub fn execute(mut state: &mut State, statement: Statement) -> bool {
    let mut doSkip = false;
    match statement.identifier.as_str() {
        "add" => add(state, statement),
        "con" => con(state, statement),
        "cop" => cop(state, statement),
        "dir" => dir(state, statement),
        "end" => end(state),
        "get" => get(state, statement),
        "load" => load(state, statement),
        "op" => op(state, statement),
        "set" => set(state, statement),
        "skip" => doSkip = skip(state),
        "type" => setType(state, statement),
        "var" => var(state, statement),
        _ => print_warning(format!("Undefined statement: {}", statement.identifier).as_str()),
    }
    doSkip
}

fn add(state: &mut State, statement: Statement) {
    match &state.variable_type {
        Some(variable_type) =>
            match variable_type.as_str() {
            "int" | "float" => add_value(state, statement.value),
            _ => panic!("The statement 'add' is not allowed for {}", variable_type)
        }
        None => panic!("No variable type declared")
    }
}

fn con(state: &mut State, statement: Statement) {
    match (state.condition.0.is_none(), state.condition.2.is_none()) {
        (true, _) => state.condition.0 = Some(statement.value),
        (_, true) => state.condition.2 = Some(statement.value),
        _ => {
            check_condition(state);
            con(state, statement);
        }
    }
}

fn cop(state: &mut State, statement: Statement) {
    if state.condition.1.is_none() {
        state.condition.1 = Some(statement.value);
    } else {
        panic!("Unexpected condition declaration");
    }
}

fn dir(state: &mut State, statement: Statement) {
    state.directory = statement.value
}

fn end(state: &mut State) {
    clear_condition(state)
}

fn get(state: &mut State, statement: Statement) {
    match &state.loaded_variable {
        Some(..) => {
            if let Some(storage) = state.variable_value.get(&statement.value) {
                add_value(state, storage.value.clone());
            }
        }
        None => panic!("No variable is loaded to get: {} {}", statement.identifier, statement.value)
    }
}

fn load(state: &mut State, statement: Statement) {
    if state.variables.contains(&(state.class.clone(), state.directory.clone(), statement.value.clone())) {
        state.loaded_variable = Some(statement.value.clone());
    } else if state.directory == "local" && state.variables.contains(&(state.class.clone(), "global".to_string(), statement.value.clone())) {
        state.loaded_variable = Some(statement.value.clone());
    } else {
        panic!("Undefined variable name: {}", statement.value);
    }
}

fn op(state: &mut State, statement: Statement) {
    state.operation = statement.value
}

fn set(state: &mut State, statement: Statement) {
    match &state.loaded_variable {
        Some(loaded_variable) => {
            if let Some(storage) = state.variable_value.get_mut(loaded_variable) {
                storage.value = statement.value;
            } else {
                panic!("No value found for {}", loaded_variable);
            }
        }
        None => panic!("Cannot find any loaded variable")
    }
}

fn skip(state: &mut State) -> bool {
    !check_condition(state)
}

fn setType(state: &mut State, statement: Statement) {
    state.variable_type = Some(statement.value);
}

fn var(state: &mut State, statement: Statement) {
    let value = &statement.value;
    if !state.variables.contains(&(state.class.clone(), state.directory.clone(), value.clone())) {
        state.variables.push((state.class.clone(), state.directory.clone(), value.clone()));
        if let Some(variable_type) = &state.variable_type {
            state.variable_value.insert(value.clone(), Storage {
                value_type: variable_type.clone(),
                value: String::new(),
            });
        }
    } else if let Some(variable_type) = &state.variable_type {
        if let Some(storage) = state.variable_value.get_mut(value) {
            storage.value_type = variable_type.clone();
            storage.value = String::new();
        }
    }
}

pub fn print_variables(state: State) {
    for val in state.variable_value {
        println!("{} {} {}", val.1.value_type, val.0, val.1.value);
    }
}