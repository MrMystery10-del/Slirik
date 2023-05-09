use std::collections::{HashMap, VecDeque};

use crate::essentials::{State, Statement};
use crate::executor_helper::{add_value, add_value_no_ref, check_condition, clear_condition, insert_variable, search_storage};

pub fn execute<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) -> bool {
    let mut do_skip = false;

    match statement.identifier {
        0x01 => add(state, statement),
        0x02 => block(state, statement),
        0x03 => call(state, statement),
        0x04 => con(state, statement),
        0x05 => cop(state, statement),
        0x06 => dir(state, statement),
        0x07 => end(state, statement),
        0x08 => get(state, statement),
        0x09 => jump(state, statement),
        0xA => load(state, statement),
        0xB => op(state, statement),
        0xC => param(state, statement),
        0xD => return_(state, statement),
        0xE => reva(state, statement),
        0xF => set(state, statement),
        0x10 => do_skip = skip(state, statement),
        0x11 => type_(state, statement),
        0x12 => var(state, statement),
        _ => {}
    }
    do_skip
}

fn add<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {
    match state.loaded_variable {
        Some(loaded_variable) =>
            match state.variable_type {
                Some(variable_type) => {
                    match variable_type.as_str() {
                        "int" | "float" => add_value(state, &statement.value),
                        _ => panic!("The statement 'add' is not allowed for {}", variable_type)
                    }
                }
                None => panic!("No variable type declared")
            }
        None => panic!("No loaded variable")
    }
}

fn block<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {}

fn call<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {}

fn con<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {
    match (state.condition.0.is_none(), state.condition.2.is_none()) {
        (true, _) => state.condition.0 = Some(&statement.value),
        (_, true) => state.condition.2 = Some(&statement.value),
        _ => {
            check_condition(state);
            con(state, statement);
        }
    }
}

fn cop<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {
    if state.condition.1.is_none() {
        state.condition.1 = Some(&statement.value);
    } else {
        panic!("Unexpected condition declaration");
    }
}

fn dir<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {
    if statement.value == "local" {
        state.local_variable.push_back(HashMap::new());
    } else if statement.value == "global" {
        state.local_variable = VecDeque::new();
    }
}

fn end<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {
    clear_condition(state);
}

fn get<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {
    match state.loaded_variable {
        Some(..) => {
            if let Some(storage) = search_storage(state, &statement.value) {
                let value = storage.value.clone();
                add_value_no_ref(state, value);
            }
        }
        None => panic!("No variable is loaded to get: {} {}", statement.identifier, statement.value)
    }
}

fn jump<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {}

fn load<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {
    match search_storage(state, &statement.value) {
        Some(..) => state.loaded_variable = Some(&statement.value),
        None => panic!("Undefined variable: {}", statement.value)
    }
}

fn op<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {
    state.operation = Some(&statement.value);
}

fn param<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {}

fn return_<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {}

fn reva<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {}

fn set<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {
    match state.loaded_variable {
        Some(loaded_variable) => {
            if let Some(storage) = search_storage(state, loaded_variable) {
                storage.value = statement.value.clone();
            } else {
                panic!("No value found")
            }
        }
        None => panic!("Cannot find any loaded variable")
    }
}

fn skip<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) -> bool {
    !check_condition(state)
}

fn type_<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {
    state.variable_type = Some(&statement.value);
}

fn var<'a, 'b>(state: &'b mut State<'a>, statement: &'a Statement) {
    let variable_type = state.variable_type.unwrap();

    match search_storage(state, &statement.value) {
        Some(storage) => {
            storage.value_type = variable_type;
            storage.value = String::new();
        }
        None => insert_variable(state, &statement.value),
    }
}