use std::collections::HashMap;
use std::string::ToString;

use crate::calculator::calculate;
use crate::executor_helper::{add_value, check_condition, clear_condition, print_warning};
use crate::Statement;

pub struct State {
    pub class: String,
    pub condition: (Option<String>, Option<String>, Option<String>),
    pub directory: String,

    // function name, parameters, return_value
    pub functions: Vec<(String, Option<Vec<Storage>>, Option<Storage>)>,
    pub loaded_variable: Option<String>,
    pub operation: String,
    pub variable_type: Option<String>,
    pub variable_value: HashMap<String, Storage>,
    pub variables: Vec<(String, String, String)>,
}

pub struct Storage {
    pub value_type: String,
    pub value: String,
}

// Execute a function based on provided statement
pub fn execute(mut state: &mut State, statement: Statement) -> bool {
    let mut doSkip = false;
    match statement.identifier.as_str() {
        "add" => add(state, statement),
        "call" => call(state, statement),
        "con" => con(state, statement),
        "cop" => cop(state, statement),
        "dir" => dir(state, statement),
        "end" => end(state),
        "get" => get(state, statement),
        "load" => load(state, statement),
        "op" => op(state, statement),
        "param" => param(state, statement),
        "return" => return_(state, statement),
        "reva" => reva(state, statement),
        "set" => set(state, statement),
        "skip" => doSkip = skip(state),
        "type" => setType(state, statement),
        "var" => var(state, statement),
        _ => print_warning(format!("Undefined statement: {}", statement.identifier).as_str()),
    }
    doSkip
}

// Add a value to loaded value with current loaded operator
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

// Calls a function and returns a value from that function
fn call(state: &mut State, statement: Statement){}

// Sets a new condition point, that can be a number or boolean value
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

// Sets the condition operator, that can be: '<', '>' or '=='
fn cop(state: &mut State, statement: Statement) {
    if state.condition.1.is_none() {
        state.condition.1 = Some(statement.value);
    } else {
        panic!("Unexpected condition declaration");
    }
}

// Sets the current directory (unused before function implementation)
fn dir(state: &mut State, statement: Statement) {
    state.directory = statement.value
}

// Clears the current condition after out of scout
fn end(state: &mut State) {
    clear_condition(state)
}

// Gets a value from variable
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

// Loads a variable
fn load(state: &mut State, statement: Statement) {
    if state.variables.contains(&(state.class.clone(), state.directory.clone(), statement.value.clone())) {
        state.loaded_variable = Some(statement.value.clone());
    } else if state.directory == "local" && state.variables.contains(&(state.class.clone(), "global".to_string(), statement.value.clone())) {
        state.loaded_variable = Some(statement.value.clone());
    } else {
        panic!("Undefined variable name: {}", statement.value);
    }
}

// Loads a new operant which will be used on next value adding
fn op(state: &mut State, statement: Statement) {
    state.operation = statement.value
}

// Adds a new parameter for function which will be required on function call
fn param(state: &mut State, statement: Statement){}

// Returns a value to last caller
fn return_(state: &mut State, statement: Statement){}

// Sets the return type of a function
fn reva(state: &mut State, statement: Statement){}

// Sets a new value to loaded value
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

// Check if condition is true or false, based on that returns a boolean value to decide if should
// skip until next end statement
fn skip(state: &mut State) -> bool {
    !check_condition(state)
}

// Sets the type of variables created after
fn setType(state: &mut State, statement: Statement) {
    state.variable_type = Some(statement.value);
}

// Creates a new variable
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

// Prints all variables with their values (just for testing)
pub fn print_variables(state: State) {
    for val in state.variable_value {
        println!("{} {} {}", val.1.value_type, val.0, val.1.value);
    }
}