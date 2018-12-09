#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32

}

impl Rectangle {
    fn calculate_area(&self) -> u32{
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 13, height: 30};

    println!("Area is:: {:?}", rect.calculate_area())
}


