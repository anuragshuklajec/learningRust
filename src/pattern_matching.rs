enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
}

fn main(){
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0,6.0);
    let area_of_rectangle  = calculate_area(rectangle);
    println!("{}", area_of_rectangle);
}


fn calculate_area(shape : Shape)->f64{
    // calculate area of the shape
    // use pattern matching
    let ans = match shape {
        Shape::Circle(radius)=> 3.14 * radius * radius,
        Shape::Square(side_length) => side_length * side_length,
        Shape:: Rectangle(width,height) => width * height,
    };
    return ans;
}
