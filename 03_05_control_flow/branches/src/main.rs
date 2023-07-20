fn main() {
    let number = 9;

    // Note you can't just say "if number", as number is an integer, not a boolean.
    // So "truthiness" doesn't work in the same way as javascript
    if number < 5 {
        println!("{number} is less than five");
    } else {
        println!("{number} is not less than five");
    }

    if number != 0 {
        println!("{number} is something other than zero");
    }

    if number % 4 == 0 {
        println!("{number} is divisible by 4");
    } else if { number } % 3 == 0 {
        println!("{number} is divisible by 3");
    } else if { number } % 2 == 0 {
        println!("{number} is divisible by 2");
    } else {
        println!("{number} is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Commenting this out as it repeats forever
    //loop {
    //    println!("Repeat!");
    //}

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {result}.");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // The following is simpler, and less error-prone:
    // Note this is a JavaScript "false friend"
    // for ... in in JavaScript gives the index
    // Rust for ... in is like JavaScript for ... of

    for element in a {
        println!("the value is: {element}");
    }

    // Note that the last item of range is exclusive
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        println!("x: {x}");
        sum += x;
    }
    println!("{sum}");
}
