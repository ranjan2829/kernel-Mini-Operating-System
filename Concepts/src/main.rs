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
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
// }
// use std::io;
// fn main(){
//     let a=[1,2,3,4,5];
//     println!("enter an array");
//     let mut index=String::new();
//     std::io::stdin()
//         .read_line(&mut index)
//         .expect("failed to do so");
//     let index:usize=index.trim().parse().expect("not an number");
//     let element=a[index];
//     println!("value of element at index {} is {}",index,element);
    


// }

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main(){
//     let mut counter=0;
//     'countingloop:loop{
//         println!("counter ={}",counter);
//         let mut remaining=10;
//         loop{
//             println!("remaining={remaining}");
//             if remaining==9{
//                 break;
//             }
//             if counter==2{
//                 break countingloop;
//             }
//             remaining-=1;
//         }
//         counter+=1;
//     }
//     println!("end ={}",counter);
// }
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }
fn main(){
    let mut number=3;
    while number!=0{
        println!("{number}!");
        number-=1;
    }
    println!("Liftoff!!");

}