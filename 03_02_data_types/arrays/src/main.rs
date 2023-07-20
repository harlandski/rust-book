use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("The first element of the array is {:?}",a[0]);

    //Makes an array five long of 3s
    let b = [3; 5];
    println!("Behold, all the threes! {:?}",b);
    
    // Indexing arrays
    let second = a[1];
    let third = a[2];
    println!("The second is {second} and the third is {third}");

    //Dealing with bad indexes
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Faild to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element =a[index];

    println!("The value of the element at index {index} is: {element}");

 }
