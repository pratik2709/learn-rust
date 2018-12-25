fn main() {
//    let r;
//    {
//        let x = 5;
//        r = &x;
//    }
//
//    println!("{}", r);
    test_lifetimes();

    println!("{}", test_important_sentence().name);
}

fn test_lifetimes(){
    let s = String::from("longlonger");
    let result;
    {
        let news = String::from("longlonger124");
        result = longest_string(&s, &news);
        println!("lifetime is :: {}", result);

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

//fn longest_string_no_association<'a>(s1: &'a str, s2: & str) -> &'a str{
//    let test = String::from("res");
//    test.as_str()
//}
#[derive(Debug)]
struct ImportantSentence{
    name: String,
}

fn test_important_sentence() -> ImportantSentence{
    let sen = String::from("testing sentences. total");
    let reff = sen.split(".").next().expect("could not find a .");
    let c = ImportantSentence{
        name:reff.to_string(),
    };
    c
}