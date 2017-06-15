fn main() {
    println!("Hello, using variables!");

    // This is a comment in RUST
    /*  Multiline comment 
    */

    // RUST use statically typed variables but inferes values when possible 
    let (x, y) = (100, 2);
    // If we want to identify the type at binding we use this format
    let z: i32 = 3;

    // binding creates unmutable variables, we can not change their value by default
    // These statements generate compilation error
    // error[E0384]: re-assignment of immutable variable `my_int`
    //let my_int = 3;
    //my_int = 8;


    // If we want to make a variable mutable we must indicate the same using mut reserved word
    // This is done for security - we indicate those variables that really want to modify
    let mut my_var = 3;
    my_var = 10;

    let my_string = "carlos";

    // If we want to print the value of a variable we use the place holder {} to be replaced by a variable value
    println!("The value of x is: {}, and my name is {}", x, my_string);

    // RUST does not allow to use unitialized variables, it does not initialize them by default
    let a: i32;
    // error[E0381]: use of possibly uninitialized variable: `a`
    //println!("The value of a is: {}", a); 

    // Scope of variables, variables are seen inside the block they are defined as happens at most of the languages
    // We can shadow variables with latest value defined
    let number: i32 = 2;
    // we initiate here a block
    {
	let mut number = 7;
        let mut local_var = 8;

        println!("The value of number is: {}, and local_var is {}", number, local_var);
    }

    // But we can not see variables outside its scope
    println!("The value of number is: {}", number);

    // this generates an error - we are out of the scope and can not see this variable
    // error[E0425]: cannot find value `local_var` in this scope
    // println!("The value of number is: {}, and local_var is {}", number, local_var);
}
