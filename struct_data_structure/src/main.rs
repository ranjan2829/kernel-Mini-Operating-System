struct user{
    active:bool,
    username:String,
    mail:String,
    sign_in:u64,

}


fn main() {
    let user1=user{
        active:true,
        username:String::from("ranjan3129"),
        mail:String::from("@gmail.com"),
        sign_in:24,

    };

}
