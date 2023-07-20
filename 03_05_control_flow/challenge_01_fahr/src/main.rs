fn main() {
    let mut celsius: f64 = -50.0;
    while celsius <= 50.0 {
        let mut fahrenheit: f64 = (celsius * (9.0/5.0)) + 32.0;
        println!("{celsius} Celsius = {fahrenheit} Fahrenheit");
        celsius += 5.0;
    }

}
