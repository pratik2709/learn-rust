fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }

    println!("{}", r);
}

fn test_lifetimes(){
    let s = String::from("long longer");
    {
        let news = "longest";
    }
    let result = longest_string(&s, &news);
}

fn longest_string(s1: &str, s2: &str) -> &str{
    if s1 > s2{
        s1
    }
    else {
        s2
    }
}
