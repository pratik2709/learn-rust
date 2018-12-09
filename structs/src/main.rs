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
}

fn main() {
    let rect1 = Rectangle { width: 100, height: 100};
    let rect2 = Rectangle { width: 500, height: 100};

//    println!("Area is:: {:?}", rect.calculate_area());
    println!("Can hold:: {:?}", rect1.can_hold(&rect2));
}


