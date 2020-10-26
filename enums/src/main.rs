
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing {:?}", ip_kind);
}

fn main() {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    let home = IpAddr::V4(String::from("127.0.0.1"));
    println!("Home IP Address : {:?}", home);
    println!("Loopback Address : {:?}", IpAddr::V6(String::from("::1")));
    let m = Message::Write(String::from("hello"));
    m.call();
}
