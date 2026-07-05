

use::std::io::stdin;

pub fn read_input(message: &str) -> String {
    println!("{}", message);

    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}