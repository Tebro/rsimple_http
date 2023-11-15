use std::io;

pub fn read_line() -> io::Result<String> {
    let mut input = String::new();

    let _n = io::stdin().read_line(&mut input)?; 
    return Ok(input[..input.len() - 1].to_string());
}
