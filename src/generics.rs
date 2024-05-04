// Generics are used where there could be multiple data types which could be passed to the given context

struct Point<T>{
    x : T,
    y : T
}

fn main(){
    let integer_point : Point<i32> = Point {x : 5, y : 5};
    let float_point  = Point {x : 10.2, y : 1.8};
    println!("Interger Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
}