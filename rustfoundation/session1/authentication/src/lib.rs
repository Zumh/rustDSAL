
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

pub fn login(username: &str, password: &str) -> bool {
    username.to_lowercase() == "admin" && password == "password" 
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
        assert!(login("ADMIN", "password"));

        assert!(login("admin", "password"));
        assert!(!login("admin-not", "password"));
        assert!(!login("admin", "not-password"));

    }
  
}
