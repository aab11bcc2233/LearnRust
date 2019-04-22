fn main() {
    let mut fraction = [12, 35, 99, 18, 76];

    let n = fraction.len();

    println!("fraction = {:?}", fraction);

    for i in 1..n {
        for j in 0..n - i {
            if fraction[j] > fraction[j + 1] {
                let t = fraction[j];
                fraction[j] = fraction[j + 1];
                fraction[j + 1] = t;
            }
        }
    }

    println!("sort fraction = {:?}", fraction);
}
