
pub fn greet(name: &str) -> String{
    format!("Hello {name}")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let name = "Rust";
        assert_eq!(greet(&name), "Hello Rust");
    }
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
