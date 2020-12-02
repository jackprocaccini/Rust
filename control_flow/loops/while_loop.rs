fn main() {
    let mut n = 0;

    while n < 10 { // loops from 0 to 9 and prints out n
        println!("{}", n);
        n -= n;
    }
}
