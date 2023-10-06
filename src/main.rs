
use std::error::Error;
use std::io::{stdin, stdout};
use ansi_term::Color;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => {
            return Err(Box::new(error));
        }
    }

    let mut output = stdout();
    match output.write_all(&mut input.as_bytes()) {
        Ok(_) => {}
        Err(error) => {
            return Err(Box::new(error));
        }
    }

    let mut count = 1;
    if let Some(arg) = std::env::args().nth(1) {
        count = match arg.parse::<u8>() {
            Ok(count) => count,
            Err(error) => {
                return Err(Box::new(error));
            }
        };
    }

    for _ in 0..count {
        let colored_output = Color::Green.paint(input.clone());
        match output.write_all(colored_output.as_bytes()) {
            Ok(_) => {}
            Err(error) => {
                return Err(Box::new(error));
            }
        }
    }

    Ok(())
}

