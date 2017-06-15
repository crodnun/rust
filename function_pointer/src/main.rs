fn do_something_with_i32(input: i32) -> i32 {
        let result = input + 2;
	println!("Adding 2 to input number {}: result is {}", input, result);
	result
}

fn main() {
    println!("Hello function pointers!");

    // RUST supports function pointers, we define them this way
    let myfunct: fn(i32) -> i32 = do_something_with_i32;
    myfunct(5);
}
