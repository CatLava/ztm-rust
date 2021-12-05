// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

#[derive(Debug)]
enum Flavors {
    Grape,
    Orange,
    Blast,
}

struct DrinkDetails {
    flavor: Flavors,
    amount: i32
}

fn Orders(drink: DrinkDetails) {
    match drink.flavor {
        Flavors::Grape => println!("grape"),
        Flavors::Orange => println!("orange"),
        Flavors::Blast => println!("Blast"),
    }

    println!("oz: {}", drink.amount)
}


fn main() {
    let one = DrinkDetails{
        flavor: Flavors::Grape,
        amount: 24
    };
    println!("{:?}, {:?}", one.flavor, one.amount);

    let two = DrinkDetails{
        flavor: Flavors::Blast,
        amount: 32
    };
    Orders(two);
}
