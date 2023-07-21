use std::io;

fn main() {
loop {
    
println!("Enter n to generate the nth Fibonacci number:");

    let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut sum = 1;

        for num in 1..n {
            sum += num;
        }

        println!("{sum} is the {n} Fibonacci number");
}
}