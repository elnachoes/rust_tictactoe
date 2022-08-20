use std::io;

//this gets the input from the cmd line, handles the errors that might throw and removes carriage returns and new lines
pub fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();

    let result = stdin.read_line(&mut input);
    match result {
        Err(_) => {
            panic!("error : input error of some kind")
        }
        _ => {}
    }

    input.replace("\r", "").replace("\n", "")
}