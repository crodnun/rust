fn main() {
    println!("Hello tuples!");

    // A tuple is an ordered list of elements with a fixed size (elements can have different types)
    let my_tuple = (1, "Hello");

    println!("My tuple first field is {}", my_tuple.0);
    println!("My tuple second field is {}", my_tuple.1);
}
