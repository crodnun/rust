fn return_statement_function(value: i32) -> i32 {
    if value > 10 {
        return value;
    } 

    value + 1
}


// IMPORTANT!!!!!!
// RUST functions only can return a single value (one or none)
fn add_numbers_return_value(x: i32, y: i32) -> i32 {

	// when we return a value in a function, it is the last statement of the branch and has no ";"
        // following line generates and error
        // error[E0308]: mismatched types
        // x + y;

        // this is the right method to return a value
	x + y
}

// We have to declare always the type of parameters, following line generates error
// error: expected one of `:` or `@`, found `,`
//fn add_numbers(x, y) {
fn add_numbers(x: i32, y: i32) {
	println!("Result of adding {} and {} is {}", x, y, x+y);
}

// There are special functions at RUST that never return - "diverge"
// To indicate the same we write that it returns a ! type
fn diverges() -> ! {
    // an special macro that does this is panic
    panic!("This function diverges!!!")
}

fn main() {
    println!("Hello functions !!!");

    let a: i32 = 10;
    let b: i32 = 20;

    // calling function and passing parameters
    add_numbers(a, b);

    let result: i32 = add_numbers_return_value(a, b);

    println!("Result of adding with return the values {} and {} is {}", a, b, result);


    // running a function that has intermediate return statements
    return_statement_function(b); 

    // call a diverging function
    diverges();
}
