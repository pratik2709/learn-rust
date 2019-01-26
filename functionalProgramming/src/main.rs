use std::thread;
use std::time::Duration;

fn main(){
  generate_workout(1,2);
}



fn generate_workout(intensity:u32, random_number:u32){
  let expensive_func = |intensity|{
    println!("inside simulated function");
    thread::sleep(Duration::from_secs(2));
    intensity
  };

  if intensity < 25 {
    expensive_func(intensity);
    println!("calling again:: {}", expensive_func(intensity));
  }
  else{
    if random_number == 3{
      println!("take a break");
    }
    else{
      expensive_func(intensity);
    }
  }

}