use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let val = vec![
        String::from("hi2"),
        String::from("there2"),
        String::from("buddy2"),
        String::from("again2"),
        ];
        for v in val {
            tx2.send(v).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
        
    });
    thread::spawn(move || {
        let val = vec![
        String::from("hi"),
        String::from("there"),
        String::from("buddy"),
        String::from("again"),
        ];
        for v in val {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        
    });

    for received in rx {
        println!("Got: {}", received);
    }
}