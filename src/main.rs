use std::io::{self, Write};

fn main() {
    let mut source = String::new();

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut source)
            .expect("Couldn't read input");
        println!("{:?}", source);
        source.clear();
    }
}
