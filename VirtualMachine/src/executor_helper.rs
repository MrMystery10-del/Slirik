use crate::calculator::calculate;
use crate::executor::State;

pub fn check_condition(state: &mut State) -> bool {
    let var1 = state.condition.0.as_ref().unwrap();

    if var1 == "true" {
        return true;
    }

    let var2 = state.condition.2.as_ref().unwrap();

    if var2 == "true" {
        return true;
    }

    let num1;
    if var1.chars().next().unwrap().is_numeric() {
        num1 = var1.parse::<f64>().unwrap();
    } else {
        match state.variable_value.get(var1) {
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
        match state.variable_value.get(var2) {
            Some(storage) => {
                num2 = storage.value.parse::<f64>().unwrap();
            }
            None => panic!("No value found for {}", var2)
        }
    }

    match state.condition.1.as_ref().unwrap().as_str() {
        "<" => {
            clear_condition(state);
            num1 < num2
        }
        ">" => {
            clear_condition(state);
            num1 > num2
        }
        "==" => {
            clear_condition(state);
            num1 == num2
        }
        _ => panic!("Invalid operator")
    }
}

pub fn clear_condition(state: &mut State) {
    state.condition = (None, None, None);
}

pub fn add_value(state: &mut State, value: String) {
    if let Some(loaded_variable) = &state.loaded_variable {
        if let Some(storage) = state.variable_value.get_mut(loaded_variable) {
            if storage.value_type == "int" {
                let current_value = storage.value.parse::<f64>().unwrap();
                let result = calculate(&state.operation, current_value, value.parse::<f64>().unwrap());
                storage.value = result.to_string();
            } else if storage.value_type == "float" {
                let current_value = storage.value.parse::<f64>().unwrap();
                let result = calculate(&state.operation, current_value, value.parse::<f64>().unwrap());
                storage.value = result.to_string();
            } else {
                panic!("The variable type '{}' does not support the 'add' statement", storage.value_type);
            }
        }
    }
}

pub fn print_warning(message: &str) {
    format!("WARNING | {}", message);
}