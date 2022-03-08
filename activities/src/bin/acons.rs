use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);
 
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
fn main() {
    let list = Cons(1, 
        Box::new(Cons( 2, 
            Box::new(Cons(3, 
                Box::new(Nil))))));
    
    let x = 8;
    let y = MyBox::new(x);

    assert_eq!(8, x);
    // * is a dereference in Rust
    assert_eq!(8, *y);

    let b = (3, Box::new(list));
    let c = (4, Box::new(list));

    
}