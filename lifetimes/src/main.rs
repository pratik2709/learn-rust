fn main() {
//    let r;
//    {
//        let x = 5;
//        r = &x;
//    }
//
//    println!("{}", r);
    test_lifetimes();
}

fn test_lifetimes(){
    let s = String::from("long longer");
    {
        let news = "longest";
        let result = longest_string(&s, &news);
    }

}

fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str{
    if s1 > s2{
        s1
    }
    else {
        s2
    }
}
