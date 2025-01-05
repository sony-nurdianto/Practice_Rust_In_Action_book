use std::{thread, time};

fn main() {
    let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(2);
    let pause = time::Duration::from_millis(20);

    for _ in 0..2 {
        let handle = thread::spawn(move || thread::sleep(pause));
        handlers.push(handle)
    }

    while let Some(handle) = handlers.pop() {
        let _ = handle.join();
    }
}
