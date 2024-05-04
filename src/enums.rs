// Enums in rust are similar to enums in typescript. They allow you to define a type by enumeratiing its possible  variants

enum Direction {
    North,
    East,
    West,
    South,
}

// Some enums might have data attached to them
enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
}

fn main(){
    let my_direction = Direction::North;
    move_around(my_direction);

    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0,6.0);
}

fn move_around(direction : Direction) -> (){
    // implements logic to move a character around
}

fn calculate_area(shape : Shape)->f64{
    // calculate area of the shape
    // use pattern matching
    return 0.00;
}

// Rather than storing String in types and limiting what can be passed in function we can use Enums so that we can add more strictness