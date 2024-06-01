use authentication::{login, greet, read_input_line};

fn main() {
    println!("Hello, world!");
    let mut tries = 0;
    loop {
        println!("Enter your username: ");
        let username = read_input_line();

        println!("Enter your password: ");
        let password = read_input_line();

        if login(&username, &password) {
            println!("Success");
            break;
        } else {
            println!("Failed");
            tries += 1;
            if tries >= 3 {
                println!("Too many tries");
                break;
            }
        }
    }

    println!("{}", greet("Rust"));
}
