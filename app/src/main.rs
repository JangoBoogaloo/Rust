fn main() {
    let some_number: Option<i32> = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    let efg = absent_number + some_number;
    println!("Hello World");
}