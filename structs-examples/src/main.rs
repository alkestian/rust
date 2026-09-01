// this is needed to output the debug info, to print the struct
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect2 = Rectangle {
        width: 4,
        height: 4,
    };
    println!("Area of rectangle is {}", area_struct(&rect2));

    // we could also use :#? to output on newlines for readability
    println!("rect2 is {rect2:#?}");

    // or, we could use dbg!() instead to do the same thing
    dbg!(&rect2);
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
