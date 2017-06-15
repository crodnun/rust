fn main() {
    // RUST is a language based on expressions not in statements
    // This is the reason why ; is not used when returning a value from a function

    // Rust is primarily an expression-based language. There are only two kinds of statements, and everything else is an expression.
    // So what's the difference? Expressions return a value, and statements do not.

    // There are two kinds of statements in Rust: ‘declaration statements’ and ‘expression statements’

    // This is a declaration
    let mut name = "carlos";
    let mut number = 3;

    // This is an expression, we assign the result to variable
    number = 1 + 2;

    println!("Value of number is {}", number);
}
