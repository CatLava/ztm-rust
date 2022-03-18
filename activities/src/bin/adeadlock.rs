// recursive function that locks and will break
//
use parking_lot::Mutex;

fn recurse(data: Rc<Mutex<u32>>, remaining: usize) -> usize {
    let mut locked = data.lock();
    match remaining {
        rem if rem == 0 => 0,
        // recursive function that will continue to call lock
        // also cloned the initial lock
        rem => recurse(Rc::clone(&data), rem-1),
    }
}