use std::fs;

pub fn cat(path: &str) -> String {
    fs::read_to_string(path).expect("The file doesn't exist!")
}

fn main() {
    let arg: Option<String> = std::env::args().nth(1);

    match arg {
        Some(str) => println!("{}", cat(&str)),
        None => println!("no file specified!"),
    }
}
