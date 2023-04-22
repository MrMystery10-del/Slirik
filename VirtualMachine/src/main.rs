use std::collections::vec_deque::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::executor::{execute, print_variables};
use crate::statement::Statement;

mod executor;
mod statement;

fn main() {
    // Collect the command line arguments for getting the path of the source code
    let _args: Vec<String> = env::args().collect();

    let file_path = "C:\\Users\\Mr.Mystery 1.0\\Desktop\\Slirik\\Main.sks";

    let reader = get_reader(file_path);

    // Parse the statements from the input file into a queue
    let queue = get_queue_of_statements(reader);

    for val in queue {
        unsafe { execute(val); }
    }

    unsafe {
        print_variables();
    }
}

// Get a buffered reader for a file given its path
fn get_reader(path: &str) -> BufReader<File> {
    let file = File::open(path).expect("Failed to open file");

    return BufReader::new(file);
}

// Parse a file of statements into a queue
fn get_queue_of_statements(reader: BufReader<File>) -> VecDeque::<Statement> {
    let mut queue = VecDeque::<Statement>::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        // If the line can be parsed as a string
        if let Ok(line) = line {
            // Split the line into whitespace-separated parts
            let mut iter = line.split_whitespace();

            // Get the first part as the statement identifier
            let identifier = iter.next().unwrap_or("");

            // Get the second part as the statement value
            let value = iter.next().unwrap_or("NONE").to_string();

            // Create a new statement with the identifier and value
            let statement = Statement {
                identifier: identifier.to_string(),
                value: value.to_string(),
            };

            queue.push_back(statement);
        }
    }

    // Return the queue of statements
    return queue;
}