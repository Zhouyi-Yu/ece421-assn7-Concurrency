use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/*
Part (a) - Why the original code fails:
The code fails due to ownership and synchronization rules in Rust.
1. Ownership: When `thread::spawn(move || ...)` is called, the `move` keyword causes `sample_data`
   to be moved into the thread's closure. This means the original `sample_data` in the main thread's
   scope is invalidated after the first iteration.
2. Concurrent Mutation: Even if ownership wasn't moved, Rust's borrow checker would prevent
   multiple threads from having mutable references to the same data (the vector) simultaneously
   to avoid data races.
*/

fn main() {
    // Part (b) - Apply Mutex and Arc so it runs
    let sample_data = Arc::new(Mutex::new(vec![1, 81, 107]));

    for i in 0..10 {
        let data = Arc::clone(&sample_data);
        thread::spawn(move || {
            let mut data_lock = data.lock().unwrap();
            data_lock[0] += i;
        });
    }

    // Give some time for threads to complete
    thread::sleep(Duration::from_millis(50));

    println!("Final sample_data: {:?}", sample_data.lock().unwrap());
}
