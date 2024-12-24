// fn main() {
//     let r;

//     let x = 10;
//     r = &x;

//     println!("{}", r);
// }
//

// Assuming longest function is defined in another module called utils
// fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// // }

// // fn main() {
// //     let string1 = String::from("long string is long");

// //     {
// //         let string2 = String::from("xyz");
// //         let result = longest(string1.as_str(), string2.as_str());
// //         println!("The longest string is {}", result);
// //     }
// // }
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
// fn first_word<'a>(s:&'a str)->&'a str{
//     let bytes=s.as_bytes();
//     for(i,item) in bytes.iter().enumerate(){
//         if item ==b' '{
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// fn main(){
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
//     let string="ranjan is the best";
//     let ans=first_word(&string)

//     println!("{},{}", i.part,ans);
// }
//
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
