//how to add a new user with new role and password
// cargo run add -- admin true fred2 password
use clap::{Parser, Subcommand};
use authentication::{LoginRole, User, get_users, save_users};


#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,

    /// Add a user.
    Add {
        /// The user's login name
        username: String,

        /// The user's password (plaintext)
        password: String,

        /// Optional - mark as an admin
        #[arg(long)]
        admin: Option<bool>
    },

    /// Delete a user
    Delete {
        /// User to delete
        username: String
    },

    /// Change a user's password
    ChangePassword {
        /// Username who's password should change
        username: String,

        /// New password
        new_password: String

    }
}

fn list_users(){
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");


    let users = get_users();

    users.iter().for_each(|(_, user)|{
        println!("{:<20}{:20?}", user.username, user.role);
    });
}

fn add_user(username: String, password: String, admin: bool){
    // get the users generated from get_users
    let mut users = get_users();

    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };

    // this newly created user will be added
    let user = User::new(&username, &password, role);

    users.insert(username, user);

    // we saved newly created users into file
    save_users(users);
}

fn delete_user(username: String){
    let mut users = get_users();

    if users.contains_key(&username){
        users.remove(&username);
        save_users(users);
    }  else {
        println!("User not found");
    }
}

fn change_password(username: String, password: String){
    let mut users = get_users();

    if let Some(user) = users.get_mut(&username) {
        user.password = authentication::hash_password(&password);
        save_users(users);
    } else {
        println!("User not found");
    }

}
fn main() {
    let cli = Args::parse();

    match cli.command{
        // this handle the list command
        Some(Commands::List) => {
            println!("## List of users ##");

            list_users();
        }
        // this command handle Add
        Some(Commands::Add { username, password, admin }) => {
            println!("## Add user: {} {} ##", username, password);
            // the default for admin is false
            add_user(username, password, admin.unwrap_or(false));
        }

        Some (Commands::Delete {username}) => {
            println!("## Delete user: {} ##", username);

            delete_user(username);
        }

        Some(Commands::ChangePassword {username, new_password}) => {
            println!("## Change password: {} ##", username);

            change_password(username, new_password);
        }

        None => {
            println!("Run with --help to see instructions.");
        }
    }
}
