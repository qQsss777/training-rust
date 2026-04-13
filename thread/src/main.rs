use std::env;
use std::{
    thread::{self},
    time::Duration,
};

fn multi_thread() {
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

fn default() {
    let handler = thread::spawn(move || {
        println!("Hello from other thread");
    });
    println!("Hello from main thread");
    handler.join().unwrap();
    println!("Hello from main thread second time");
}

fn attach_thread() {
    thread::spawn(move || {
        println!("Hello from other thread");
    });
    println!("Hello from main thread");
    println!("Hello from main thread");
    println!("Hello from main thread");

    // si pas de sleep, le programme se termine car le thread est détaché
    thread::sleep(Duration::from_secs(3));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let value: &str = args[1].as_str();
            match value {
                "multi" => multi_thread(),
                "attach" => attach_thread(),
                _ => {
                    default();
                }
            };
        }
        _ => default(),
    }
}
