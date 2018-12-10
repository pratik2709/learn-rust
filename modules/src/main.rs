mod plant{
    pub mod vegetable{

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
}

fn plants_rise(){
    println!("intake carbon");
}