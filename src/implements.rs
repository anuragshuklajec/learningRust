// impl lets us add function to structs and make them similar to classes in other languages

struct Rect{
    width : u32,
    height : u32
}

impl Rect {
    fn area(&self) -> u32{
        self.width * self.height  // you can return directly without using return keyword and not adding semicolon
    }

    fn perimeter(&self) -> u32{
        2 * (self.height + self.width)
    }
}

fn main(){
    let rect = Rect{
        width : 30,
        height : 50,
    };
    print!("The area of the rectangle is {} and perimeter is {}",rect.area(), rect.perimeter());
}