#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write (String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
    let q = Message::Quit;
    q.call();
    let mv = Message::Move{x:10, y:20};
    mv.call();
    let cc = Message::ChangeColor(10, 20, 30);
    cc.call();
}
