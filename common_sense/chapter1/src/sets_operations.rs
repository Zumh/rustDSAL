use std::collections::HashSet;

// Read value from sets operations
fn read_value(fruits: &HashSet<&str>, index: &str) {
    // using hashing techniques to store and retrieve elements access time is O(1)
    operation(format!("Read value from index {index} in fruits's array \nTime Complexity O(1)").as_str());
    println!("Before: fruits: {:?}", fruits);
    if let Some(fruit) = fruits.get(index) {
        println!("After: Fruit at index {index} is {}", fruit);
    }
}
// Search value from sets operations
fn search_value(fruits: &HashSet<&str>, target_fruit: &str) {
    operation(format!("Search {target_fruit} in fruits's array \nTime Complexity O(n)").as_str());
    println!("Before: fruits: {:?}", fruits);
    if let Some(index) = fruits.iter().position(|&x| x == target_fruit) {
        println!("After: {} is at index {}", target_fruit, index);
    }
    
}

// Add value from sets operations
// Lifetime parameter here is needed to ensure new_fruit is in scope live long enough
// to be added to the HashSet
// Because we pass multiple variable as reference.
fn add_value<'a>(fruits: &mut HashSet<&'a str>, new_fruit: &'a str) {
    operation(format!("Add {new_fruit} in fruits's array \nTime Complexity O(log N) 
    \nIt use binary search and then insert ").as_str());
    println!("Before: fruits: {:?}", fruits);
    fruits.insert(new_fruit);
    println!("After: fruits: {:?}", fruits);
}


// Remove value from sets operations
fn remove_value(fruits: &mut HashSet<&str>, target_fruit: &str) {
    operation(format!("Remove {target_fruit} from fruits's array \nTime Complexity O(n)").as_str());
    println!("fruits: {:?}", fruits);
    fruits.remove(target_fruit);
    println!("fruits: {:?}", fruits);
}
pub fn sets_operations() {
    println!("\n\nSets_operations");
    //     
    let mut fruits: HashSet<&str> = HashSet::from(["apples", "bananas", "cucumbers", "dates", "elderberries"]);
   
    read_value(&fruits, "apples");

    search_value(&mut fruits, "dates");

    // Add value from sets operations
    add_value(&mut fruits, "grapes");
   
    // Remove value from sets operations
    remove_value(&mut fruits, "grapes");


}










// Order sets operations
use std::collections::BTreeSet;


fn operation(operation_name: &str) {
    println!("\n\n{}\n", operation_name);
}

pub fn order_sets_operations() {
    println!("\n\nOrder_sets_operations");
    let mut fruits:BTreeSet<&str> = BTreeSet::from(["apples", "bananas", "cucumbers", "dates", "elderberries"]);
     
    println!("fruits: {:?}", fruits);
    let index = "cucumbers";
    // Read value from sets operations
    operation(format!("Read value from index {index} in fruits's array \nTime Complexity O(1)").as_str());
    println!("Before: fruits: {:?}", fruits);
    if let Some(fruit) = fruits.get(index) {
        println!("After: Fruit at index {index} is {}", fruit);
    }

    // Search value from sets operations
    operation(format!("Search {index} in fruits's array \nTime Complexity O(n)").as_str());
    println!("Before: fruits: {:?}", fruits);
    println!("After: {} is at index {:?}", index, fruits.contains(index));

    // Add value from sets operations
    operation(format!("Add {index} in fruits's array \nTime Complexity O(2n+1) 
    \nSteps: search for duplicate, shift all of the elements, add new element").as_str());
    println!("Before: fruits: {:?}", fruits);
    fruits.insert("durains");
    println!("After: fruits: {:?}", fruits);

    // Remove value from sets operations
    operation(format!("Remove {index} from fruits's array \nTime Complexity O(n)").as_str());
    println!("Before: fruits: {:?}", fruits);
    fruits.remove(index);
    println!("After: fruits: {:?}", fruits);
    
}   