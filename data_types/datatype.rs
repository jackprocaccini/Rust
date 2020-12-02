fn main() {
    let a: u8 = 10; // unsigned, 8 bit integer. Stores any number from 0 to 255
    let b: i16 = -50; // signed, 16 bit integer. Stores any number from -32769 to 65535
    let c = 20; // no explicit type or size, so the compiler will default to a signed 32 bit integer

    let x = 2.0; // f64. Implicit declartion to a 64-bit floating-point number
    let y: f32 = 3.0; // f32. Explicit declaration of a 32-bit floating-point number

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1); // explicit tuple
    let tup = (500, 6.4, 1); // implicit tuple
    let (x, y, z) = tup; // turns 'tup' into three separate values: x, y and z

    println!("The value of y is: {}", y);

    // getting a specific value out of a tuple
    let x: (i32, f64, u8) = (500, 6.4, 1); // explicit declaration. First index must contain a signed 32-bit int,
    let five_hundred = x.0;                // second must be 64-bit float, third must be an unsigned 8-bit int
    let six_point_four = x.1;
    let one = x.2;

    let a = [1, 2, 3, 4, 5];
    let a = [3; 5]; // same thing as writing "let a = [3, 3, 3, 3, 3]"
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // getting a specific value out of an array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}