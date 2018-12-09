#[derive(Debug)]
enum Ipaddress {
    Ipaddrv4(i32,i32,i32,i32),
    Ipaddrv6(String)
}

#[derive(Debug)]
enum Option<T>{
    Some(T),
    None
}

fn main() {
    let v4 = Ipaddress::Ipaddrv4(0,0,0,0);
    let v6 = Ipaddress::Ipaddrv6(String::from("::!"));
    println!("{:?}", v4);
    println!("This address:: {}", which_address(v4));


    println!("This enum:: {:?}", handle_null(Option::Some(3)));
}

fn which_address(ipaddress:Ipaddress) -> u32{
    match ipaddress {
        Ipaddress::Ipaddrv4(_,_,_,_) => {
            println!("Compete");
            1
        },
        Ipaddress::Ipaddrv6(_) => 2,
    }
}

fn handle_null(option: Option<i32>)->Option<i32>{
    match option {
        Option::Some(i) => Option::Some(i+1),
        Option::None => Option::None,
    }
}
