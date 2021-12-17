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

pub fn get_input() -> Option<String> {
    println!("input bill");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let word = input.to_string();
    if &word == "" {
        None
    } else {
        Some(word)
    }
    
}

pub fn get_amount_input() -> Option<f32> {
    println!("input bill amount");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let amount = match input.trim().parse::<f32>().unwrap() {
        amount => amount,
        _ => 0.0,
    };
    Some(amount)
}

pub mod BillOperation {
    use crate::Bill;
    use std::collections::HashMap;
    use std::io;
    use crate::get_amount_input;
    use crate::get_input;

    pub fn view_bill(tracker: &mut  HashMap<Option<String>, Option<f32>>) {
       for (bill, amount) in tracker {
           println!("bill {:?}, amount {:?}", bill, amount)
       }
    }
    pub fn add_bill(tracker: &mut  HashMap<Option<String>, Option<f32>>)  {
        let i = get_input();
        let j = get_amount_input();
        tracker.insert(i,j);
    }
}
fn main() {
    use BillOperation::*;
    // need to implement menu options with loop
    let mut tracker: HashMap<Option<String>, Option<f32>> = HashMap::new();
    add_bill(&mut tracker);
    view_bill(&mut tracker);
    

}
