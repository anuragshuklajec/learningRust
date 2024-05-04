// Rust doesn't has the concept of none so we can use option enum in case we ever need to return none

// enum Option<T>{
//     Some(T),
//     None
// } Rust has this inbuilt we dont need to create

fn find_first_a(s:String) -> Option<usize>{
    for (idx,char) in s.chars().enumerate(){
        if char == 'a' {
            return Some(idx);
        }
    }
    return None;
}

fn main(){
    let my_string = String :: from("anurag");
    let res = find_first_a(my_string);
    match res{
        Some(index) => println!("The letter a was first found in index : {}", index),
        None => println!("The letter a is not present in the string"),
    };
}