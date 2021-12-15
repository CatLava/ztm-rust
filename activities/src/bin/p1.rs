// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::collections::HashMap;
use std::io;

// defining bill
struct Bill {
    name: String,
    amount: f32,
}

pub mod BillOperation {
    use crate::Bill;
    use std::collections::HashMap;
    
    pub fn view_bill(tracker: HashMap) {
        for bill in tracker{
            println!("bill {}", bill)
        }
    }
    pub fn add_bill(tracker: type<HashMap>) {
        // user input of bill
        let bill = io::read_line();
        let amount = io::read_line();
        let item = Bill {
            name: bill,
            amount: amount,
        }
        tracker.insert(item)
    }
}
fn main() {
    use BillOperation::*;
    // need to implement menu options with loop
    let mut tracker = HashMap::new();
    add_bill(tracker)

}
