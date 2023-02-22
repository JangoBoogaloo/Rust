fn main() {
    let some_number: Option<i32> = Some(5);
    let some_char = Some('e');
    println!("{}", some_number.unwrap());

    let absent_number: Option<i32> = None;
    println!("{}", absent_number.unwrap());
    println!("Hello World");
}