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

enum PowerStates {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
    Invalid,
}

fn TranslateInput(word: PowerStates) {
    match word {
        PowerStates::Off => println!("Turning Off"),
        PowerStates::Sleep => println!("sleeping"),
        PowerStates::Reboot => println!("reboiting"),
        PowerStates::Shutdown => println!("shut down"),
        PowerStates::Hibernate => println!("Hibernating"),
        _ => println!("not an input"),
    }
}    
fn main() {
    let user_input = "invalid".to_lowercase().to_string();
    let newinput = match user_input.as_str() {
        "off" => PowerStates::Off,
        "sleep" => PowerStates::Sleep,
        "reboot" => PowerStates::Reboot,
        "shutdown" => PowerStates::Shutdown,
        "hibernate" => PowerStates::Hibernate,
        _ => PowerStates::Invalid,

    };
    TranslateInput(newinput)
}
