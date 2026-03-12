use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Debug)]
struct Bank {
    accounts: Arc<Mutex<Vec<i32>>>,
}

impl Bank {
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(100); // Give them some initial balance so it's not always negative
        }
        Bank {
            accounts: Arc::new(Mutex::new(v)),
        }
    }

    pub fn transfer(&self, from: usize, to: usize, amount: i32) -> Result<(), ()> {
        let mut accounts = self.accounts.lock().unwrap();
        let len = accounts.len();

        if from >= len || to >= len {
            return Err(());
        }

        accounts[from] -= amount;
        accounts[to] += amount;

        println!("Amount of ${} transferred from account id: {} to account id: {}.", amount, from, to);
        Ok(())
    }
}

struct Person {
    ac_id: usize,
    buddy_id: usize,
}

impl Person {
    pub fn new(id: usize, b_id: usize) -> Self {
        Person {
            ac_id: id,
            buddy_id: b_id,
        }
    }
}

fn main() {
    let num_accounts = 20;
    let bank = Bank::new(num_accounts);
    
    let mut handles = vec![];
    
    for i in 0..10 {
        let bank_clone = bank.clone();
        // Each person i transfers to person i+1 (mod num_accounts)
        let person = Person::new(i, (i + 1) % num_accounts);
        let handle = thread::spawn(move || {
            // Making a payment to buddy_id
            let _ = bank_clone.transfer(person.ac_id, person.buddy_id, 33);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All transfers completed.");
    println!("Final bank state: {:?}", bank);
}
