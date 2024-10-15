use std::char;
use std::fs;
use std::io;
use std::usize;

use chrono::Local;
use chrono::Utc;
use rand::thread_rng;
use rand::Rng;


fn main() {
    heap_fn();
    stack_fn();
    ownership();    
    borrowing();
    main2();
    main3();
    error();
    error2();
    a();
    local();
    random();
}

// stack vs heap in rust
fn heap_fn() {
    let s1 = String::from("Mit");
    let s2 = String::from("Amin");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: combined string is {}", combined);
}

fn stack_fn() {
    let s1 = 10;
    let s2 = 20;
    let c: i32 = s1 + s2;
    // Corrected: Now using `s1` and `s2` instead of undefined `a` and `b`
    println!("Stack function: the sum of {} and {} is {}", s1, s2, c);
}


// Ownership in rust
fn ownership() {
    
    let s1 = String::from("Hii there");
    let s2 = s1;
    // println!("{}",s1); //can't use s1 
    println!("{}",s2 )

}

    // Ownership eg no.2

// fn main() {
//     let my_string = String::from("hello");
//     takes_ownership(my_string); // if you still want to use both the variables then use my_string.clone(); 
//     println!("{}", my_string); // This line would cause a compile error because ownership has been moved.
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string); // `some_string` now owns the data.
// }

fn borrowing() {
    let s1 = String::from("Mit amin");
    let s2 = &s1; //here s2 is a borrower of s1
    println!("{}", s1)
}

// borrow mutably
fn main2() {
    let mut s1 = String::from("My name is");
    update_str(&mut s1);
    println!("{}",s1);
}

fn update_str(s: &mut String) {
    s.push_str("Mit Amin");
}


// Enums

enum Direction {
    North,
    East,
    South,
    West,
}

fn enums() {
    let my_direction = Direction::North;
    move_around(my_direction);
}

fn move_around(direction: Direction) {
    // implements logic to move a character around
}

// enums with pattern matching

enum Shape {
    Circle(f64),  //value with associated data (radius), vise versa for the rest of the value
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape:Shape) -> f64 {
   match shape {
       Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
       Shape::Rectangle(width, height) => width * height,
       Shape::Square(side_length) => side_length * side_length,
   }
}

fn main3 () {
    let circle = Shape::Circle(5.25);
    let square = Shape::Square(5.696);
    let rectangle = Shape::Rectangle(2.58, 4.85);

    println!("Area of circle is: {}", calculate_area(circle));
    println!("Area of rectangle is: {}", calculate_area(rectangle));
    println!("Area of square is: {}", calculate_area(square));
}


// error handling

fn error() {
    let res = fs::read_to_string("example.txt");

    match res {
        Ok(content) => {
            println!("File content is {}",content);
        },
        Err (err) => {
            println!("Error {}",err);
        }
    }
    print!("hii there");
}
    // error handling #2

    fn error2() {
        let res = read_from_file_unsafe("mit.txt".to_string());
        match res {
            Ok(content) => println!("File content: {}", content),
            Err(err) => println!("Failed to read file: {}", err),
        }
    }
    
    fn read_from_file_unsafe(file_content: String) -> Result<String, String> {
        let res = fs::read_to_string(file_content);
        match res {
            Ok(content) => Ok(content),
            Err(_) => Err("Error reading file".to_string()),  // Removed semicolon
        }
    }
    
// option enum

fn find_first_a(s: String) -> Option<i32> {
    for (index , character) in s.chars().enumerate() {
        if character == 'a' {
            return  Some(index as i32);
        }
    }
    return None;
}

fn a() {
    let my_string = String::from("raman");
    let res = find_first_a(my_string);

    match res {
        Some(index) => println!("The letter 'a' is fount at index: {}",index),
        None => println!("The letter 'a' is not found in the string. ")
    }
}



// using libraries

fn local() {
    let now = Utc::now();
    println!("Current date and time in UTC: {}",now);

    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time {}", formatted);

    let local = Local::now();
    println!("Current date and time in local: {}",local);
}

// random number

fn random() {
    let mut rng = thread_rng();
    let n: u32 = rng.gen();
    println!("Random number is : {}",n);
}