static NUM: i32 = 120;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn execute(){
    {
        println!("hello world {}", NUM);
    }
    {
        let sample = 12;
        let checking = coerce_static(&sample);
        println!("checking: {}", checking)
    }

}