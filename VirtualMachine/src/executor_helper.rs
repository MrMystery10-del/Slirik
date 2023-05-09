use std::collections::HashMap;

use crate::calculator::{calculate, calculate_condition};
use crate::essentials::{State, Storage};

pub fn add_value<'a, 'b>(state: &'b mut State<'a>, value: &'a String) {
    let num = value.parse::<f64>().unwrap();
    let operator = state.operation.unwrap();

    if let Some(loaded_variable) = &state.loaded_variable {
        if let Some(storage) = search_storage(state, loaded_variable) {
            let current_value = storage.value.parse::<f64>().unwrap();
            let result = calculate(&operator, current_value, num);

            storage.value = result.to_string();
        }
    }
}

pub fn add_value_no_ref(state: &mut State, value: String) {
    let num = value.parse::<f64>().unwrap();
    let operator = state.operation.unwrap();

    if Some(&state.loaded_variable).is_some() {
        if let Some(storage) = search_storage(state, &value) {
            let current_value = storage.value.parse::<f64>().unwrap();
            let result = calculate(&operator, current_value, num);

            storage.value = result.to_string();
        }
    }
}

pub fn check_condition(state: &mut State) -> bool {
    let var1 = state.condition.0.clone().unwrap();

    if var1 == "true" {
        return true;
    }

    let var2 = state.condition.2.clone().unwrap();

    if var2 == "true" {
        return true;
    }

    let num1;
    if var1.chars().next().unwrap().is_numeric() {
        num1 = var1.parse::<f64>().unwrap();
    } else {
        match search_storage(state, var1) {
            Some(storage) => {
                num1 = storage.value.parse::<f64>().unwrap();
            }
            None => panic!("No value found for {}", var1)
        }
    }

    let num2;
    if var2.chars().next().unwrap().is_numeric() {
        num2 = var2.parse::<f64>().unwrap();
    } else {
        match search_storage(state, var2) {
            Some(storage) => {
                num2 = storage.value.parse::<f64>().unwrap();
            }
            None => panic!("No value found for {}", var2)
        }
    }

    calculate_condition(state, num1, num2)
}

pub fn clear_condition(state: &mut State) {
    state.condition = (None, None, None);
}

pub fn insert_variable<'a, 'b>(state: &'b mut State<'a>, value: &'a String) {
    if state.local_variable.len() == 0 {
        state.global_variable.insert(value, Storage {
            value_type: state.variable_type.unwrap(),
            value: String::new(),
        });
    } else {
        let mut map = state.local_variable.get_mut(state.local_variable.len() - 1).unwrap();

        map.insert(value, Storage {
            value_type: state.variable_type.unwrap(),
            value: String::new(),
        });
    }
}

pub fn search_storage<'a, 'b>(state: &'b mut State<'a>, value: &String) -> Option<&'b mut Storage<'a>> {
    let local = state.local_variable.iter_mut().rev().filter_map(|local_layer| {
        local_layer.get_mut(value)
    }).next();

    if local.is_none() {
        return state.global_variable.get_mut(value);
    }

    local
}