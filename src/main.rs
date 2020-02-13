mod lib;

fn main() {
    let n = 100;
    let k = 50;
    let p = 0.25;

    let r = lib::binomail::binomial(n, k, p);
    println!("{}", r);
}
