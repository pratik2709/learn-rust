fn main() {
    println!("Hello, world!");
    let num_list = vec![1,100,2,3];
    let charec_list = vec!['a','b','d'];
    println!("{}", largest_number(&charec_list));
    println!("{}", largest_number_without_clone(&charec_list));
}

fn largest_number<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list.iter(){
        if largest < item {
            largest = item;
        }
    }
    largest

}

fn largest_number_without_clone<T: PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];
    for item in list.iter(){
        if largest < item {
            largest = item;
        }
    }
    largest

}