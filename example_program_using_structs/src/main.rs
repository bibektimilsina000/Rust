#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self,rectangle: &Rectangle)->bool{

      self.height>rectangle.height&&self.width>rectangle.width
    }
}

fn main() {
    let height1 = 32;
    let width1 = 30;
    let rect1 = (12, 13);
    let scale = 2;

    let rect2 = Rectangle {
        height: 30,
        width: dbg!(30 * scale),
    };

    println!("{:#?}", rect2);

    println!("area of rectangle is {}", area(height1, width1));
    println!(
        "area of rectangle using its dimension is {}",
        area_uisng_tuple(rect1)
    );
    println!(
        "area of rectangle  using struct is {}",
        area_using_struct(&rect2)
    );
    println!("the area of rectangre is using methods {}", rect2.area());
    dbg!(&rect2);
    println!("rectangle can hold ? {} ",rect2.can_hold(&rect2))
}

fn area(length: u32, breath: u32) -> u32 {
    length * breath
}

fn area_uisng_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
fn area_using_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}


 