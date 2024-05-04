fn main(){
    let s1 = String::from("Hello");
    // let s2 = s1 ;
    // println!("{:?}", s1) ; // this will result in error since s1 is no longer present in memory and the ownership is transferred to s2

    /*  We can use borrowing to tackle this
    there are few rules for borrowing :
    1. You can have as many immutable reference of a variable
    2. If you create even a single mutable refernce of a variable you can't have more mutable or immutable reference
    */  

    // This example works because each borrowing's scope ends after their respective print statement
    let mut str1 = String::from("Hello");
    let str2 = &str1;
    println!("{}",str2);
    let str3 = &mut str1;
    println!("{}",str3);
    let str4 = &mut str1;
    println!("{}",str4);

    // // This won't work because you are trying to access other reference after a mutable reference
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &mut s;
    // println!("{}", r1);
}
