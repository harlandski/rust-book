use std::io;

fn main() {
    loop {
        println!("Enter n to generate the nth Fibonacci number:");

        let mut sum = 0;
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n :u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n == 1 {
            println!("0 is the 1 Fibonacci number");
        }

        else if n == 2 {
            println!("1 is the 2 Fibonacci number");
        }
        else {
            let iterations = n-1;
            let mut first_number = 0;
            let mut second_number = 1;
            for _num in 1..iterations {
                sum = first_number + second_number;
                first_number = second_number;
                second_number = sum;
            }
        println!("{sum} is the {n} Fibonacci number");
        }

    }
}
