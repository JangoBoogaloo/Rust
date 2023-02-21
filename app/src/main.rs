fn main() {
    let s = String::from("hello");
    borrow(&s);
    borrowAndCHange(&s);
}

fn borrow(some_string: &String) {
    println!("{some_string}");
}

fn borrowAndCHange(some_string: &String) {
    some_string.push_str(", world");
}

