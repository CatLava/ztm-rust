// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
struct Customer {
    age: i32,
    items: Items,
}

fn Purchase_Approve(limit: &Customer) -> Result<String, String> {
    if limit.age >= 21 {
        Ok("Purchase approved".to_string())
    } else {
        Err("Unable to purchase because below age".to_string())
    }
}

enum Items {
    Food(String),
    Alcohol(String),
    Other(String)
}

fn main() {
     let tim = Customer {
         age: 23,
         items: Items::Food("banana".to_string())
     };

     let eric = Customer {
         age: 33,
         items: Items::Alcohol("banana vodka".to_string())
     };

     let mike = Customer {
         age: 13,
         items: Items::Alcohol("banana bourbon".to_string())
     };

     let all = vec![&tim, &eric, &mike];

     for person in all {
         match &person.items {
             Items::Alcohol(item) => {
                 println!("{:?}" , Purchase_Approve(person));
             },
             _ => println!("no alcohol, continue")
         }
     }

}
