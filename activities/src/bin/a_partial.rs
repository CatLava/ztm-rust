#[derive( PartialEq, PartialOrd)]
struct Test {
    // PartialOrd will only compare the first instanct of the struct
    id: i32,
    name: String
}

fn main() {
    let a = Test{ id: 1, name: "1".to_string()};
    let b = Test{ id: 2, name: "2".to_string()};
    let c = Test{ id: 1, name: "1".to_string()};
    if a ==c {
        println!("true")
    }

    if a < b {
        println!("true2")
    }

}