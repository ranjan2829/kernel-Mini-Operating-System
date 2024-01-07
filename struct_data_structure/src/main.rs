// // struct user{
// //     active:bool,
// //     username:String,
// //     mail:String,
// //     sign_in:u64,

// // }


// // fn main() {
// //     let user1=user{
// //         active:true,
// //         username:String::from("ranjan3129"),
// //         mail:String::from("@gmail.com"),
// //         sign_in:24,

// //     };
// //     println!("{},{},{},{}",user1.active,user1.username,user1.mail,user1.sign_in);

// // }
// struct User {
//     active: bool,
//     Email: String,
//     name: String,
//     sign_in: u32,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         Email: email,
//         name: username,
//         sign_in: 1,
//     }
// }

// fn main() {
//     let mail = String::from("ranjan@");
//     let uname = String::from("ranjan3129");
    
//     let var = build_user(mail, uname);

//     println!("{},{},{},{}", var.Email, var.sign_in, var.name, var.active);
// }
// struct user{
//     active :bool,
//     username:String,
//     email:String,
//     sign_in:u64,
// }
// fn build_user(email:String,username:String)->user{
//     user{
//         active:true,
//         username:username,
//         email:email,
//         sign_in:1,
//     }
// }
// fn main(){
//     let user1=build_user(
//         String::from("@@@@@@@"),
//         String::from("ranjan3129"),
//     );
    
// }
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     // --snip--

//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };
//     println!("{},{},{},{}",user2.email,user2.username,user2.sign_in_count,user2.active);
// }
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     // --snip--

//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
// }
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
// fn main(){
//     let rect=(23,23);
//     println!("{}",area(rect));
// }
// fn area(dimensions:(u32,u32))->u32{
//     dimensions.0 * dimensions.1
// }
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!("Area: {}", area(&rect1));
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:?}", rect1);
// }
struct Rectangle{
    width:u32,
    height:u32,
}
impl Rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }
}
fn main(){
    let rect1=Rectangle{
        width:30,
        height:50,
    };
    println!("{}",rect1.area());
}
