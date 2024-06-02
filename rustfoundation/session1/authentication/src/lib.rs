
pub fn greet(name: &str) -> String{
    format!("Hello {name}")
}

pub fn read_input_line() -> String {
    // how to read user input using read line stdio
    // also we handle unexpected errors
    let mut input = String::new();

    std::io::stdin().read_line( &mut input).expect("Failed to read line");

    input.trim().to_string() 
}

#[derive(PartialEq, Eq, Debug)]
pub enum LoginAction {
    Admin, User, Denied,
}

pub fn login(username: &str, password: &str) -> LoginAction {
    let username = username.to_lowercase();
    if username == "admin" && password == "password" {
        LoginAction::Admin
    } else if username == "bob" && password == "password" {
        LoginAction::User
    } else {
        LoginAction::Denied
    }

    //username.to_lowercase() == "admin" && password == "password" 
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let name = "Rust";
        assert_eq!(greet(&name), "Hello Rust");
    }

    #[test]
    fn test_login() {
        /*
        assert_eq!(login("ADMIN", "password"));

        assert!(login("admin", "password"));
        assert!(!login("admin-not", "password"));
        assert!(!login("admin", "not-password"));
*/
        assert_eq!(login("admin", "password"), LoginAction::Admin);
        assert_eq!(login("bob", "password"), LoginAction::User);
        assert_eq!(login("admin", "not-password"), LoginAction::Denied);
    }
  
}
