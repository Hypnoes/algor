use std::ops::Add;

struct Matrix {
    containner: Vec<Elem>,
    size: (usize, usize)
}

impl Matrix {
    pub fn new(m: usize, n: usize, e: Elem) -> Matrix {

    }
}

struct Elem<T> {
    e: T,
    position: (usize, usize)
}