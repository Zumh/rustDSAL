use authentication::{ LoginAction, login, greet, read_input_line};

fn main() {
    println!("Hello, world!");
    let mut tries = 0;
    loop {
        println!("Enter your username: ");
        let username = read_input_line();

        println!("Enter your password: ");
        let password = read_input_line();

        match login(&username, &password){
            LoginAction::Granted(role) => {
                match role {
                    authentication::LoginRole::Admin => println!("Admin"),
                    authentication::LoginRole::User => println!("User"),
                }
                break;
            }
            LoginAction::Denied => println!("Denied"),
        }

        println!("Incorrect username or password");
        tries += 1;
        if tries >= 3 {
            println!("Too many tries");
            break;
        }

        /*
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
        }*/

    }

    println!("{}", greet("Rust"));
}
