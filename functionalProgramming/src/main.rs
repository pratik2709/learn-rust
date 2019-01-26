use std::thread;
use std::time::Duration;

fn main(){
  generate_workout(1,2);
}

fn simulated_expensive_function(intensity: u32) -> u32 {
  println!("inside simulated function");
  thread::sleep(Duration::from_secs(2));
  intensity
}

fn generate_workout(intensity:u32, random_number:u32){
  if(intensity < 25){
    simulated_expensive_function(intensity);
  
    println!("calling again:: {}", simulated_expensive_function(intensity));
  }
  else{
    if random_number == 3{
      println!("take a break");
    }
    else{
      simulated_expensive_function(intensity);
    }
  }

}