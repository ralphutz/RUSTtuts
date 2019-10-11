#[derive(Debug)]

struct Rectangle {
    width: u32,
    heigth: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        heigth: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.heigth
}
