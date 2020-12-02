fn main() {

}

fn doesnt_work(){
    let a: u8 = 10; // explicit u8
    let b: i16 = 100; // explicit i16
    let c: u8 = a + b; // explicit typing of u8 for 'c', will cause an error, since 'a' is a u8 and 'b' is an i16
    let c = a + b; // implicit typing of 'c', will still cause an error because a and b are different types

    let d = 22; // implicit
    let e: u8 = 10; // explicit
    let f = d + e; // implicit, this WILL compile since the compiler assumes the type of 'd' to be u8 and 'f' to be u8 based on context
}