fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}");
}