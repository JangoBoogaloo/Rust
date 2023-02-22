#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    //associate functions
    fn area(&self) -> u32 {
        return self.width*self.height;
    }

    //associate functions: like a constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {

    //associate functions
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle::square(30);

    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    dbg!(&rect2);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );
}