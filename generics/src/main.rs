#[derive(Debug)]
struct gen <T,U>{
    name: T,
    age: U
}

impl<T, U> gen<T,U>{
    fn test_knowlege(&self) -> &T{
        &self.name
    }

    fn test_knowlege1(&self) -> &U{
        &self.age
    }
}

fn main() {
    let test = gen{
        name: "Pratik",
        age: 32
    };
    println!("{:?}", test.test_knowlege());
    println!("{:?}", test.test_knowlege1());
}
