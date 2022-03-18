use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

struct Counter(usize);



fn main() {
    let counter = Counter(0);
    let shared_counter = Arc::new(Mutex::new(counter));

    let thread_1_counter = Arc::clone(&shared_counter);
    let thread_2_counter = shared_counter.clone();

    let thread1 = thread::spawn(move || {
        let mut counter = thread_1_counter.lock();
        counter.0 += 1;
    });

    let thread2 = thread::spawn(move || {
        let mut counter = thread_2_counter.lock();
        counter.0 += 1;
    });

    thread1.join().and_then(|_| thread2.join());
    println!("{}", shared_counter.lock().0);

}