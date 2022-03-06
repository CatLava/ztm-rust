use std::thread::{self, JoinHandle};
use std::time::Duration;

fn t1(){
    let iterations = 10;
    let a = thread::spawn(move || {
        for i in 0..iterations{
            println!("      A:{}", i)
        }
    });

    let b = thread::spawn(move || {
        for i in 0..iterations{
            println!("         B:{}", i)
        }
    });
    // need to join threads for execution
    a.join();
    b.join();

}

fn ex2() {
    let t: JoinHandle<usize> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });

    println!("waiting on thread");
    match t.join() {
        Ok(n) => println!("value: {}", n),
        Err(e) => println!("Error, {:?}", e),
    }
}
fn main() {
    let data = vec!['a', 'b', 'c'];
    let nd = thread::spawn(move || {
        let data: Vec<char> = data.iter()
        .map(|letter| letter.to_ascii_uppercase())
        .collect();
        data
    });

    match nd.join() {
        Ok(n) => println!("{:?} value is", n),
        Err(e) => println!("{:?} error", e)
    }

   
}