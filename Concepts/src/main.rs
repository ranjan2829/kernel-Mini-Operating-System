// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// fn main(){
//     let x=2.0;
//     let y:f32=3.0;
//     println!("x={}",x);
//     println!("y={}",y);
// }
// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let cat = '😻';
//     println!{"catttt={}",cat};
// }
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     println!("{},{},{}", tup.0, tup.1, tup.2);
// }
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
