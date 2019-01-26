use std::thread;
use std::time::Duration;

struct Cacher<T> where T: Fn(u32) -> u32{
  name: T,
  value: Option<u32>,
}

impl<T> Cacher<T>  where T: Fn(u32) -> u32{
  fn new(calcu: T) -> Cacher<T>{
    println!("inside new");
    Cacher{
      name: calcu,
      value:None
    }
  }

  fn value(&mut self, arg:u32)-> u32{
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.name)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

fn main(){
  generate_workout(1,2);
}


fn generate_workout(intensity:u32, random_number:u32){
  let mut expensive_func = Cacher::new(|intensity|{
    println!("inside simulated function");
    thread::sleep(Duration::from_secs(2));
    intensity
  });

  if intensity < 25 {
    expensive_func.value;
    println!("calling again:: {:?}", expensive_func.value);
  }
  else{
    if random_number == 3{
      println!("take a break");
    }
    else{
      expensive_func.value;
    }
  }

}