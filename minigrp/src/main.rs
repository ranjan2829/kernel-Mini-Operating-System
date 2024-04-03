use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    //iterator on the command line to collect the string
    dbg!(args);
}
