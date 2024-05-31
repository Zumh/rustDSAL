fn main() {
    println!("Hello, world!");

    // Data types 
    // boolean
    let b1: bool = true;

    println!("True or false\n{b1}");

    // unsigned integers
    println!("Unsigned integers");
    let ui1: u8 = 1;
    let ui2: u16 = 1;
    let ui3: u32 = 1;
    let ui4: u64 = 1;
    let ui6: u128 = 1;
    println!("{ui1} {ui2} {ui3} {ui4} {ui6}");

    // unsigned integers
    println!("Signed integers");
    let i7: i8 = 2;
    let i8: i16 = -2;
    let i9: i32 = 2;
    let i10: i64 = -2;
    let i11: i128 = 2;
    println!("{i7} {i8} {i9} {i10} {i11}");

    println!("Floating point number");
    // floating point numbers
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    println!("{f1} {f2}");

    println!("Platform specific integers");
    // platform specific integers
    let p1: usize = 1;
    let p2: isize = 1;
    println!("pointer size unsigned integer :{p1} \npointer size signed integer : {p2}");



} 
