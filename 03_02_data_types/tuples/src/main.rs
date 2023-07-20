fn main() {
    let tup: (String, String, u32, u8) = ("James".to_string(), "Harland".to_string(), 1978, 45);

    let (first_name, last_name, birth_year, age) = tup;
    println!("{} {} was born in {} and is now {} years old.", first_name, last_name, birth_year, age);

    let tup2: (String, u32)  = ("Speed of light".to_string(), 299792458);

    println!("The {} is {}.", tup2.0, tup2.1);
}

