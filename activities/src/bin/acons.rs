use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
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
        Rc::new(Cons( 2, 
            Rc::new(Cons(3, 
                Rc::new(Nil))))));
    
    let x = 8;
    let y = MyBox::new(x);

    assert_eq!(8, x);
    // * is a dereference in Rust
    assert_eq!(8, *y);

    let b = (3, Rc::new(&list));
    let c = (4, Rc::new(&list));

    
}