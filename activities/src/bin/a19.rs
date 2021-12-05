// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut items = HashMap::new();
    items.insert("Chairs".to_string(), 5);
    items.insert("Beds".to_string(), 3);
    items.insert("Tables".to_string(), 2);
    items.insert("Couches".to_string(), 0);

    let mut total = 0;
    for (i, amount) in items.iter() {

        total += amount;
        match amount{
            0 => println!("{} are out of stock", i),
            _ => println!("There are {} {}", amount, i),
        }

    }
    println!("total items = {}", total)
}
