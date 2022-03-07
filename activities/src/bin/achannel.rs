use crossbeam_channel::unbounded;
use std::thread;
fn channel_ex() {

    let (sender, receiver) = unbounded();

    sender.send("hello, channel");

    match receiver.recv() {
        Ok(msg) => println!("message {:?}", msg),
        Err(e) => println!("error {:?}", e)
    }
}
fn main() {
    let (s,r) = unbounded();

    let handle = thread::spawn(move || match r.recv() {
        Ok(msg) => println!("message {:?}", msg),
        Err(e) => println!("error {:?}", e)
    });

    s.send("hello from main");
    handle.join();

}