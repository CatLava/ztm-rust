// a thread will print a messsage
// digital sign board method
// another thread will be able to continually update that method
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// user defined type with atomic reference pointer
// mutual exclusion lock
type SharedSignData = Arc<Mutex<String>>;

struct DigitalSignBoard {
    display: SharedSignData,
}

impl DigitalSignBoard {
    // lock the data, read it, displa on board
    fn update(&self) {
        // because this data type is Arc with a mutex, we are able to lock it
        let data = self.display.lock();
        println!("Sign data = {}", data);
        // function that would place on digital sign
    }
}

fn spawn_display_thread(display_data: SharedSignData) {
    thread::spawn(|| {
        let board = DigitalSignBoard {
            display: display_data
        };
        loop {
            board.update();
            //to not update constantly will sleep
            thread::sleep(Duration::from_millis(200));
        }
    });
}

fn change_data(display_data: SharedSignData, new_data: &str) {
    let mut data = display_data.lock();
    *data = new_data.to_owned();
    println!("-------updated: {}", data);
}

fn main() {
    let display_data = Arc::new(Mutex::new("initial".to_string()));
    spawn_display_thread(Arc::clone(&display_data));

    thread::sleep(Duration::from_millis(200));
    change_data(Arc::clone(&display_data), "test1");

    thread::sleep(Duration::from_millis(600));
    change_data(Arc::clone(&display_data), "test2");


}
