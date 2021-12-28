// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn perimeter(&self) -> i32;
}

struct square  {
    l: i32,
}
impl Perimeter for square {
    fn perimeter(&self) -> i32 {
        l*4
    }
}

struct triangle {
    s1: i32,
    s2: i32,
    s3: i32,

} 
impl Perimeter for triangle {
    fn perimeter(&self) -> i32 {
        s1 + s2 + s3
    }
}

fn main() {}
