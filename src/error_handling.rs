// errors are handled in rust using result enum

// enum Result<A,B>{
//     Ok(A),
//     Err(B),
// } rust has it's own implementation of result which looks like this

use std::fs;
fn divide_num(num1 : u32, num2 : u32) -> Result<f64, String>{
    if num2 == 0{
        Err("can't divide number with zero".to_string())
    }else{
        let result = (num1 as f64) / (num2 as f64);
        Ok(result)
    }
}
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

    let division = divide_num(1,0);
    match division {
        Ok(res) => {
            println!("Division is {}", res);
        },
        Err(err) => {
            println!("{}",err)
        }
    }
}

