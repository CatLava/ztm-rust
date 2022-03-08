// custom smart point implementing the drop trait
struct CustomSmartPointer{
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping data of {}", self.data)
    }
}

fn main() {
    let a = CustomSmartPointer{data: "hello a".to_string()};
    let b = CustomSmartPointer {data: "Hello 2b".to_string()};
    println!("CustomSmartPointers created");
    
}

