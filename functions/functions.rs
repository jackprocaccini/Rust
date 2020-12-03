/* 
 * Rust does not care *where* a function is defined,
 * only that the function defined
 */
fn another_function(){
    println!("Another function")
}

fn main(){ // must have a main function if you want to run your Rust code
    println!("Hello World!");

    another_function();
    function_two(3);
    multi_param_function(10, 20);
}

fn function_two(num: i32){ // MUST declare the type of each parameter in function signature
    println!("The i32 number {} was passed into this function", num);
}

fn multi_param_function(x: i32, y: i32){ // multiple parameters are separated by a comma
    println!("You passed in {} and {}", x, y);
}