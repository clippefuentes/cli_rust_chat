use std::io;

mod server;
mod client;

fn main() {
    println!("Hello, world!");
    println!("Please choose type of app");
    println!("1 - server");
    println!("2 - client");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read");

    let type_app: i8 = input.trim().parse().unwrap();

    if type_app == 1 {
        server::start_client();
    } else if type_app == 2 {
        client::start_client();
    } else {
        println!("only select 1 and 2");
    } 
}
