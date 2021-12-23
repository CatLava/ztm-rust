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
#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f32,
}

// hashmap after this one
pub struct Bills {
    inner: Vec<Bill>
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: vec![]
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.push(bill); 
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.iter().collect()
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
    EditBill,
    DeleteBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::EditBill),
            "4" => Some(Self::DeleteBill),
            _ => None
        }
    }

    fn show() {
        println!("Select a menu option");
        println!("1: Add bills");
        println!("2: View bills");
        println!("3: Edit bills");
        println!("4: Delete bills");
        println!("5: Exit");
        println!("Enter Selection:");

    }
}

pub fn get_input() -> Option<String> {
    println!("input bill");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let word = input.trim().to_string();
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

// refactoring for usage of structs and not these hashmap
// too much borrowing and inconsistent 
pub mod BillOperation {
    use crate::Bill;
    use crate::Bills;
    use std::collections::HashMap;
    use std::io;
    use crate::get_amount_input;
    use crate::get_input;

    pub fn view_bill(tracker: &mut Bills) {
       for (bill, amount) in tracker {
           println!("bill {:?}, amount {:?}", bill, amount)
       }
    }
    pub fn add_bill(tracker: &mut Bills)  {
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        let amount = match get_amount_input() {
            Some(amount) => amount,
            None => return,
        } ;
        let bill = Bill {name, amount};
        tracker.push(Bill);
        println!("Bill added!")
    }

    pub fn delete_bill(tracker: &mut  Bills) {
        println!("enter bill name to delete: ");
        let delete_name = get_input();
        if tracker.contains_key( & &delete_name) {
            tracker.remove( & delete_name);
            println!("deleting");
        } else {
            println!("bill not found");
        }
    }

    pub fn edit_bill(tracker: &mut  Bills) {
        println!("enter bill name to eidt: ");
        let edit_name = get_input();
        if tracker.contains_key( & &edit_name) {
            tracker.remove( & edit_name);
            println!("deleting");
            let edit_amount = get_amount_input();
            tracker.insert(edit_name, edit_amount);
        } else {
            println!("bill not found");
        }
    }
}
fn main() {
    use crate::BillOperation::*;
    // need to implement menu options with loop
    let mut tracker: HashMap<Option<String>, Option<f32>> = HashMap::new();
    loop {
        // Display menu
        // make a choice based on input
        MainMenu::show();
        let input = get_input().expect("no data");
        println!("{:?}", input);
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => add_bill(&mut tracker),
            Some(MainMenu::ViewBill) => view_bill(&mut tracker),
            Some(MainMenu::EditBill) => edit_bill(&mut tracker),
            Some(MainMenu::DeleteBill) => delete_bill(&mut tracker),
            _ => println!("")
        }

    }
    
    

}
