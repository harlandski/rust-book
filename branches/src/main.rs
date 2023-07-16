fn main() {
    let number = 9;

    // Note you can't just say "if number", as number is an integer, not a boolean.
    // So "truthiness" doesn't work in the same way as javascript
    if number < 5 {
        println!("{number} is less than five");
    } else {
        println!("{number} is not less than five");
    }

    if number !=0 {
        println!("{number} is something other than zero");
    }

    if number %4 == 0 {
        println!("{number} is divisible by 4");
    } else if {number} % 3 == 0 {
        println!("{number} is divisible by 3");
    } else if {number} % 2 == 0 {
        println!("{number} is divisible by 2");
    } else {
        println!("{number} is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is: {number}");

}
