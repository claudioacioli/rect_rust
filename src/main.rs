#[derive(Debug)] //for prints like {:?} and {:#?} to help the debug
struct Rectangle {
    length: u16,
    width: u16
}

impl Rectangle {
    fn area (&self) -> u16 {
        self.width * self.length
    }
}

fn main() {
    let rect:Rectangle = build_rect(50, 30);
    println!("{:?}", rect);
    println!("{:#?}", rect);
}

fn build_rect(length:u16, width:u16) -> Rectangle {
    Rectangle {
        length,
        width
    }
}