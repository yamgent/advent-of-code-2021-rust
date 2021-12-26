use std::io::{self, Write};

const ACTUAL_INPUT: &str = include_str!("input.txt");

struct StdinInputReader;

impl crate::alu::InputReader for StdinInputReader {
    fn read_i64(&mut self) -> i64 {
        loop {
            print!("Program request your input: ");
            io::stdout().flush().expect("We cannot write? GGWP");

            let mut buffer = String::new();
            // if input cannot be read, we can't really do anything
            // so unwrap is fine
            io::stdin()
                .read_line(&mut buffer)
                .expect("We cannot read? GGWP");

            let buffer = buffer.trim();
            match buffer.parse::<i64>() {
                Ok(value) => return value,
                Err(_) => println!("Expecting number, found '{}'. Please retry", buffer),
            }
        }
    }
}

pub fn do_interactive() {
    let mut prog = crate::alu::Program::create(ACTUAL_INPUT);
    prog.set_debug(true);
    prog.execute(&mut crate::alu::Store::new(), &mut StdinInputReader);
}
