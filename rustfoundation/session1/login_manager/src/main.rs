use clap::{Parser, Subcommand};

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
}
fn main() {
    let cli = Args::parse();

    match cli.command{
        Some(Commands::List) => {
            println!("List of users");
        }
        None => {
            println!("Run with --help to see instructions.");
        }
    }
}
