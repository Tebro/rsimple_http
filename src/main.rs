use regex::Regex;
use crate::io::read_line;
use std::fmt::{Display, Formatter};

mod io;

#[derive(Debug)]
enum Command {
    Exit,
    Formula(String),
}

// Checks for special commands, or falls back to a formula
fn parse_command(input: &str) -> Command {
    match input {
        "exit" => Command::Exit,
        _ => Command::Formula(String::from(input)),
    }
}

enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let as_string = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        };
        write!(f, "{}", as_string)
    }
}

fn parse_formula(input: &str) -> Result<(f64, Operator, f64), &str> {
    let re = Regex::new(r"(?<val1>\d+\.?\d*)\s?(?<op>\+|\-|\*|/)\s?(?<val2>\d+\.?\d*)").unwrap();
    match re.captures(input) {
        None => return Err("No matches"),
        Some(caps) => {
            let val1 = &caps["val1"];
            let val2 = &caps["val2"];
            let op = match &caps["op"] {
                "+" => Operator::Plus,
                "-" => Operator::Minus,
                "*" => Operator::Multiply,
                "/" => Operator::Divide,
                _ => return Err("invalid operator"),
            };

            return Ok((
                val1.parse::<f64>().unwrap(),
                op,
                val2.parse::<f64>().unwrap(),
            ));
        }
    }
}

fn handle_calculate(input: String) {
    let Ok((val1, op, val2)) = parse_formula(&input) else {
        println!("could not parse formula");
        return;
    };

    let result = match op {
        Operator::Plus => val1 + val2,
        Operator::Minus => val1 - val2,
        Operator::Multiply => val1 * val2,
        Operator::Divide => val1 / val2,
    };
    println!("{input} = {result}");
}

fn handle_command(cmd: Command) -> bool {
    match cmd {
        Command::Exit => return false,
        Command::Formula(input) => handle_calculate(input),
    }
    return true;
}

fn main() {
    println!("Welcome to the calculator!");
    loop {
        println!("Enter your input: ");
        let Ok(input) = read_line() else {
            println!("Could not read input");
            continue;
        };
        let cmd = parse_command(&input);
        let go_again = handle_command(cmd);
        if !go_again {
            break;
        }
    }
    println!("Good bye!");
}
