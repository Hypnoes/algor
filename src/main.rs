fn main() {
    let mut i = [[0; 3]; 3];
    for m in 0..3 {
        for n in 0..3 {
            if m == n {
                i[m][n] = 1;
            } else {
                i[m][n] = 0;
            }
        }
    }

    for m in 0..3 {
        for n in 0..3 {
            print!(" {}", i[m][n]);
        }
        println!("");
    }
}
