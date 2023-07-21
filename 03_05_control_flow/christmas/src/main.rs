fn main() {
   let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
   let gifts = [
    "A partridge in a pear tree!\n",
    "Two turtle doves, and ",
    "Three French hens, ",
    "Four calling birds, ",
    "Five gold rings, ",
    "Six geese a-laying, ",
    "Seven swans a-swimming, ",
    "Eight maids a-milking, ",
    "Nine ladies dancing, ",
    "Ten lords a-leaping, ",
    "Eleven pipers piping, ",
    "Twelve drummers drumming, "];
   
    let mut index = 1;

    for day in days {
        println!("On the {day} day of Christmas my true love gave to me: ");
        for gift in (0..index).rev() {
            println!("{}",gifts[gift]);
        }
        index +=1;
    }
}
