// closures don't have a defined size and need to be stored on the heap 
// closures are essentially one line functions 
// need to use a box for these closures
// Box stores on the heap vs. the stack
// dyn refers to different types

fn math (a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32> ) -> i32 {
    op(a,b)
}

fn main() {
    // when something is boxed it needs to own everything
    // in this case we will use a move 
    let name  = "ted";
    let add = Box::new(move |a, b| {
        println!("add function for {}", name);
        a + b
    });
    println!("{}", math(2, 2, add)) 
}