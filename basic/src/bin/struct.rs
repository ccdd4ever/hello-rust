struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn perimeter(&self) -> u32 {
        2 * (self.length + self.width)
    }
    fn new(width: u32, length: u32) -> Rectangle {
        Rectangle { width, length }
    }
}

fn main() {
    let rect1 = Rectangle::new(10, 20);
    println!("area: {}, perimeter:{}", rect1.area(), rect1.perimeter())
}
