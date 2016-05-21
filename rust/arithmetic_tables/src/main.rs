use std::cmp;
use std::env;
use std::process;

#[derive(Debug)]
enum DataError {
    InvalidArgument,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl std::str::FromStr for Operator {
    type Err = DataError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('+') => Ok(Operator::Add),
            Some('-') => Ok(Operator::Sub),
            Some('*') => Ok(Operator::Mul),
            Some('/') => Ok(Operator::Div),
            _ => Err(DataError::InvalidArgument),
        }
    }
}

fn apply_op(op: Operator, a: i32, b: i32) -> i32 {
    match op {
        Operator::Add => a + b,
        Operator::Sub => a - b,
        Operator::Mul => a * b,
        Operator::Div => {
            if b == 0 {
                0
            } else {
                a / b
            }
        }
    }
}

fn print_arithmetic_table(op: Operator, max_number: i32) {
    let spacing = 3;
    let ndigits = |n: i32| (n as f32).log10() as usize;
    let max_width = spacing + ndigits(cmp::max(max_number, apply_op(op, max_number, max_number)));

    print!(" {:?}", op);
    for _ in 0..max_width - 2 {
        print!(" ");
    }
    let operand_width = 3 + max_width - 2;

    print!("| ");
    for i in 0..max_number + 1 {
        print!("{:width$?}", i, width = max_width);
    }
    let bar_width = 2;

    println!("");
    let line_width = operand_width + bar_width + 1 + (1 + max_number as usize) * max_width;

    for _ in 0..line_width {
        print!("-");
    }
    println!("");

    for i in 0..max_number + 1 {
        print!("{:width$}  | ", i, width = max_width);
        for j in 0..max_number + 1 {
            print!("{:width$}",
                   apply_op(op, i, j),
                   width = max_width);
        }
        println!("");
    }
}

fn main() {
    let mut args = env::args();

    let program_name = args.next();
    let operator = args.next();
    let number = args.next();

    if operator.is_none() || number.is_none() {
        println!("usage: {} {{ + | - | * | / }} max_number",
                 program_name.unwrap());
        process::exit(1);
    }

    let operator: Operator = operator.unwrap().parse().unwrap();
    let number: i32 = number.unwrap().parse().unwrap();

    print_arithmetic_table(operator, number);
}
