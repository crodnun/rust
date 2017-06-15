fn main() {
    println!("Hello, types!");
    /* basic types in RUST */
    let a: bool = true;
    let b: char = 'a';

    println!("Bool {}, char {}", a, b);

    /* numeric types in RUST */
    let num1: i32  = 10; // i8, i16, i32, i64
    let num2: u64 = 100; // u8, u16, u32, u64
    let numf: f64 = 0.0002; // f32/f64 

    println!("INT {}, UINT {} FLOAT {}", num1, num2, numf);
  
    /* variable size values */
    let var1: isize = 10000;
    let var2: usize = 38382382;

    println!("ISIZE {}, USIZE {}", var1, var2);
}
