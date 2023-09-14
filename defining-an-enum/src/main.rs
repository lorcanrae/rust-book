#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}


fn route(ip_kind: IpAddrKind) {
    println!("IP kind of {:?}", ip_kind)
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}
