use std::io::{self, Write};

mod lexer;
mod token;

fn main() {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut source = String::new();

        io::stdin()
            .read_line(&mut source)
            .expect("Couldn't read input");
        println!("{:?}", source);
        source.clear();
    }
}
