use crate::calc::handle_calculate;
use crate::io::read_line;

mod calc;
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

fn handle_command(cmd: Command) -> bool {
    match cmd {
        Command::Exit => return false,
        Command::Formula(input) => {
            let result = handle_calculate(&input);
            match result {
                Ok(val) => println!("{input} = {val}"),
                Err(e) => println!("{e}"),
            };
        }
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
