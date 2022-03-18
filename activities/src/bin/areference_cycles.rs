// This code will show a memory leak/ Reference cyle 
// we will investigate how to prevent
// skipping on the memory leak
// This will be child and parent refernces with wak nodes
use std::rc::{Rc, Weak};
use std::cell::RefCell;


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// we want the node to own the children

fn main() {
    // we want leaf to know branch is a parent
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong = {}, weak = {}",
     Rc::strong_count(&leaf),
     Rc::weak_count(&leaf)
    );

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("branch strong = {}, weak = {}",
    Rc::strong_count(&branch),
    Rc::weak_count(&branch)
    );

    println!("leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf)
        );

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!("leaf strong = {}, weak = {}",
   Rc::strong_count(&leaf),
   Rc::weak_count(&leaf)
    );

}

