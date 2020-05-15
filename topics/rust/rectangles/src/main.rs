mod stuff;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle{ width: 30, height: 50 };
    let rect2 = Rectangle{ width: 20, height: 40 };

    println!("1 can hold 2: {}", rect1.can_hold(&rect2));

    stuff::hello();
}

