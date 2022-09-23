use ferris_says::say;
use std::io::{stdout,BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello Rust...2022/09/22");
    let width = message.chars().count();
    
    let mut write = BufWriter::new(stdout.lock());
    say(message.as_bytes(),width,&mut write).unwrap();
}
