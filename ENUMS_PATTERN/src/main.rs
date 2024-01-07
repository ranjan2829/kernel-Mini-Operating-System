#[derive(Debug)]
enum IPRKIND {
    V4,
    V6,
}

fn main() {
    let four = IPRKIND::V4;
    let six = IPRKIND::V6;

    route(IPRKIND::V4);
    route(IPRKIND::V6);

    // Corrected println! statement
    println!("{:?}{:?}", four, six);
}

fn route(ip: IPRKIND) {}
