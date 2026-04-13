use std::{
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    loop {
        for i in 1..8 {
            let _ = thread::Builder::new().name(i.to_string()).spawn(move || {
                for j in 1..10 {
                    println!("hi number {j} from the spawned thread {i}!");
                    thread::sleep(Duration::from_secs(10));
                }
            });
        }
        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_secs(10));
        }
    }
}
