fn binary_search(key: isize, a: &[isize]) -> isize {
    let mut lo = 0;
    let mut hi = a.length;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if key < a[mid] {
            hi = mid;
        } else if key > a[mid] {
            lo = mid;
        } else {
            return a[mid];
        }
    }
    return -1;
}

fn binary_search_r(key: isize, a: &[isize]) -> isize {
    let lo = 0;
    let hi = a.len();
    if hi == 0 {
        return -1;
    }

    mid = hi / 2;
    if key > a[mid] {
        binary_search_r(key, a[mid..hi])
    } else if key < a[mid] {
        binary_search_r(key, a[lo..mid])
    } else {
        return a[mid];
    }
}
