mod sound;
mod plant{
    pub mod vegetable{

        pub mod music{
            pub fn piano(){
                println!("playing piano")
            }
        }
        #[derive(Debug)]
        pub struct plant_types{
            pub name: String,
            size: i32,
        }

        impl plant_types{
            pub fn set_plant_details() -> plant_types{
                plant_types{
                    name:String::from("Tree"),
                    size:100
                }
            }
        }

        pub fn test_mod(){
            println!("Testing modules");
            super::super::plants_rise();
        }


    }
}

use std::collections::HashMap;



fn main() {
    //use absolute paths mostly
    use crate::plant::vegetable::music;
    music::piano();
    plant::vegetable::test_mod();
    //absolute path
    crate::plant::vegetable::test_mod();
    let t = plant::vegetable::plant_types::set_plant_details();
    println!("{:?}", t.name);

    let mut hashmap = HashMap::new();
    hashmap.insert(1,2);
    println!("{:?}", hashmap);

    use crate::sound::guitar::strings;
    strings::play_guitar();
}

fn plants_rise(){
    println!("intake carbon");
}