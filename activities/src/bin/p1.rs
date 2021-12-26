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
    inner: HashMap<String, Bill>
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill); 
    }

    fn get_all(&self) -> Vec<&Bill> {
        // collect values into a vector
        self.inner.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        // is_some will return true if value is removed
        // else value will be false if not removed
        self.inner.remove(name).is_some()
    }

    fn edit(&mut self, name: &str, amount: f32) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => { 
                bill.amount = amount;
                true
            }
            None => false,
        }
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
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let word = input.trim().to_string();
    if &word == "" {
        None
    } else {
        Some(word)
    }
    
}

// refactor this 
pub fn get_amount_input() -> Option<f32> {
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None
        };
        let amount = input.parse::<f32>() ;
        match amount {
            Ok(amount) => return Some(amount),
            Err(_) => println!("invalid try again"),
        };
    }
    // parse will attempt to turn into appropriate data type
   
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

    pub fn view_bill(tracker: &Bills) {
       for bills in tracker.get_all() {
           println!("bill {:?}", bills)
       }
    }
    pub fn add_bill(tracker: &mut Bills)  {
        println!("enter bill name:");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        println!("enter bill amount:");
        let amount = match get_amount_input() {
            Some(amount) => amount,
            None => return,
        } ;
        let bill = Bill {name, amount};
        // impl method add created above for adding bills 
        tracker.add(bill);
        println!("Bill added!")
    }

    pub fn delete_bill(tracker: &mut  Bills) {
        for bill in tracker.get_all() {
            println!("{:?}", bill)
        }
        println!("enter bill name to delete: ");
        let delete_name = match get_input() {
            Some(name) => name,
            None => return,
        };
        if tracker.remove(&delete_name) {
            println!("successfully delete")
        } else {
            println!("bill does not match name")
        }
    }

    pub fn edit_bill(tracker: &mut  Bills) {
        println!("Edit bills from below");
        for bill in tracker.get_all() {
            println!("{:?}", bill)
        }
        println!("bill name");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        println!("new bill amount");
        let amount = match get_amount_input() {
            Some(amount) => amount,
            None => return,
        } ;
       
        if tracker.edit(&name, amount) {
            println!("new bill list");
            for bill in tracker.get_all() {
                println!("{:?}", bill)
            }
        } else {
        println!("new bill list");
        }
    }       
}

fn main() {
    use crate::BillOperation::*;
    use crate::MainMenu;
    // need to implement menu options with loop
    let mut tracker = Bills::new();
    loop {
        // Display menu
        // make a choice based on input
        MainMenu::show();
        let input = get_input().expect("no data");
        println!("{:?}", input);
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => add_bill(&mut tracker),
            Some(MainMenu::ViewBill) => view_bill(&tracker),
            Some(MainMenu::EditBill) => edit_bill(&mut tracker),
            Some(MainMenu::DeleteBill) => delete_bill(&mut tracker),
            _ => println!("")
        }

    }
    
    

}
