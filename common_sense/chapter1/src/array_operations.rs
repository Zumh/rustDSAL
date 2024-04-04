fn operation(operation_name: &str) {
    println!("\n\n{}\n", operation_name);
}
pub fn array_operations() {

    //test_even_numbers();
    
    // Array operations
   let mut fruits = vec!["apples", "bananas", "cucumbers", "dates", "elderberries"];
   println!("Fruits\n{:?}", fruits);

   // reading from an array
   let index = 2;
   operation(format!("Reading fruit from index {index} in an array \nTime Complexity O(1)").as_str());
   println!("{:?}", fruits);
   println!("{:?}", fruits[index]);
  


   // writing to an array
   operation("Over writing to an array \nTime Complexity O(1)");
   fruits[0] = "grapes";
   println!("{:?}", fruits);

   // Search a value in an array
   let target_fruit = "dates";
   operation(format!("Search {target_fruit} in fruits's array \nTime Complexity O(n)").as_str());
   if let Some(index) = fruits.iter().position(|&x| x == target_fruit) {
       println!("{} is at index {}", target_fruit, index);
   } else {
       println!("{} not found", target_fruit);
   }
   println!("{:?}", fruits);


    let target_fruit = 3;
   operation(format!("Insert a {} in fruits's array \nTime Complexity O(n + 1)", fruits[target_fruit]).as_str());
   // Insert fruits in array
   fruits.insert(3, "figs");
   println!("{:?}", fruits);
   
   operation(format!("Remove a {} in fruits's array \nTime Complexity O(n)", fruits[target_fruit]).as_str());
   // Remove fruits from array
   fruits.remove(3);
   println!("{:?}", fruits);

   

}