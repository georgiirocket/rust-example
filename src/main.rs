struct Rectangle {
    width: u32,
    height: u32,

}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn product(&mut self, number: u32) {
        self.width *= number;
        self.height *= number;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height, }
    }
}


fn main() {
    let mut  rectangle = Rectangle { width: 30, height: 50 };
    rectangle.product(2);

    let rect2 = Rectangle::new(10, 40);

    println!("Can hold: {}", rect2.can_hold(&rectangle));
    println!("Area: {}", rectangle.area());
}
