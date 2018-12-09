#[derive(Debug)]
enum Ipaddress {
    Ipaddrv4(i32,i32,i32,i32),
    Ipaddrv6(String)
}

fn main() {
    let v4 = Ipaddress::Ipaddrv4(0,0,0,0);
    let v6 = Ipaddress::Ipaddrv6(String::from("::!"));
    println!("{:?}", v4);
}
