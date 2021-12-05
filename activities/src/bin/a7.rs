// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Red,
    Blue,
    Green,
    Purple,
}

fn pcolor(var: Colors) {
    match var {
        Colors::Red => println!("Red"),
        Colors::Blue => println!("Blue"),
        Colors::Green => println!("Green"),
        Colors::Purple => println!("Purple"),
    }

}
fn main() {
    let test = Colors::Blue;
    pcolor(test);
}
