fn main() {
  let v1 = vec![1,2,3];
  let v1_iter = v1.iter();
  println!("{:?}", v1_iter);

  for v in v1_iter{
    println!("{}", v)
  }
  let sumof:i32 = v1.iter().sum();
  let t:Vec<_> = v1.iter().map(|x| x+1).collect();
  println!("{:?}", t);
  shoe_main();

  println!("{:?}",Counter::new());
  let mut c = Counter::new();
  println!("{:?}",c.next());
  println!("{:?}",c.next());
  println!("{:?}",c.next());
}

#[derive(Debug)]
struct Shoe{
  name: String,
  size: u32
}

fn find_my_shoe(shoe:Vec<Shoe>, size:u32) -> Vec<Shoe>{
  shoe.into_iter().filter(|s| s.size == size).collect()
}

fn shoe_main(){
  let s = vec![
    Shoe{name:String::from("t1"),size:10},
    Shoe{name: String::from("t2"),size: 20},
    Shoe{name:String::from("t3"), size:10},
    ];
  println!("{:?}",find_my_shoe(s,10));
}



#[derive(Debug)]
struct Counter{
  count:u32,
}

impl Counter{
  fn new() -> Counter{
    Counter{count:0}
  }
}

impl Iterator for Counter{
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item>{
    self.count += 1;
    if self.count < 3{
      Some(self.count)
    }
    else{
      None
    }

  }
}