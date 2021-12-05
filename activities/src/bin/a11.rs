// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn quantity(gi: &GroceryItem) {
    println!("quantity: {:?}", gi.quantity)
}

fn id(gi: &GroceryItem) {
    println!("id: {:?}", gi.id)
}

fn main() {
    let banana = GroceryItem {
        id: 8,
        quantity: 5,
    };

    quantity(&banana);
    id(&banana)
}
