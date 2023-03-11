use std::io;

fn main() {
println!("Welcome to the Rust calculator app.");

loop {
// Prompt the user for input.
println!("Please enter an operation.");

let mut input = String::new();

io::stdin().readline(&mut input)
.expect("Failed to read line");

// Evaluate the user's input.
evaluateinput(input);

} // End of the loop.
} // End of main().


fn evaluateinput(input: String) { // Begin function definition.

// Split the string on any whitespace character.
let v: Vec<&str> = input.trim().split(' ').collect();

if !v.isempty() {
match v0 {
"add" => if let (Ok(x), Ok(y)) = (v1.parse::<i32>(), v2.parse::<i32>()) { // Parse as i32 integers in a tuple.
println!("{} + {} = {}", x, y, add(x, y)); // Call add() function with parameters x & y and print results with interpolation of variables x & y and the result of add().
} else { // Else not parseable as i32 integer..
println!("Operation not valid!"); // Print error message. },

"subtract" => if let (Ok(x), Ok(y)) = (v1.parse::<i32>(), v2.parse::<i32>()) 
}}}} // Parse as i32 integers in a tuple. Ṕ println!("{} - {} = {}", x, y, subtract(x, y)); // Call subtract() function with parameters x & y and print results with interpolation of variables x & y and the result of subtract(). } else { // Else not parseable as i32 integer.. println!("Operation not valid!"); // Print error message.