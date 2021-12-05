#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32

}

impl Rectangle {
    fn calculate_area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool{
        let te1 = self.calculate_area();
        let te2 = rect.calculate_area();
        if te1 > te2 {
            true
        }
        else{
            false
        }
    }

    fn create_square(size:u32) -> Rectangle{
        Rectangle {width:size, height:size}
    }
}

fn main() {
    let rect1 = Rectangle { width: 100, height: 100};
    let rect2 = Rectangle { width: 500, height: 100};

//    println!("Area is:: {:?}", rect.calculate_area());
    //rect2.width uses copy ?
    let rect3 = Rectangle::create_square(rect2.width);
    println!("Can hold:: {:?}", rect1.can_hold(&rect2));

    println!("Square size:: {:?}", rect3)
}


