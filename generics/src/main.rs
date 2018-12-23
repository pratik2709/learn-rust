mod lib;

#[derive(Debug)]
struct gen<T, U> {
    name: T,
    age: U,
}

impl<T, U> gen<T, U> {
    fn test_knowlege(&self) -> &T {
        &self.name
    }

    fn test_knowlege1(&self) -> &U {
        &self.age
    }
}

impl gen<&str, i32> {
    fn find_age_times_10(&self) -> i32 {
        self.age * 10
    }
}

#[derive(Debug)]
struct point<T, U> {
    name: T,
    age: U,
}

impl<T, U> point<T, U> {
    fn mixup<V, W>(self, other: point<V, W>) -> point<T, W> {
        point {
            name: self.name,
            age: other.age,
        }
    }
}


fn main() {
    let test = gen {
        name: "Pratik",
        age: 32,
    };
    println!("{:?}", test.test_knowlege());
    println!("{:?}", test.test_knowlege1());
    println!("{:?}", test.find_age_times_10());

    let p = point {
        name: "pt",
        age: "pw",
    };

    let p1 = point {
        name: 1,
        age: 2,
    };
    let p3 = p.mixup(p1);
//    println!("{:?}", p);
    println!("{:?}", p3);

    let n = NewsFeed {
        location: String::from("india"),
        date: String::from("1/2"),
        content: String::from("fffff"),
        author: String::from("prat"),
    };
    println!("{}",n.summarize());
}


struct NewsFeed {
    location: String,
    date: String,
    content: String,
    author: String,
}

impl Summary for NewsFeed {
    fn summarize_author(&self) -> String {
        format!("Summarising author:: {}, {}", self.location, self.date)
    }
}

pub trait Summary {
    fn summarize(&self) -> String{
        format!("{}", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}