
#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize(u32,u32),
    Move{x: i32,y: i32},
    Echo(String),
    ChangeColor(u8,u8,u8),
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize(300,200));
    println!("{:?}", Message::Move{x:10,y:10});
    println!("{:?}", Message::Echo("Hamdaaa".to_string()));
    println!("{:?}", Message::ChangeColor(255, 0, 0));
    println!("{:?}", Message::Quit);
}