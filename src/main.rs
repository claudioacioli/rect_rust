#[derive(Debug)] //for prints like {:?} and {:#?} to help the debug
struct Rectangle {
    length: u16,
    width: u16
}

impl Rectangle {
    fn area (&self) -> u16 {
        self.width * self.length
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.length > rect.length && self.width > rect.width
    }
}

fn main() {
    let rect_a: Rectangle = build_rect(50, 30);
    let rect_b: Rectangle = build_rect(10, 20);
    let rect_c: Rectangle = build_rect(80, 10);

    println!(
        "The area of rectangle is {} square pixels.\n\
        Can hold rect_b in react_a: {}\n\
        Can hold rect_c in rect_a: {}",
        rect_a.area(),
        rect_a.can_hold(&rect_b),
        rect_a.can_hold(&rect_c)
    );
}

fn build_rect(length:u16, width:u16) -> Rectangle {
    Rectangle {
        length,
        width
    }
}