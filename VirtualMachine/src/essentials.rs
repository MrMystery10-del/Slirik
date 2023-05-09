use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Statement {
    pub identifier: u8,
    pub value: String,
}

pub struct State<'a> {
    pub class: String,
    pub condition: (Option<&'a String>, Option<&'a String>, Option<&'a String>),

    // function name, parameters, return_value
    pub functions: Vec<(&'a String, Option<Vec<Storage<'a>>>, Option<Storage<'a>>)>,
    pub global_variable: HashMap<&'a String, Storage<'a>>,
    pub loaded_variable: Option<&'a String>,
    pub local_variable: VecDeque<HashMap<&'a String, Storage<'a>>>,
    pub operation: Option<&'a String>,
    pub variable_type: Option<&'a String>,
}

pub struct Storage<'a> {
    pub value_type: &'a String,
    pub value: String,
}

pub fn get_command_map() -> HashMap<String, u8> {
    let mut command_map: HashMap<String, u8> = HashMap::with_capacity(18);

    command_map.extend([
        ("add", 0x01),
        ("block", 0x02),
        ("call", 0x03),
        ("con", 0x04),
        ("cop", 0x05),
        ("dir", 0x06),
        ("end", 0x07),
        ("get", 0x08),
        ("jump", 0x09),
        ("load", 0xA),
        ("op", 0xB),
        ("param", 0xC),
        ("return", 0xD),
        ("reva", 0xE),
        ("set", 0xF),
        ("skip", 0x10),
        ("type", 0x11),
        ("var", 0x12),
    ].iter().map(|(k, v)| (k.to_string(), *v)));

    command_map
}

pub fn get_reader(path: &str) -> BufReader<File> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Failed to open file");

    let buf_reader = BufReader::new(file);
    buf_reader
}

pub fn get_queue_of_statements(mut reader: BufReader<File>, command_map: HashMap<String, u8>) -> VecDeque<Statement> {
    let capacity_hint = reader.get_ref().metadata().ok().map_or(0, |m| m.len() as usize);
    let mut queue = VecDeque::with_capacity(capacity_hint);
    let mut line = Vec::with_capacity(128);

    while reader.read_until(b'\n', &mut line).unwrap() > 0 {
        let line_str = std::str::from_utf8(&line).expect("Invalid UTF-8");
        if let Some((id, val)) = line_str.split_once(' ') {
            let statement = Statement {
                identifier: *command_map.get(id).unwrap(),
                value: val.trim_end().to_string(),
            };
            queue.push_back(statement);
        } else {
            panic!("Could not read line");
        }
        line.clear();
    }

    queue
}

// Just for debugging
pub fn print_variables(state: State) {
    let mut variable: Option<(&&String, &Storage)> = None;

    println!("-------Global variables-------");
    let mut global_iter = state.global_variable.iter();

    loop {
        variable = global_iter.next();

        if variable.is_none() {
            break;
        }

        println!("{} {} {}", variable.unwrap().1.value_type, variable.unwrap().0, variable.unwrap().1.value);
    }

    println!("-------Local variables-------");
}