fn main() {
    let rect:(u16, u16) = (50, 30);

    println!(
        "The area of rectangle is {} square pixels.",
        area(&rect)
    );
}

fn area (rect: &(u16, u16)) -> u16 {
    rect.0 * rect.1
}