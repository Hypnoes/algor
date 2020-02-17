use std::ops::{Add, Index, Mul, Sub};

struct Matrix<T> {
    containner: Box<Vec<Vec<T>>>,
    size: (usize, usize),
}

impl std::str::FromStr for Matrix<i8> {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_out: Vec<Vec<i8>> = s
            .split(';')
            .map(|i| i.split(',').map(|j| i8::from_str(j).unwrap()).collect())
            .collect();

        let n = parse_out.len();
        let m = parse_out.iter().map(|x| x.len()).max().unwrap();

        let mat = Matrix::<i8> {
            containner: Box::new(parse_out),
            size: (m, n),
        };

        return Ok(mat);
    }
}

impl std::str::FromStr for Matrix<i16> {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_out: Vec<Vec<i16>> = s
            .split(';')
            .map(|i| i.split(',').map(|j| i16::from_str(j).unwrap()).collect())
            .collect();

        let n = parse_out.len();
        let m = parse_out.iter().map(|x| x.len()).max().unwrap();

        let mat = Matrix::<i16> {
            containner: Box::new(parse_out),
            size: (m, n),
        };

        return Ok(mat);
    }
}

impl std::str::FromStr for Matrix<i32> {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_out: Vec<Vec<i32>> = s
            .split(';')
            .map(|i| i.split(',').map(|j| i32::from_str(j).unwrap()).collect())
            .collect();

        let n = parse_out.len();
        let m = parse_out.iter().map(|x| x.len()).max().unwrap();

        let mat = Matrix::<i32> {
            containner: Box::new(parse_out),
            size: (m, n),
        };

        return Ok(mat);
    }
}

impl std::str::FromStr for Matrix<i64> {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_out: Vec<Vec<i64>> = s
            .split(';')
            .map(|i| i.split(',').map(|j| i64::from_str(j).unwrap()).collect())
            .collect();

        let n = parse_out.len();
        let m = parse_out.iter().map(|x| x.len()).max().unwrap();

        let mat = Matrix::<i64> {
            containner: Box::new(parse_out),
            size: (m, n),
        };

        return Ok(mat);
    }
}

impl std::str::FromStr for Matrix<i128> {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_out: Vec<Vec<i128>> = s
            .split(';')
            .map(|i| i.split(',').map(|j| i128::from_str(j).unwrap()).collect())
            .collect();

        let n = parse_out.len();
        let m = parse_out.iter().map(|x| x.len()).max().unwrap();

        let mat = Matrix::<i128> {
            containner: Box::new(parse_out),
            size: (m, n),
        };

        return Ok(mat);
    }
}
