use rand::seq::SliceRandom;
use rand::thread_rng;

use std::env::args;
use std::convert::TryInto;
use std::fmt::Display;

fn main() {
    let m: i32 = args().nth(1).unwrap().parse().unwrap();
    let n: i32 = args().nth(2).unwrap().parse().unwrap();

    let mut v: Vec<Vec<i32>> = Vec::with_capacity(n.try_into().unwrap());
    for _ in 0..n {
        let mut rng = thread_rng();
        let mut mlist: Vec<i32> = (0..m).collect();
        mlist.shuffle(&mut rng);
        v.push(mlist)
    }

    show(&v);

    println!("-----------------");

    let mut g: Vec<Vec<i32>> = Vec::with_capacity(m.try_into().unwrap());
    for _ in 0..m {
        let mut a = Vec::new();
        for _ in 0..m {
            a.push(0);
        }
        g.push(a);
    }

    // g[i][j] == x where x is the number of i in column j

    for i in 0..m {
        for j in 0..m {
            let i_: usize = i.try_into().unwrap();
            let j_: usize = j.try_into().unwrap();
            g[i_][j_] = v.iter()
                       .map(|x| x[j_])
                       .map(|x| if x == i { 1 } else { 0 })
                       .sum::<i32>();
        }
    }

    show(&g);
}

fn show<T: Display>(x: &Vec<Vec<T>>) {
    for i in x.iter() {
        for j in i.iter() {
            print!("{} ", j);
        }
        println!("");
    }
}