#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    update(&mut rect1, 40, 20);
    println!("{:#?}",rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    //debug print
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn update(rectangle: &mut Rectangle, width: u32, height: u32) {
    rectangle.width = width;
    rectangle.height = height;
}