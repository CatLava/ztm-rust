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

struct Square  {
    l: i32,
}
impl Perimeter for Square {
    fn perimeter(&self) -> i32 {
        self.l*4
    }
}

struct Triangle {
    s1: i32,
    s2: i32,
    s3: i32,

} 
impl Perimeter for Triangle {
    fn perimeter(&self) -> i32 {
        self.s1 + self.s2 + self.s3
    }
}

fn print_perimeter(shape: impl Perimeter) {
    let perimeter = shape.perimeter();
    println!("perimeter is: {}", perimeter)
}

fn main() {
    let mut sq = Square {l : 8} ;
    let mut tri =  Triangle{ s1: 5, s2: 4, s3: 9};
    print_perimeter(sq);
    print_perimeter(tri);
    
}
