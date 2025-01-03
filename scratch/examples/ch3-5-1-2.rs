use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(100_000));

    let x1_clone = Arc::clone(&x);
    let h1 = std::thread::spawn(move || {
        let mut guard = x1_clone.lock().unwrap();
        *guard -= 20_000;
    });

    let x2_clone = x.clone();
    let h2 = std::thread::spawn(move || {
        let mut guard = x2_clone.lock().unwrap();
        *guard -= 10_000;
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("{}", x.lock().unwrap());
    {
        let x = x.clone();
        let x = x.clone();
        let gurad1 = x.lock().unwrap();
        println!("{}", *gurad1);
    }
    {
        let guard2 = x.lock().unwrap();
        println!("{}", *guard2);
    }
}
