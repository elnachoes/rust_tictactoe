pub fn clean_input(input : &mut String) {
    *input = input.replace("\r", "").replace("\n", "");
}