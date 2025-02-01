use std::env;

pub struct Cli {
    pub args: Vec<String>,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            args: env::args().collect(),
        }
    }

    pub fn get_arg(&self, index: usize) -> Option<&String> {
        self.args.get(index)
    }
}