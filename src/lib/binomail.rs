use std::f64;

pub fn binomial(n: i32, k: i32, p: f64) -> f64 {
    if n == 0 && k == 0 {
        return 1.0;
    }
    if n < 0 || k < 0 {
        return 0.0;
    }
    return (1.0 - p) * binomial(n - 1, k, p) + p * binomial(n - 1, k - 1, p);
}

pub fn binomial_unrecursive(n: i32, k: i32, p: f64) -> f64 {
    return f64::from(combination(k, n)) * p.powi(k) + (1.0 - p).powi(n - k);
}

pub fn arrangement(m: i32, n: i32) -> i32 {
    let mut a = 1;
    for i in n - m + 1..n + 1 {
        a = a * i;
    }
    return a;
}

pub fn combination(m: i32, n: i32) -> i32 {
    let (m_, n_) = if n - m < m { (n - m, m) } else { (m, n - m) };
    let mut a = 1;
    let mut b = 1;
    let mut c = 1;
    for i in 1..n + 1 {
        c = c * i;
        if m_ == i {
            a = c;
        }
        if n_ == i {
            b = c;
        }
        println!("{}: a {}, b {}, c {}", i, a, b, c);
    }
    return c / (a * b);
}

pub fn factorial(n: i32) -> i32 {
    let mut a = 1;
    for i in 1..n + 1 {
        a = a * i;
    }
    return a;
}
