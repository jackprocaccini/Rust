fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1); // constructing
    let tup = (500, 6.4, 1); // destructing
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // getting a specific value out of a tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // getting a specific value out of an array
    let a = [3; 5]; // same thing as writing "let a = [3, 3, 3, 3, 3]"
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}