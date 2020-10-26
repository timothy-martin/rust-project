
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing {:#?}", ip_kind);
}

fn main() {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}
