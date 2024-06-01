fn main() {
    let n: i32 = 5;
    println!("{n}");
    println!("Hello, world!");

    mutables();
    scope_function();
    functions();

    let name: String = "John Hello".to_string();
    greet(name.clone());
    // owner ship 
    greet(name);

    // borrow give me the access to the variable
    let borrow_name = "Borrow Johnny hello".to_string();
    greet_borrow(&borrow_name);

    println!("\n");
    // mutable borrowing
    let mut mut_name = "Sammy mutable borrow string.".to_string();
    greet_borrow_mut(&mut mut_name);
    println!("mut name {mut_name}");

    println!("\n");

    // read line 
    println!("## Read line ##");
    let input:String = read_input_line();
    println!("You type [{input}]");
}

fn read_input_line() -> String {
    // how to read user input using read line stdio
    // also we handle unexpected errors
    let mut input = String::new();

    std::io::stdin().read_line( &mut input).expect("Failed to read line");

    input.trim().to_string() 
}

fn greet_borrow_mut(s: &mut String) {
    println!("## Borrowing mutable variable name ##");
    // give me the access vriable that you own.
    *s = format!("Hello {s}");
}

fn greet_borrow(s: &String) {
    // give me the access vriable that you own.
    println!("Borrowing variable name");
    
    println!("Hello {s}");

}
fn greet(s: String)-> String {

    println!("Hello {s}");

    s
}

fn double(n: i32) -> i32 {
    println!("### Double ###");
    n * 2
}

fn functions(){
    println!("#### functions ####");
    // let n = double(5);
    let n = double(5);
    println!("double {n}");
    let i: i32 = 5;
    let n: i32 = if i > 0 { 5 } else { 0 };
    println!("check i value {n}");
    // double the number
    let old_number: i32 = 5;
    let n: i32 = double(old_number);
    println!("double original {old_number} and {n}");
    
    // double or nothing
    let new_number: i32 = double_or_nothing(20);
    println!("double or nothing {new_number}");
} 

fn double_or_nothing(n: i32)-> i32{
    println!("## Double or nothing ##");
    if n > 0 {
        return n * 2;
    }
    0
}
fn scope_function(){

    let _n: i32 = 5;
    let _n: () = {

        let _n: i32 =  6;
    };

    println!("{_n:?}");
}

fn mutables(){
    // Mutable
    let mut n: i32 = 5;
    println!("Before increment {n}");

    n += 1;

    println!("Incremented mutable value {n}");

    let n: i32 = 5;
    // Over shadowing 
    let n: i32 = n + 1;

    println!("{n}");

    
}
