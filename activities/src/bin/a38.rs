// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

use std::thread;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let m1 = thread::spawn(move || {
        msg_hello()
    });
    let m2 = thread::spawn(move || {
        msg_thread()
    });
    let m3 = thread::spawn(move || {
        msg_excited()
    });

    // Join returns a result
    // could use the expect funtion here rather than match
    match m1.join() {
        Ok(m) => println!("message1 {:?}", m),
        Err(e) => println!("error of some sort{:?}", e),
    };
    match m2.join() {
        Ok(m) => println!("message1 {:?}", m),
        Err(e) => println!("error of some sort{:?}", e),
    };
    match m3.join() {
        Ok(m) => println!("message1 {:?}", m),
        Err(e) => println!("error of some sort{:?}", e),
    };
}
