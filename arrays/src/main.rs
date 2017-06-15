fn main() {
    println!("Hello arrays!");

    let mut my_array = [1, 2, 3, 4];
    my_array[0] = 0;

    println!("My array has {} elements", my_array.len());

    // Define array using its type
    let other_array: [i32; 3] = [0;3];

    println!("This array has {} elements", other_array.len());

    // slices are just portions of arrays
    let total_slice = &other_array[..];
    println!("This slice has {} elements", total_slice.len()); 

    let slice = &other_array[2..3];
    println!("This slice has {} elements", slice.len());
}
