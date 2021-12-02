// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

#[derive(Debug)]
struct Person {
    age: i32,
    name: String,
    color: String,
}

// we will do an impl on Person to print info
impl Person {
    fn print_name(&self) {
        println!("name is: {:?}", self.name );
    }

    fn print_color(&self) {
        println!("favorite color is: {:?}", self.color );
    }
}

fn print_data(data: &str) {
    println!("The color is {:?}", data );
}

fn main() {
    let people = vec![
        Person {
            age: 25,
            // convert string literal using to_string, or String::from
            name: "Moosha".to_string(),
            color: String::from("blue"),
        },
        Person {
            age: 43,
            name: "Hemholt".to_string(),
            color: String::from("red"),
        },
        Person {
            age: 32,
            name: "Stan".to_string(),
            color: String::from("yellow"),
        }

    ];

    for person in people {
        if person.age < 40 {
            person.print_name();
            person.print_color();
            print_data(&person.color);
        }
        println!("{:?}", person );
    }
}
