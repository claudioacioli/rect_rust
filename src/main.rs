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

    println!(
        "The area of rectangle is {} square pixels.",
        rect.area()
    );
}

fn build_rect(length:u16, width:u16) -> Rectangle {
    Rectangle {
        length,
        width
    }
}