mod plant{
    pub mod vegetable{
        pub fn test_mod(){
            println!("Testing modules");
        }
    }
}

fn main() {
    plant::vegetable::test_mod();
}
