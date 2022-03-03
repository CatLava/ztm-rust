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
fn main() {
    //t1();
    let t: JoinHandle<usize> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });
   
}