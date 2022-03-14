use std::cell::RefCell;
use std::rc::Rc;
use std::thread::AccessError;

#[derive(Debug)]
enum MenuItem {
    Drink,
    Salad,
}

#[derive(Debug)]
struct ItemOrder {
    item: MenuItem,
    quantity: u32,
}

#[derive(Debug)]
struct TableOrder {
    items: Vec<ItemOrder>,
}

fn new_table_order() -> TableOrder {
    TableOrder {
        items: vec![ItemOrder {
            item: MenuItem::Drink,
            quantity: 1,
        }],
    }
}
// Generic type
// using Rc and RefCell for mutliple and mutable borrow;
// This will allow to modify and add to the TableOrder structure
type Order = Rc<RefCell<Vec<TableOrder>>>;
// by adding Order type, all these can modify and add to it
#[derive(Debug)]
struct Chef(Order);
#[derive(Debug)]

struct WaitStaff(Order);

#[derive(Debug)]
struct Accounting(Order);

fn main() {
    let orders = Rc::new(RefCell::new(vec![]));
    // passing the order vec to all particpants to modify
    // 3 references to the orders vector
    let chef = Chef(Rc::clone(&orders));
    let wait_staff = WaitStaff(Rc::clone(&orders));
    let accounting = Accounting(Rc::clone(&orders));

    let order = new_table_order();
    // create a new scope to avoid issues
    {
        // borrow_mut is par o fRefCeell, Rc lib
        orders.borrow_mut().push(order);
    }

    dbg!(chef.0.borrow());
    dbg!(wait_staff.0.borrow());
    dbg!(accounting.0.borrow());


}