fn main() {
    //create new vector
    let mut v1: Vec<i32> = Vec::new();
    let v = vec![1,2,3];
    v1.push(5);
    v1.push(6);

    let third:&i32 = &v[2];
    println!("Third is:: {}", third);

    let t = v1.get(1);
    println!("T is:: {:?}", t);

    match v1.get(1) {
        Some(A) => println!("ts"),
        None => println!("tFds"),
    }

    for i in &v1{
        println!("{}", i);
    }

    for i in &mut v1{
        *i *= 5;
        println!("{}", i);
    }
}
