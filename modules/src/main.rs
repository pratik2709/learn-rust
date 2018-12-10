mod plant{
    pub mod vegetable{
        #[derive(Debug)]
        pub struct plant_types{
            name: String,
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

fn main() {
    plant::vegetable::test_mod();
    //absolute path
    crate::plant::vegetable::test_mod();
    let t = plant::vegetable::plant_types::set_plant_details();
    println!("{:?}", t)
}

fn plants_rise(){
    println!("intake carbon");
}