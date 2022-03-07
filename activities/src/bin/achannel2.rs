
// This will create channel and multithreaded script
use std::thread;
use crossbeam_channel::unbounded;

enum ThreadMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

fn main () {
    // need to create channel first
    let (s, r) = unbounded();
    // inifinite loop thread that receives messages
    let handle = thread::spawn(move || loop {
        match r.recv() {
            Ok(msg) => match msg {
                ThreadMsg::PrintData(d) => println!("{}", d),
                ThreadMsg::Sum(x,y) =>  println!("sum {}", x + y),
                ThreadMsg::Quit => {
                    println!("thread terminating");
                    break;
                    }
            }
            Err(e) => {
                    println!("Disconnected");
                    break;
                }
            }
            
        
    });

    s.send(ThreadMsg::PrintData("Hi".to_string()));
    s.send(ThreadMsg::Sum(3,4));
    s.send(ThreadMsg::Quit);
    handle.join();
}