use std::io;

//this gets the input from the cmd line, handles the errors that might throw and removes carriage returns and new lines
pub fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();

    let result = stdin.read_line(&mut input);
    if let Err(_) = result {
        panic!("error : input error of some kind")
    }

    input.replace("\r", "").replace("\n", "")
}