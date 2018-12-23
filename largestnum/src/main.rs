fn main() {
    println!("Hello, world!");
    let num_list = vec![1,2,3];
    largest_number(&num_list);
}

fn largest_number<T>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list.iter(){
        if largest < item {
            largest = item;
        }
    }
    largest

}