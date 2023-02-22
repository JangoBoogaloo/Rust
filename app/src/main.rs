#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let m = Message::Quit;
    m.call();

    let m = Message::Move {
        x:1,
        y:2
    };
    m.call();
}