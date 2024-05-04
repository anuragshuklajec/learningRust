// errors are handled in rust using result enum

// enum Result<A,B>{
//     Ok(A),
//     Err(B),
// } rust has it's own implementation of result which looks like this

use std::fs;
fn main(){
    let res  = fs::read_to_string("example.txt");
    match res{
        Ok(content) => {
            println!("File Content : {}",content);
        },
        Err(err) => {
            println!("Error : {}", err);
        }
    }
}