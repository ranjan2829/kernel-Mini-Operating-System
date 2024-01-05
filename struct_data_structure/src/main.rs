// struct user{
//     active:bool,
//     username:String,
//     mail:String,
//     sign_in:u64,

// }


// fn main() {
//     let user1=user{
//         active:true,
//         username:String::from("ranjan3129"),
//         mail:String::from("@gmail.com"),
//         sign_in:24,

//     };
//     println!("{},{},{},{}",user1.active,user1.username,user1.mail,user1.sign_in);

// }
struct User {
    active: bool,
    Email: String,
    name: String,
    sign_in: u32,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        Email: email,
        name: username,
        sign_in: 1,
    }
}

fn main() {
    let mail = String::from("ranjan@");
    let uname = String::from("ranjan3129");
    //let var = build_user(email: mail, username: uname);
    let var = build_user(mail, uname);

    println!("{},{},{},{}", var.Email, var.sign_in, var.name, var.active);
}
