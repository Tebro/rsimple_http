use crate::calc::handle_calculate;
use crate::http::server::start_server;
use crate::io::read_line;

mod calc;
mod http;
mod io;

#[derive(Debug)]
enum Command {
    Exit,
    Serve,
    Formula(String),
}

// Checks for special commands, or falls back to a formula
fn parse_command(input: &str) -> Command {
    match input {
        "exit" => Command::Exit,
        "serve" => Command::Serve,
        _ => Command::Formula(String::from(input)),
    }
}

fn handle_command(cmd: Command) -> bool {
    match cmd {
        Command::Exit => return false,
        Command::Serve => {
            // TODO handle
            let _res = start_server(|req| match handle_calculate(&req.body) {
                Ok(result) => format!("{}", result).to_string(),
                Err(e) => format!("Error: {}", e).to_string(),
            });
        }
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
