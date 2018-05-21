extern crate math_solver;

use std::io::{stdin, BufRead};


fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    loop {
        let mut line = String::new();
        if let Err(e) = stdin.read_line(&mut line) {
            println!("{:?}", e);
            return;
        }
        let line = line.trim();

        if line == "q" {
            return;
        }

        match math_solver::evaluate(line.to_string()) {
            Ok(v) => println!("{}", v),
            Err(e) => {
                if let Some(position) = e.get_position() {
                    println!("{}", line);
                    println!("{}^", std::iter::repeat("-").take(position).collect::<String>());
                }
                println!("{:?}", e);
            }
        }
    }
}
