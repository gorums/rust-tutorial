#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }   
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels using tuple.",
        area_tuple(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels using struct.",
        area_struct(&rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels using method.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("rect1 is {rect1:?} without format");
    println!("rect1 is {rect1:#?} with format");

    dbg!(&rect1); // debug mode

    let rect4 = Rectangle::square(20);
    println!("square area of rect4 {}", rect4.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//we want to borrow the struct rather than take ownership of it
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}