// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing
#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}
#[derive(Debug)]
struct Shirt(Color);


impl Shirt {
    fn new(c: Color) -> Self {
        Self(c)   
    }
}

#[derive(Debug)]
struct Shoes(Color);
impl Shoes {
    fn new(c: Color) -> Self {
        Self(c)   
    }
}

#[derive(Debug)]
struct Pants(Color);
impl Pants {
    fn new(c: Color) -> Self {
        Self(c)   
    }
}

fn display_shirt(shirt: Shirt) {
    println!("{:?}", shirt)
}

fn display_shoes(shoes: Shoes) {
    println!("{:?}", shoes)
}


fn display_pants(pants: pants) {
    println!("{:?}", pants)
}
fn main() {
    let outfit = Shirt::new(Color::Yellow); 
    display(outfit)
}
