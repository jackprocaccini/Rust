fn main() {
    let mut count = 0u32;

    // Infinite loop
    loop {

        count += 1;
        if count == 3 {
            println!("three");
            continue; // Skip the rest of this iteration
        }

        println!("{}", count); // if count is equal to 3, this line (and subsiquent lines) will not be run

        if count == 5 {
            println!("OK, that's enough");
            break; //exit the loop
        }
    }
}