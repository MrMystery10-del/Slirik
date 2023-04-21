use std::collections::vec_deque::VecDeque;
use std::io::BufRead;
use std::env;
use std::fs;

struct Statement {
    identifier: String,
    value: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = "C:\\Users\\Mr.Mystery 1.0\\Desktop\\Slirik\\Main.sks";

    let file = fs::File::open(filename).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);

    let mut queue = VecDeque::<Statement>::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut iter = line.split_whitespace();
            let identifier = iter.next().unwrap_or("");
            let value = iter.next().unwrap_or("NONE").to_string();
            let statement = Statement {
                identifier: identifier.to_string(),
                value,
            };
            queue.push_back(statement);
        }
    }

    for val in queue {
        println!("{} {}", val.identifier, val.value);
    }
}