// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display
// match must be exhaustive and explore every option

fn test(x: bool) {
    match x {
        true => println!("true statement" ),
        false => println!("false" ),
    }
}
fn main() {
    test(true);
    test(false);
    
}
