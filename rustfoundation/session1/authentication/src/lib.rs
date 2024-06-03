// cargo add serde -F derive
// cargo add serde_json
use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};

pub fn hash_password(password: &str) -> String{
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin, User
}


#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied, 

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            //password: password.to_string(),
            password: hash_password(password), 
            role
        }
    }
}

/*
pub fn get_users() -> Vec<User> {
    vec![
        User::new("admin", "password", LoginRole::Admin),
        User::new("bob", "password", LoginRole::User)
    ]
}
*/

pub fn get_default_users()-> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));
    users
}


pub fn get_users()-> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists(){
        // Load the file!
        //HashMap::new()

        let contents = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&contents).unwrap();
        users
    } else {
        // Create a file and return it
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();

        users
    }
}
/*
pub fn get_users()-> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));
    users
}*/

/*
pub fn get_admin_users(){
    let users: Vec<String> = get_users()
        .into_iter()
        .filter(|u| u.role == LoginRole::Admin)
        .map(|u| u.username)
        .collect();
}*/


pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let users = get_users();
    if let Some(user) = users.get(&username) {

        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    
    }

    /*
    if let Some(user) = users.iter().find(|user| user.username == username) {
        if user.password == password {

            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }*/

    None
        /*
    let username = username.to_lowercase();

    if username != "admin" && password != "password" {
       return  None
    }

    if username == "admin" && password == "password" {
        Some(LoginAction::Granted(LoginRole::Admin))
    } else if username == "bob" && password == "password" {
        Some(LoginAction::Granted(LoginRole::User))
    } else {
        Some(LoginAction::Denied) //LoginAction::Denied
    }

    */
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
/*
        assert_eq!(login("admin", "password"), LoginAction::Admin);
        assert_eq!(login("bob", "password"), LoginAction::User);
        assert_eq!(login("admin", "not-password"), LoginAction::Denied);
*/



    }
  
}
