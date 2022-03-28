
async fn main() {
    println!("Hello, world!");
    let handle = thread::spawn(||{
        for i in 1..10{
            println!("{}")
        }
    });
}
