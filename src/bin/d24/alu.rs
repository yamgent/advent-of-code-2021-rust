use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Register::W => "w",
                Register::X => "x",
                Register::Y => "y",
                Register::Z => "z",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Placeholder {
    Value(i64),
    Register(Register),
}

impl Display for Placeholder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Placeholder::Value(val) => val.to_string(),
                Placeholder::Register(reg) => reg.to_string(),
            }
        )
    }
}

impl Placeholder {
    fn get_value(&self, store: &Store) -> i64 {
        match self {
            Placeholder::Value(value) => *value,
            Placeholder::Register(register) => store.get(*register),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Input(Register),
    Add(Register, Placeholder),
    Mul(Register, Placeholder),
    Div(Register, Placeholder),
    Mod(Register, Placeholder),
    Eql(Register, Placeholder),
}

impl Instruction {
    fn parse_instruction(line: &str) -> Self {
        let parts = line.trim().split(' ').collect::<Vec<_>>();

        let dest = match parts[1] {
            "w" => Register::W,
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            _ => panic!("Cannot have {} in ths position", parts[1]),
        };

        let src = if parts.len() > 2 {
            Some(match parts[2] {
                "w" => Placeholder::Register(Register::W),
                "x" => Placeholder::Register(Register::X),
                "y" => Placeholder::Register(Register::Y),
                "z" => Placeholder::Register(Register::Z),
                _ => Placeholder::Value(parts[2].parse::<i64>().unwrap()),
            })
        } else {
            None
        };

        match parts[0] {
            "inp" => Instruction::Input(dest),
            "add" => Instruction::Add(dest, src.unwrap()),
            "mul" => Instruction::Mul(dest, src.unwrap()),
            "div" => Instruction::Div(dest, src.unwrap()),
            "mod" => Instruction::Mod(dest, src.unwrap()),
            "eql" => Instruction::Eql(dest, src.unwrap()),
            _ => panic!("Unknown instruction {}", parts[0]),
        }
    }

    fn parse_instructions(input: &str) -> Vec<Self> {
        input.trim().lines().map(Self::parse_instruction).collect()
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Input(dest) => write!(f, "{} = input()", dest.to_string()),
            Instruction::Add(dest, src) => write!(f, "{} += {}", dest.to_string(), src.to_string()),
            Instruction::Mul(dest, src) => write!(f, "{} *= {}", dest.to_string(), src.to_string()),
            Instruction::Div(dest, src) => write!(f, "{} /= {}", dest.to_string(), src.to_string()),
            Instruction::Mod(dest, src) => write!(f, "{} %= {}", dest.to_string(), src.to_string()),
            Instruction::Eql(dest, src) => write!(
                f,
                "{} = if {} == {} {{ 1 }} else {{ 0 }}",
                dest.to_string(),
                dest.to_string(),
                src.to_string()
            ),
        }
    }
}

pub trait InputReader {
    fn read_i64(&mut self) -> i64;
}

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<Instruction>,
    debug: bool,
}

impl Program {
    pub fn create(input: &str) -> Self {
        Program {
            instructions: Instruction::parse_instructions(input),
            debug: false,
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn execute(&self, store: &mut Store, input_reader: &mut impl InputReader) {
        // todo!()
        self.instructions
            .iter()
            .enumerate()
            .for_each(|(index, instruction)| {
                match instruction {
                    Instruction::Input(dest) => {
                        let result = input_reader.read_i64();
                        store.set(*dest, result);
                    }
                    Instruction::Add(dest, src) => {
                        let a = store.get(*dest);
                        let b = src.get_value(store);
                        store.set(*dest, a + b);
                    }
                    Instruction::Mul(dest, src) => {
                        let a = store.get(*dest);
                        let b = src.get_value(store);
                        store.set(*dest, a * b);
                    }
                    Instruction::Div(dest, src) => {
                        let a = store.get(*dest);
                        let b = src.get_value(store);
                        store.set(*dest, a / b);
                    }
                    Instruction::Mod(dest, src) => {
                        let a = store.get(*dest);
                        let b = src.get_value(store);
                        store.set(*dest, a % b);
                    }
                    Instruction::Eql(dest, src) => {
                        let a = store.get(*dest);
                        let b = src.get_value(store);
                        store.set(*dest, if a == b { 1 } else { 0 });
                    }
                }

                if self.debug {
                    println!(
                        "[{}] w:{}, x:{}, y:{}, z:{} ({})",
                        index + 1,
                        store.get(Register::W),
                        store.get(Register::X),
                        store.get(Register::Y),
                        store.get(Register::Z),
                        instruction
                    );
                }
            });
    }
}

#[derive(Debug, PartialEq)]
pub struct Store {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Store {
    pub fn new() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn get(&self, register: Register) -> i64 {
        match register {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
        }
    }

    fn set(&mut self, register: Register, value: i64) {
        match register {
            Register::W => self.w = value,
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::Z => self.z = value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            Instruction::parse_instruction("inp w"),
            Instruction::Input(Register::W)
        );
        assert_eq!(
            Instruction::parse_instruction("add x w"),
            Instruction::Add(Register::X, Placeholder::Register(Register::W))
        );
        assert_eq!(
            Instruction::parse_instruction("mul y x"),
            Instruction::Mul(Register::Y, Placeholder::Register(Register::X))
        );
        assert_eq!(
            Instruction::parse_instruction("div z y"),
            Instruction::Div(Register::Z, Placeholder::Register(Register::Y))
        );
        assert_eq!(
            Instruction::parse_instruction("mod y z"),
            Instruction::Mod(Register::Y, Placeholder::Register(Register::Z))
        );
        assert_eq!(
            Instruction::parse_instruction("eql w 0"),
            Instruction::Eql(Register::W, Placeholder::Value(0))
        );
        assert_eq!(
            Instruction::parse_instruction("eql w 123"),
            Instruction::Eql(Register::W, Placeholder::Value(123))
        );
        assert_eq!(
            Instruction::parse_instruction("eql w -123"),
            Instruction::Eql(Register::W, Placeholder::Value(-123))
        );
    }

    #[test]
    fn test_parse_instructions() {
        assert_eq!(
            Instruction::parse_instructions(
                r"
inp w
add x w
mul y x
div z y
mod y z
eql w 123
"
            ),
            [
                Instruction::Input(Register::W),
                Instruction::Add(Register::X, Placeholder::Register(Register::W)),
                Instruction::Mul(Register::Y, Placeholder::Register(Register::X)),
                Instruction::Div(Register::Z, Placeholder::Register(Register::Y)),
                Instruction::Mod(Register::Y, Placeholder::Register(Register::Z)),
                Instruction::Eql(Register::W, Placeholder::Value(123))
            ]
        );
    }

    #[test]
    fn test_program_create() {
        assert_eq!(
            Program::create(
                r"
inp w
add x w
mul y x
div z y
mod y z
eql w 123
"
            ),
            Program {
                instructions: vec![
                    Instruction::Input(Register::W),
                    Instruction::Add(Register::X, Placeholder::Register(Register::W)),
                    Instruction::Mul(Register::Y, Placeholder::Register(Register::X)),
                    Instruction::Div(Register::Z, Placeholder::Register(Register::Y)),
                    Instruction::Mod(Register::Y, Placeholder::Register(Register::Z)),
                    Instruction::Eql(Register::W, Placeholder::Value(123))
                ],
                debug: false
            }
        );
    }

    #[test]
    fn test_program_set_debug() {
        let mut prog = Program::create("inp w");

        prog.set_debug(true);
        assert!(prog.debug);

        prog.set_debug(false);
        assert!(!prog.debug);
    }

    #[test]
    fn test_store_new_get_set() {
        let mut store = Store::new();

        assert_eq!(
            store,
            Store {
                w: 0,
                x: 0,
                y: 0,
                z: 0
            }
        );

        store.set(Register::W, 1);
        store.set(Register::X, 2);
        store.set(Register::Y, 3);
        store.set(Register::Z, 4);

        assert_eq!(
            store,
            Store {
                w: 1,
                x: 2,
                y: 3,
                z: 4
            }
        );

        assert_eq!(store.get(Register::W), 1);
        assert_eq!(store.get(Register::X), 2);
        assert_eq!(store.get(Register::Y), 3);
        assert_eq!(store.get(Register::Z), 4);
    }

    #[test]
    fn test_placeholder_get_value() {
        let store = Store {
            w: 2,
            x: 3,
            y: 4,
            z: 5,
        };

        assert_eq!(Placeholder::Value(1).get_value(&store), 1);
        assert_eq!(Placeholder::Register(Register::W).get_value(&store), 2);
        assert_eq!(Placeholder::Register(Register::X).get_value(&store), 3);
        assert_eq!(Placeholder::Register(Register::Y).get_value(&store), 4);
        assert_eq!(Placeholder::Register(Register::Z).get_value(&store), 5);
    }

    struct TestInputReader {
        values: Vec<i64>,
        current: usize,
    }

    impl InputReader for TestInputReader {
        fn read_i64(&mut self) -> i64 {
            if self.current >= self.values.len() {
                panic!("Too many inputs expected.");
            }

            let val = self.values[self.current];
            self.current += 1;
            val
        }
    }

    #[test]
    fn test_program_execute() {
        fn run(prog: &str, reader_values: Vec<i64>, output_register: Register) -> i64 {
            let prog = Program::create(prog);
            let mut store = Store::new();
            let mut input_reader = TestInputReader {
                values: reader_values,
                current: 0,
            };
            prog.execute(&mut store, &mut input_reader);
            store.get(output_register)
        }

        fn run_with_all(prog: &str, reader_values: Vec<i64>) -> Store {
            let prog = Program::create(prog);
            let mut store = Store::new();
            let mut input_reader = TestInputReader {
                values: reader_values,
                current: 0,
            };
            prog.execute(&mut store, &mut input_reader);
            store
        }

        let neg_prog = r"
inp x
mul x -1
";
        assert_eq!(run(neg_prog, vec![2], Register::X), -2);
        assert_eq!(run(neg_prog, vec![-4], Register::X), 4);
        assert_eq!(run(neg_prog, vec![0], Register::X), 0);

        let three_times_larger_prog = r"
inp z
inp x
mul z 3
eql z x
";
        assert_eq!(run(three_times_larger_prog, vec![3, 8], Register::Z), 0);
        assert_eq!(run(three_times_larger_prog, vec![3, 9], Register::Z), 1);
        assert_eq!(run(three_times_larger_prog, vec![3, 10], Register::Z), 0);

        let to_binary_prog = r"
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2
";
        assert_eq!(
            run_with_all(to_binary_prog, vec![1]),
            Store {
                w: 0,
                x: 0,
                y: 0,
                z: 1
            }
        );
        assert_eq!(
            run_with_all(to_binary_prog, vec![2]),
            Store {
                w: 0,
                x: 0,
                y: 1,
                z: 0
            }
        );
        assert_eq!(
            run_with_all(to_binary_prog, vec![4]),
            Store {
                w: 0,
                x: 1,
                y: 0,
                z: 0
            }
        );
        assert_eq!(
            run_with_all(to_binary_prog, vec![8]),
            Store {
                w: 1,
                x: 0,
                y: 0,
                z: 0
            }
        );
        assert_eq!(
            run_with_all(to_binary_prog, vec![16]),
            Store {
                w: 0,
                x: 0,
                y: 0,
                z: 0
            }
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(Register::W.to_string(), "w");
        assert_eq!(Register::X.to_string(), "x");
        assert_eq!(Register::Y.to_string(), "y");
        assert_eq!(Register::Z.to_string(), "z");

        assert_eq!(Placeholder::Value(-1).to_string(), "-1");
        assert_eq!(Placeholder::Register(Register::X).to_string(), "x");

        assert_eq!(
            Instruction::parse_instructions(
                r"
inp w
add x w
mul y x
div z y
mod y z
eql w 123
"
            )
            .into_iter()
            .map(|inst| inst.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
            r"w = input()
x += w
y *= x
z /= y
y %= z
w = if w == 123 { 1 } else { 0 }"
        );
    }
}
