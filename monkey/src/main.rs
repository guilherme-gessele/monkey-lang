use std::io::{stdin, stdout, Write, BufRead};

use monkey_repl::Repl;

fn main() {

    const PROMPT: &str = ">>";
    let repl = Repl::new();

    loop {

        print!("{PROMPT} ");

        let _ = stdout().flush();

        if let Some(Ok(ref line)) = stdin().lock().lines().next() {
            let result = repl.evaluate(line);
            println!("{:?}", result);
        }      
    }
}
