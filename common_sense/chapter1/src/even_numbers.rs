// Measure the speed of the print_numbers_version_one and print_numbers_version_two functions
// using cpu time is the most in-accurate because it fluctuates over time.
// We can only measure using steps it take to operate a datas with effecient algorithm. In this case Big O notation.

use std::{thread, time::Instant};

const NUMBER: u32 = 100;
const EVEN_NUMBER: u32 = 2;

fn print_numbers_version_one() {
    let mut number: u32 = EVEN_NUMBER;
    while number <= NUMBER {
        if number % EVEN_NUMBER == 0 {
            println!("1-Version {}", number);
        }
        number += 1;
    }
}

fn print_numbers_version_two() {
    let mut number: u32 = EVEN_NUMBER;
    while number <= NUMBER {
        println!("2-Version {}", number);
        number += 2;
    }
}

pub fn test_even_numbers() {

    let thread1 = thread::spawn(|| {
        measure_speed("1-Version", &print_numbers_version_one);
    });

    let thread2 = thread::spawn(|| {
        measure_speed("2-Version", &print_numbers_version_two);
    });


    
    //println!("## Question 1 if number % 2 == 0 : Print all the even numbers from 2 to 100");

    thread1.join().unwrap();

    //println!("## Question 1 version 2  number += 2; : Print all the even numbers from 2 to 100");
    thread2.join().unwrap();


}

fn measure_speed(version: &str, func: &dyn Fn()) {
    let start = Instant::now();
    
    // Call your function or code snippet here
    func();
    
    let end = Instant::now();
    let elapsed = end - start;
    
    println!("{} Elapsed time: {:?}", version, elapsed);
}
