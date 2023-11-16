use crate::calc::operator::Operator;
use regex::Regex;

mod operator;

struct Formula(f64, Operator, f64);

impl Formula {
    fn execute(&self) -> f64 {
        let Formula(val1, op, val2) = self;
        match op {
            Operator::Plus => val1 + val2,
            Operator::Minus => val1 - val2,
            Operator::Multiply => val1 * val2,
            Operator::Divide => val1 / val2,
        }
    }
}

fn parse_formula(input: &str) -> Result<Formula, &str> {
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

            return Ok(Formula(
                val1.parse::<f64>().unwrap(),
                op,
                val2.parse::<f64>().unwrap(),
            ));
        }
    }
}

pub fn handle_calculate(input: &String) -> Result<f64, String> {
    let formula = parse_formula(&input)?;
    Ok(formula.execute())
}
