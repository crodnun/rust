fn main() {
    println!("Hello loops!");

    // RUST has 3 different loop statements

    // infinite loop
    // comment-out this block if you want to see an infinite loop - Ctrl+C to abort it
    /*
    loop {
	println!("Running an infinite loop!");
    } */

    // undetermined loop - we do not know how many iterations
    let mut done = false;
    let mut counter: i32 = 0;

    while !done {
	counter += 1;	
        if counter > 100 {
	    done = true;
        }
        println!("Counter value is {}", counter);
    }

    // fixed number of iterations loop - IMPORTANT: This has nothing to do with C for(;;)
    // It follows this syntax 
    /*
       for var in expression {
          code
       }
    */

    for var in 1..10 {
        println!("Doing for loop with value {}", var);
    }

    // Ending loops - continue or break - when nested loops we can use also labels to identify the loop that we want to abort
    // we can also use the return statement if we want to stop a function completely - not a good practice
    counter = 0;
    loop {
        counter += 1;
        if counter < 10 {
            continue; // we skip printing this value
        } else if counter == 120 {
            break; // we stop at 120 value
        }

    
        println!("Counter value is {}", counter);
    }

    // Test that we can stop a loop with return inside a function only

    // we can define labels at loops
    'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
        if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
        println!("x: {}, y: {}", x, y);
    }

    // When you need to keep track of how many times you have already looped, you can use the 
    // .enumerate() function.
    // ???? Complex to understand ?? It uses a tuple and parenthesis to get the tuple
    for (index, value) in (5..10).enumerate() {
    	println!("index = {} and value = {}", index, value);
    }
}

}
