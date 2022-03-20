// recursive function that locks and will break
//
use parking_lot::Mutex;

// change mutex to reentrant mutex for this code to work
// Mutex -> Rentrant Mutex
fn recurse(data: Rc<Mutex<u32>>, remaining: usize) -> usize {
    let mut locked = data.lock();
    match remaining {
        rem if rem == 0 => 0,
        // recursive function that will continue to call lock
        // also cloned the initial lock
        rem => recurse(Rc::clone(&data), rem-1),
    }
}