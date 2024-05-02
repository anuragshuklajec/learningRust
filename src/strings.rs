fn main(){
    let greetings = String::from("Hi from Anurag");
    println!("{}", greetings);

    // You can't access index of strings easily because it might exist or doesn't exists
    let char1 = greetings.chars().nth(1);
    // println!("char1 : {}", char1); // This will lead to an compilation error

    // Using pattern matching to get along
    match char1{
        Some(c) => println!("{}",c),
        None => println!("No char at that index")
    }

    // Using unwrap method that means assuring the compiler that you'll access within the range (not recommended)
    println!("char1 : {}", char1.unwrap());
}