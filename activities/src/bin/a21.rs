// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let tryname = "sam".to_string();
    let verify = find_user(&tryname)
        .map(|user_id| { User {
            user_id: user_id,
            name: tryname.to_string()
            }
        }
    );

    match verify {
        Some(verify) => println!("{:?}", verify),
        _ => println!("user not found")
    }


}
