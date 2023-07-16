fn main() {
    let number = 7;

    // Note you can't just say "if number", as number is an integer, not a boolean.
    // So "truthiness" doesn't work in the same way as javascript
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;

    if number !=0 {
        println!("number was something other than zero");
    }
}
