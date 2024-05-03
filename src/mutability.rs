fn main() {
    // let i : i32 = 5;
    // i = 6 ;  // Every variable in rust is immutable so far so, assigning this a new value won't let it compile

    // Instead convert it to a mutable first
    let mut j : i32 = 10;
    j = 5;
    println!("{}", j);
}