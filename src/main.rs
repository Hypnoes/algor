use std::thread;

mod lib;

fn main() {
    let n = 100;
    let k = 50;
    let p = 0.25;

    let p1 = thread::spawn(move || {
        let r1 = lib::binomail::binomial(n, k, p);
        println!("r1: {} form thread 1", r1);
    });

    let p2 = thread::spawn(move || {
        let r2 = lib::binomail::binomial_unrecursive(n, k, p);
        println!("r1: {} form thread 1", r2);
    });

    p1.join().unwrap();
    p2.join().unwrap();

    println!("Thread Main Exit.");
}
