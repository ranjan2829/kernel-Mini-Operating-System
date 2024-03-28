// fn main() {
//     let r;

//     let x = 10;
//     r = &x;

//     println!("{}", r);
// }
//

// Assuming longest function is defined in another module called utils
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
