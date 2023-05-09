use std::collections::VecDeque;
use std::time::Instant;

use crate::essentials::{get_command_map, get_queue_of_statements, get_reader, print_variables, State};
use crate::executor::execute;

mod essentials;
mod executor;
mod executor_helper;
mod calculator;

fn main() {
    let start = Instant::now();
    // ---------------code starts here--------------------------

    // declaring all allowed commands
    let command_map = get_command_map();

    // getting all statements from source code
    let reader = get_reader("Main.sks");
    let mut queue = get_queue_of_statements(reader, command_map);

    // getting default state
    let mut state = get_default_state();

    // variables for managing the pointer
    let mut index = 0;
    let mut end_points = 1;
    let mut skip = false;
    let mut jumps: VecDeque<usize> = VecDeque::new();

    while index < queue.len() {
        let statement = queue.get(index).unwrap();

        if skip {
            if statement.identifier == 0x02 {
                end_points += 1;
            } else if statement.identifier == 0x07 {
                end_points -= 1;

                if end_points == 0 {
                    skip = false;
                }
            }

            index += 1;
            continue;
        }

        match statement.identifier {
            0x02 if statement.value == "NONE" => {
                jumps.push_back(index + 1);
                index += 1;
            }
            0x09 => {
                index = *jumps.back().unwrap();
            }
            0x07 if statement.value == "next" => {
                skip = true;
                index += 1;
            }
            _ => {
                skip = execute(&mut state, statement);
                index += 1;
            }
        }
    }

    // ----------------code ends here---------------------------
    let elapsed = start.elapsed();

    println!("Run time: {}ms", elapsed.as_millis());

    print_variables(state);
}

fn get_default_state() -> State<'static> {
    State {
        class: "".to_string(),
        condition: (None, None, None),
        functions: vec![],
        global_variable: Default::default(),
        loaded_variable: None,
        local_variable: Default::default(),
        operation: None,
        variable_type: None,
    }
}