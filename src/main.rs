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

        match math_solver::evaluate(line) {
            Ok(v) => println!("{}", v),
            Err(e) => {
                if let Some(span) = e.get_span() {
                    println!("{}", line);
                    println!(
                        "{}{}",
                        std::iter::repeat(" ").take(span.from).collect::<String>(),
                        std::iter::repeat("^")
                            .take(span.to - span.from)
                            .collect::<String>()
                    );
                }
                println!("{:?}", e);
            }
        }
    }
}
