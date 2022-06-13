pub fn stringToI32() {
	io::stdin() 
    .read_line(&mut input_line)
    .expect("Failed to read line");
	 let x: i32 = input_line
    .trim()
    .parse()
    .expect("Input not an integer");
}