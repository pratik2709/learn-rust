use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    let random_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number {}", random_number);

    loop {
        let mut guess = String::new();


        io::stdin().read_line(&mut guess)
            .expect("fail!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too great"),
            Ordering::Equal => {
                println!("equal");
            break;
            }
        }
    }
}
