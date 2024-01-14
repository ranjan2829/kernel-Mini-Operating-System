// #[derive(Debug)]
// // enum IPRKIND {
// //     V4,
// //     V6,
// // }

// // fn main() {
// //     let four = IPRKIND::V4;
// //     let six = IPRKIND::V6;

// //     route(IPRKIND::V4);
// //     route(IPRKIND::V6);

// //     // Corrected println! statement
// //     println!("{:?}{:?}", four, six);
// // }

// fn route(ip: IPRKIND) {}

// // enum IPRKIND {
// //     V4,
// //     V6,
// // }

// // struct IPV {
// //     kind: IPRKIND,
// //     address: String,
// // }

// // fn main() {
// //     let home = IPV {
// //         kind: IPRKIND::V4,
// //         address: String::from("137.0.0.1"),
// //     };

// //     let homie = IPV {
// //         kind: IPRKIND::V6,
// //         address: String::from("192.168.0.1"),
// //     };

// //     println!("{:?}, {:?}, {:?}, {:?}", home.kind, home.address, homie.kind, homie.address);
// // }

// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// fn value(coin:Coin)->u8{
//     match coin{
//         Coin::Penny=>{
//             println!("PENNY");
//             1
//         },
//         Coin::Nickel=>2,
//         Coin::Dime=>3,
//         Coin::Quarter=>4,
//     }
// }
// fn main(){
//     let var=value(Coin::Penny);
//     println!("{}",var)

// }