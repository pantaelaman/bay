use log::info;

pub fn fetch(args: Vec<Value>) {
    info!(target: "bay", "Fetching data");
}

pub fn traverse(args: Vec<Value>) {
    info!(target: "bay", "Traversing tree");
}

pub fn run(args: Vec<Value>) {
    info!(target: "bay", "Running a command");
}

#[derive(Debug)]
pub struct Command {
    command: fn (Vec<Value>),
    args: Vec<Value>
}

impl Command {
    pub fn new(command: fn (Vec<Value>), args: Vec<Value>) -> Self {
        Command {command, args}
    }
}

#[derive(Debug)]
pub enum Value {
    FPATH(String),
    URL(String),
    COMMAND(Command),
    STRING(String),
    NUM(usize),
    FLAG(String),
    BLOCK(Vec<Command>),
}

