use std::fs::File;
use std::io::prelude::*;
use std::env::args;
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;
use std::fmt;
use std::convert::TryFrom;

enum OpCode {
    NOP,
    ACC,
    JMP,
}

struct Operation {
    op: OpCode,
    val: i32,
}

#[derive(Debug)]
struct ParseOpError {
    msg: String,
}

impl From<&str> for ParseOpError {
    fn from(s: &str) -> Self {
        ParseOpError {
            msg: String::from(s),
        }
    }
}

impl fmt::Display for ParseOpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse operation: {}", self.msg)
    }
}

impl Error for ParseOpError {}

impl FromStr for Operation {
    type Err = ParseOpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let op = match parts.next() {
            Some(s) => Ok(s),
            None => Err("Failed to split operation into '<op> <val>'"),
        }?;
        let val = match parts
            .next()
            .map(|val| val.parse::<i32>().ok())
            .flatten() {
                Some(val) => Ok(val),
                None => Err("Failed to parse valur"),
            }?;
        let op = match op {
            "nop" => Ok(OpCode::NOP),
            "acc" => Ok(OpCode::ACC),
            "jmp" => Ok(OpCode::JMP),
            _ => Err("Failed to parse op"),
        }?;

        Ok(Self {
            op,
            val,
        })
    }
}

fn main() -> std::io::Result<()> {
    let arg: Option<String> = args().skip(1).next();
    if arg.is_none() {
        println!("Missing arg");
        return Ok(());
    }
    let file_path = arg.unwrap();
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let ops: Vec<Operation> = buffer
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.parse::<Operation>().unwrap())
        .collect();
    let expected_jmp_exit = i32::try_from(ops.len()).unwrap();

    println!("Expected jmp exit: {}", expected_jmp_exit);

    for i in 0..ops.len() {
        println!("Trying op at index {}", i);
        let should_continue = match ops.get(i) {
            None => true,
            Some(op) => match op.op {
                OpCode::JMP => false,
                OpCode::NOP => false,
                _ => true,
            }
        };

        if should_continue {
            println!("Skipping op at index {}", i);
            continue;
        }

        let mut seen_operation_indices = HashSet::new();
        let mut current_op: i32 = 0;
        let mut accumulator: i32 = 0;
        let converted_changed_instruction = i32::try_from(i).unwrap();

        let mut completed_successfully = false;

        while !seen_operation_indices.contains(&current_op) {
            seen_operation_indices.insert(current_op.clone());

            match ops.get(usize::try_from(current_op).unwrap()) {
                Some(operation) => match operation.op {
                    OpCode::NOP => {
                        if converted_changed_instruction == current_op {
                            current_op = current_op + operation.val;
                        } else {
                            current_op += 1;
                        }
                    },
                    OpCode::ACC => {
                        accumulator = accumulator + operation.val;
                        current_op += 1;
                    },
                    OpCode::JMP => {
                        if converted_changed_instruction == current_op {
                            current_op += 1;
                        } else {
                            current_op = current_op + operation.val;
                        }
                    },
                },
                None => {
                    println!("Out of bounds! current_op={}, accumulator={}", current_op, accumulator);
                    if current_op == expected_jmp_exit {
                        completed_successfully = true;
                    }
                }
            }
        }

        if completed_successfully {
            println!("Changed instruction: {}", i);
            break;
        }
    }

    Ok(())
}
