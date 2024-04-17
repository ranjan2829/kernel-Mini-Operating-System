use std::env;
use std::fs;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    //let (query, file_path) = parsing_config(&args);
    let config = parsing_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
struct config {
    query: String,
    file_path: String,
}
// fn parsing_config(args: &[String]) -> config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//     config { query, file_path }
// }
