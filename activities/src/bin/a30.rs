// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors
#[derive(Debug)]
enum VehicleBody {
    Truck,
    Car,
    Scooter,
}
trait Body {
    fn show(&self);
}

impl Body for VehicleBody {
    fn show(&self) {
        println!("This is the body {:?}", self)
    }
}

#[derive(Debug)]
enum VehicleColor {
    Red,
    Green,
    Blue,
    Orange,
}

trait Color {
    fn show(&self);
}

impl Color for VehicleColor {
    fn show(&self) {
        println!("This is the color {:?}", self)
    }
}

#[derive(Debug)]
struct Vehicle<T: Body, U: Color> {
    body: T,
    color: U,

}

impl<T: Body, U: Color> Vehicle<T, U> {
    pub fn new(body: T, color: U) -> Self {
        Self { body, color}
    }
}

fn main() {
    let nv = Vehicle::new(VehicleBody::Scooter, VehicleColor::Orange);
    println!("{:?}", nv)
}
