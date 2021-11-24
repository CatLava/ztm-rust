// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn fun(i: i32) {
    match i {
        1 => println!("only 1" ),
        2 => println!("even 2"),
        5 => println!("nice 5"),
        _ => println!("not even close" ),
    }
}
fn main() {
    fun(1);
    fun(2);
    fun(5);
    fun(8);
}
