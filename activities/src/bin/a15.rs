// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info



// price is included as the f32
#[derive(Debug)]
enum TicketType {
    Standard(f32),
    Backstage(String, f32),
    Vip(String, f32),
}


fn main() {
    let s = TicketType::Standard(32.0);
    let b = TicketType::Backstage("Moosha".to_string(), 100.0);
    let v = TicketType::Vip("Hemholt".to_string(), 250.00);

    let tickets = vec![&s, &b, &v];

    for tic in tickets {
        match tic {
            TicketType::Backstage(name, price) => {
                println!("Backstage held by: {:?} for : {:?}", name, price )
            },
            TicketType::Vip(name, price) => println!("Vip held by: {:?} for : {:?}", name, price ),
            TicketType::Standard(_) => println!("{:?}", tic ),
            _ => println!("test" ),
        }
    }
}
