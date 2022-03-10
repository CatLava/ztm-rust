// This will use RefCell<t> for mutabaility
// ability to mutate an immutable reference
// Borrowing immutably, Box, Rc, RefCell

// trait to implement on enums or structs
pub trait Messenger {
    fn send(&self, msg: &str);
}

// struct with lifetime and generic involving messenger
pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<a', T> 
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: Your are over quota");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("urgent used 90% of quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning, used 75% of quota");
        }
    }
}
