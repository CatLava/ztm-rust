// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
/// This is a documentation comment
struct Student {
    name: String,
    locker: Option<i32>
}
fn main() {
    let moosha = Student{
        name: "Moosha Cartman".to_string(),
        locker: Some(420)
    };

    let eric = Student {
        name: "Eric Cartman".to_string(),
        locker: None
    };

    let studs = vec![&moosha, &eric];

    for i in studs {
        match i.locker {
            Some(num) => println!("locker: {:?}", i.locker),
            None => println!("No locker assigned"),
        }

    }
}
