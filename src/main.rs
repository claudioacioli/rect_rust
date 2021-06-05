fn main() {
    let length: u32 = 50;
    let width: u32 = 30;

    println!(
        "The area of rectangle is {} square pixels.",
        area(&length, &width)
    );
}

fn area (length: &u32, width: &u32) -> u32 {
    length * width
}