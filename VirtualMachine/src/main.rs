use std::collections::vec_deque::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use crate::executor::{execute, print_variables, State};
use crate::statement::Statement;

mod executor;
mod statement;
mod calculator;

fn main() {
    let start = Instant::now();

    // Collect the command line arguments for getting the path of the source code
    let _args: Vec<String> = env::args().collect();

    let file_path = "C:\\Users\\Mr.Mystery 1.0\\Desktop\\Slirik\\Main.sks";

    let reader = get_reader(file_path);

    // Parse the statements from the input file into a queue
    let queue = get_queue_of_statements(reader);

    let mut state = State {
        class: String::new(),
        directory: String::new(),
        operation: String::new(),
        variable_type: None,
        loaded_variable: None,
        variables: Vec::new(),
        variable_value: Vec::new(),
    };

    for val in queue {
        execute(&mut state, val);
    }

    let elapsed = start.elapsed();

    println!("Run time: {}", elapsed.as_millis());

    print_variables(state);
}

// Get a buffered reader for a file given its path
fn get_reader(path: &str) -> BufReader<File> {
    let file = File::open(path).expect("Failed to open file");

    return BufReader::new(file);
}

fn get_queue_of_statements(reader: BufReader<File>) -> VecDeque<Statement> {
    let mut queue = VecDeque::<Statement>::new();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        if let Ok(line) = line {
            if let Some((identifier, value)) = line.split_once(' ') {
                let statement = Statement {
                    identifier: identifier.to_string(),
                    value: value.to_string(),
                };

                queue.push_back(statement);
            }
        }
    }

    queue
}