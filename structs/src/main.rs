struct rectangle  {
    width:u32,
    height:u32

}

fn main() {
    let rect = rectangle { width: 10, height: 30};

    let area = calculate_area(&rect);
    println!("Area is:: {}", area)
}

fn calculate_area(dimensions: &rectangle) -> u32 {
    dimensions.width * dimensions.height
}
