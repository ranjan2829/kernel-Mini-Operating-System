// use std::String;
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
// fn main(){
//     let mut s=String::from("ranjan");
//     change(&mut s);
//     println!("{}",s);
// }
// fn change(s:&mut String){
//     s.push_str(",shitole");

// }
// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//         println!("{}",r1);
//     } // r1 goes out of scope here, so we can make a new reference with no problems.

//     let r2 = &mut s;
//     println!("{}",r2);
// }
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);
// }
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point

//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }
// fn main() {
//     let string = no_dangle();
// }

// fn no_dangle() -> String {
//     let s = String::from("hello");

//     s
// }
// fn first_word(s:&String)->usize{
//     let bytes=s.as_bytes();
//     for(i,&item) in bytes.iter().enumerate(){
//         if item ==b' '{
//             return i;
//         }
//     }
//     s.len()
// }
// fn main(){
//     let word=String::from("ran is the worst");
//     let length = first_word(&word);
//     println!("{}",length);

// }
//fn main(){
//     let s =String::from("ranjan is worst");
//     let hello=&s[0..6];
//     let worst=&s[7..9];
//     println!("{}{}",hello,worst);
// }
fn first_word(s: &String) -> (usize, &str) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (i, &s[0..i]);
        }
    }

    (s.len(), &s)
}


fn main() {
    let word=String::from("ranjan1 is the worst");
    let var=first_word(&word);
    println!("Index: {}, Word: {}", var.0, var.1);

}



