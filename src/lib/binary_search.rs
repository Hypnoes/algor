use std::result::Result;

fn binary_search(key: usize, a: &[usize]) -> Result<usize, &'static str> {
    let mut lo = 0;
    let mut hi = a.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if key < a[mid] {
            hi = mid;
        } else if key > a[mid] {
            lo = mid;
        } else {
            return Ok(a[mid]);
        }
    }
    return Err("Error");
}

fn binary_search_r(key: usize, a: &[usize]) -> Result<usize, &'static str> {
    let lo = 0;
    let hi = a.len();
    if hi == 0 {
        return Err("Error");
    }

    let mid = hi / 2;
    if key > a[mid] {
        binary_search_r(key, &a[mid..hi])
    } else if key < a[mid] {
        binary_search_r(key, &a[lo..mid])
    } else {
        return Ok(a[mid]);
    }
}
