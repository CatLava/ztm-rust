// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

// we want to do an impl on the power stat
#[derive(Debug)]
enum PowerStates {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
    Invalid,
}

impl PowerStates {
    fn new(state: &str)-> Option<PowerStates> {
        let state = state.trim().to_lowercase();
        match state.as_str() {
            "off" => Some(PowerStates::Off),
            "sleep" => Some(PowerStates::Sleep),
            "reboot" => Some(PowerStates::Reboot),
            "shutdown" => Some(PowerStates::Shutdown),
            "hibernate" => Some(PowerStates::Hibernate),
            _ => Some(PowerStates::Invalid),
        
        }
    }
}

fn TranslateInput(word: Option<PowerStates>) {
    use PowerStates::*;
    match word {
        Some(Off) => println!("Turning Off"),
        Some(Sleep) => println!("sleeping"),
        Some(Reboot) => println!("reboiting"),
        Some(Shutdown) => println!("shut down"),
        Some(Hibernate) => println!("Hibernating"),
        _ => println!("not an input"),
    }
}    
fn main() -> io::Result<()> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?; 
    println!("{}", user_input);
    let newinput = PowerStates::new(&user_input);

    Ok(TranslateInput(newinput))
}
