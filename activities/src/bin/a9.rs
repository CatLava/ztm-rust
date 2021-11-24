// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn tup() -> (i32, i32) {
    let (x,y) = (2, 9);
    if y > 5 {
        println!("Higher");
    } else if y < 5 {
        println!("lower" );
    } else {
        println!("equal");
    }


}
fn main() {
    tup()

}
