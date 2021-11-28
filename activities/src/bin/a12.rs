// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// we can also do an impl for color
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    // borrowing self and doing a match on the color
    fn show(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Blue => println!("Blue"),
            Color::Green => println!("Green"),
        }
    }
}

// dimensions can also have an impl and print line implementation
struct Dimensions {
    length: f32,
    width: f32,
    height: f32,
}

impl Dimensions {
    fn show(&self) {
        println!("length: {:?}", self.length);
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
    }
}

struct Box {
    dimensions: Dimensions,
    weight: f32,
    color: Color
}

impl Box {
    // creates a standard issue box if no inputs applied
    fn new_box(weight: f32, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn show_characterics(&self) {
        self.dimensions.show();
        println!("weight: {}", self.weight);
        self.color.show()
    }
}
fn main() {
    let dims = Dimensions {
        length: 20.0,
        width: 1.1,
        height: 4.6,
    };
    let boxie = Box::new_box(8.0, Color::Green, dims);
    boxie.show_characterics()
}
