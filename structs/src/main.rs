fn main() {
    let rect = (10,20);

    let area = calculate_area(rect);
    println!("Area is:: {}", area)
}

fn calculate_area(dimensions:(i32,i32)) -> i32{
    dimensions.0 * dimensions.1
}
