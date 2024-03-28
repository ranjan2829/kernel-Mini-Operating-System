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
// }

// fn main() {
//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     }
// }
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}
