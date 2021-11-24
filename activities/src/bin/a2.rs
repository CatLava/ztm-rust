// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add(x: u64, y: u64) -> u64 {
    x+y
}

fn dap(result: u64) {
    println!("{:?}", result);
}
fn main() {
    dap(add(3,4))
}
