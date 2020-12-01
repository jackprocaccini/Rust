const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Main function");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x); //Should result in an error if x isnt 'mut'
    shadowing();
}

fn shadowing() {
    println!("Shadowing function");
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x); // this is acceptable, since x is being reassigned with 'let'
}