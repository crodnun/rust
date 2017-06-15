fn main() {
    println!("Hello vectors!");

    // A vector is a growable array

    // Declaration - macro vec![]
    let my_first_vector = vec![1, 2, 3, 4, 5, 6];

    // We can initialize size and value
    // vector with 10 zeroes
    let other_vector = vec![0; 10];


    // indexing must be done with usize variables
    let i: usize = 0;

    let v = vec![1, 2, 3];

    // Works:
    v[i];

    // Doesn’t: it errors out:  cannot be indexed by `i32`
    // let j: i32 = 0;
    //v[j];

    // access specific elements, it starts at index 0 - (N-1)
    println!("The second item in vector is {}", my_first_vector[1]);

    // out of bounds errors - default behaviour is to panic
    // thread 'main' panicked at 'index out of bounds: the len is 6 but the index is 10', /checkout/src/libcollections/vec.rs:1422
    // comment this line out to see the panic/error
    // println!("This is an out of bounds error {}", my_first_vector[10]);

    // If we want to avoid out of bounds we can use safe methods that return the value None when not found
    let mut safe_vector = vec![1, 2, 3];
    
    // match == kind of switch
    match safe_vector.get(10) {
        // Can return some value or None to let us decide what to do next
	Some(x) => println!("Item 10 is {}", x),
        None => println!("Sorry, this vector is too short has no item at position 10.") 
    }

    // iterate it with basic loop
    for index in 0..5 {
	println!("Item is {}", my_first_vector[index]);
        println!("Other item is {}", other_vector[index]);
    }

    // loops must be done this way - otherwise we modify the vector and can not be used later
    // 1) We can pass an unmutable reference to the vector
    for item in &safe_vector {
        println!("Iterate vector reference at item {}", item);
    }
    // 2) We can pass a mutable reference
    for item in &mut safe_vector {
        println!("Iterate mutable vector reference at item {}", item);
    }
    // 3) We can take ownership of the vector - this type does not allow us to use it later - discouraged
    for item in safe_vector {
        println!("Iterate vector taking ownership at item {}", item);
    }

    // This line will fail once we did a loop taking ownership
    // Note: You cannot use the vector again once you have iterated by taking ownership of the vector
    // Comment out this line to generate error: error[E0382]: use of moved value: `safe_vector`
    // safe_vector[0];
}
