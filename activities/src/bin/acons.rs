use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;
use std::cell::{self, RefCell};

#[derive(Debug)]
enum List {
    // an Rc and a Refcell allow for multiple borrows that can be mutated
    // Rc is for multiple borrows
    // refCell is for mutable borrows
    Cons(Rc<RefCell<i32>>, Rc<List>),
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
    /*let list = Cons(1, 
        Rc::new(Cons( 2, 
            Rc::new(Cons(3, 
                Rc::new(Nil))))));
    */
    
    let x = 8;
    let y = MyBox::new(x);

    assert_eq!(8, x);
    // * is a dereference in Rust
    assert_eq!(8, *y);
    let value = Rc::new(RefCell::new(5));

    let ac = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let ab = Cons(Rc::new(RefCell::new(6)), Rc::clone(&ac));

    let ad = Cons(Rc::new(RefCell::new(10)), Rc::clone(&ac));

    *value.borrow_mut() += 10;

    println!("ac after = {:?}", ac);
    println!("ab after = {:?}", ab);
    println!("ad after = {:?}", ad);



    
}