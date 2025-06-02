#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

fn route(ip_kind: IpAddrKind){
    println!("routing {:?}", ip_kind)
}

enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    

    println!("Hello, world!");
}
