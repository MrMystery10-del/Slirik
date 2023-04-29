use std::collections::HashMap;
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
mod executor_helper;

fn main() {
    let start = Instant::now();

    // Collect the command line arguments for getting the path of the source code
    let _args: Vec<String> = env::args().collect();

    let file_path = "C:\\Users\\Mr.Mystery 1.0\\Desktop\\Slirik\\Main.sks";

    let reader = get_reader(file_path);

    // Parse the statements from the input file into a queue
    let queue = Vec::from(get_queue_of_statements(reader));

    let mut state = State {
        class: String::new(),
        directory: String::new(),
        operation: String::new(),
        variable_type: None,
        loaded_variable: None,
        condition: (None, None, None),
        variables: Vec::new(),
        variable_value: HashMap::new(),
    };

    let mut skip = false;
    let mut index = 0;
    let mut endPoint = 0;
    let mut blocks: VecDeque<usize> = VecDeque::new();
    loop {
        if index >= queue.len() {
            break;
        }

        let mut statement: Statement = queue.get(index).into();

        if !skip {
            if statement.identifier == "block" && statement.value == "NONE" {
                blocks.push_back(index + 1);
                endPoint += 1;
                index += 1;
                continue;
            } else if statement.identifier == "jump" && statement.value == "NONE" {
                index = *blocks.back().unwrap();
                continue;
            } else if statement.identifier == "end" {
                index += 1;
                continue;
            }

            skip = execute(&mut state, statement);
            index += 1;
        } else {
            if statement.identifier == "end" {
                if endPoint == 1 {
                    skip = true;
                }
                endPoint -= 1;
            }
            index += 1;
            continue;
        }
    }

    let elapsed = start.elapsed();

    println!("Run time: {}ms", elapsed.as_millis());

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
            let mut identifier = String::new();
            let mut value = String::new();
            if let Some((id, val)) = line.split_once(' ') {
                identifier = id.to_string();
                value = val.to_string();
            } else if !line.trim().is_empty() {
                identifier = line.trim().to_string();
                value = "NONE".to_string();
            }
            let statement = Statement {
                identifier,
                value,
            };
            queue.push_back(statement);
        }
    }

    queue
}