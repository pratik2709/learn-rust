#[derive(Debug)]
struct gen <T,U>{
    name: T,
    age: U
}

//impl gen <T,U>{
//    fn test_knowlege(){
//        gen{
//
//        }
//    }
//}

fn main() {
    let test = gen{
        name: "Pratik",
        age: 32
    };
    println!("{:?}", test);
}
