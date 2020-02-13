use std::fmt::Display;
use std::ops::{Add, Fn, Index, Mul};

struct Matrix<T> {
    part: Box<T>,
    size: (u32, u32),
}

impl<T> Matrix<T> {
    pub fn new(m: u32, n: u32) -> Matrix<T> {
        return matrix {
            part: Box::new([[T; m]; n]),
            size: (m, n),
        };
    }
}

impl<T> Matrix<T> {
    fn get_elem(&self, i: (usize, usize)) -> T {
        let (m, n) = i;
        return self.part[m][n];
    }

    fn get_row(&self, m: usize) -> [T; m] {
        let (m, n) = self.size;
        let mut a: [T; n];
        for i in 0..n {
            a[i] = self.part[m][i]
        }
        return a;
    }

    fn get_col(&self, n: usize) -> [T; n] {
        let (m, n) = self.size;
        let mut a: [T; m];
        for i in 0..m {
            a[i] = self.part[i][n];
        }
        return a;
    }
}

impl<T> Matrix<T> {
    pub fn dot(&self) -> Matrix<T>{

    }
    
    pub fn mul() -> Matrix<T>{

    }
    
    pub fn mul() -> Matrix<T>{

    }
    
    pub fn mul() -> Matrix<T>{

    }
    
    pub fn T(&self) -> Matrix<T> {

    }
}

impl<T> Index for Matrix<T> {
    type Output = Matrix<T>;

    pub fn index(&self, i: (Option(usize), Option(usize))) -> Result<T, E> {
        if i == (None, None) { return E; }
        match i {
            i.0 == None => { return OK(get_col(i.1.unwrap())); },
            i.1 == None => { return OK(get_row(i.0.unwrap())); },
            _   => { return OK(get_elem(i.0.unwrap(), i.1.unwrap())); }
        }
    }
}

impl<T> Add for Matrix<T> {
    type Output = Matrix<T>;

    pub fn add(&self, b: Matrix) -> Matrix {}
}
