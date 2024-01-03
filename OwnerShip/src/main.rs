// //use std::String;
// fn main() {
//     let mut s=String::from("ranjan");
//     s.push_str(",shitole");
//     println!("{}",s);
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);
// }
// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.
// fn main(){
//     let s1=String::from("hello");
//     let s2=gives_ownership();
//     let s3=gives_takes(s1);


// }

// fn gives_ownership()->String{
//     let some=String::from("ranjan");
//     some
// }
// fn gives_takes(sometr:String)->String{
//     sometr
// }
// fn main(){
//     let s1=String::from("hi");
//     let (s2,len)=calculate(s1);
//     println!("length of {} is {}",s2,len);
// }
// fn calculate(s:String)->(String,usize){
//     let length=s.len();
//     (s,length)

// }
// fn main(){
//     let s1=String::from("ranjan");
//     let length=calculate(&s1);
//     println!("{} length {}",s1,length);
// }
// fn calculate(s:&String)->usize{
//     s.len()
// }
