fn main() {
    println!("Hello conditional statements!");

    let done: bool = false;

    if done {
        println!("We are done!");
    } else {
        println!("We are NOT done!");
    }


    let counter = 5;

    if counter == 1 {
	println!("counter is 1!");
    } else if counter > 5 {
        println!("counter is higher than 5!");
    } else {
	println!("counter is in [2-5] range!");
    }

    // We can also if statements to assign values to variables, this way
    let x = 5;

    let mut y = if x == 5 {
    	10
    } else {
    	15
    }; // y: i32

    println!("The value of y is {}", y);

    // shortcut to do the same in a single line
    y = if x == 5 { 10 } else { 15 }; // y: i32

    println!("The value of y is {}", y);
}
