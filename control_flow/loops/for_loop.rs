fn main(){
    let arr = ["1", "Hello", "Rust", "Is", "Great", "6"];

    for n in 0..arr.len(){ // loop from 0 to the length of the array
        println!("Data at position {}: {}", n, arr[n]);
    }

    for x in 0..101{ // loop from 0 to 100
        println!("{}", x);
    }
}
