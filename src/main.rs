struct Rectangle {
    length: u16,
    width: u16
}

fn main() {
    let rect:Rectangle = build_rect(50, 30);

    println!(
        "The area of rectangle is {} square pixels.",
        area(&rect)
    );
}

fn area (rect:&Rectangle) -> u16 {
    rect.length * rect.width
}

fn build_rect(length:u16, width:u16) -> Rectangle {
    Rectangle {
        length,
        width
    }
}