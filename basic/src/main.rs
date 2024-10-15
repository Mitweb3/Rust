use core::f64;

fn main() {
    number();
    boolean();
    strings();
    conditionals();
    functions();
    loops();
    structs();
    implementing_structs();
    even();
    fib();
    str();
    struct_example2();
    
}

// numbers
fn number() {
    let x = 24;
    let y = 35;
    let z = 20000.34;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // Addition
    let  a = 24;
    let  b = 44;
    let  d = a + b;
    println!("a: {}, b: {}, d: {}", a, b, d); // Changed to `println!` for proper formatting with spaces

    let mut v = 30;
    v = 55;
    print!("v: {}", v)
}

// boolean
fn boolean() {
    let is_male = true;
    let is_above_18 = true;

    if is_male {
        print!("\nYou are a male");
    } else {
         print!("\nyou are not male\n");
    }

    if is_male && is_above_18 {
        print!("\nYOu are a legal male\n")
    }
}

// string
fn strings() {
    let greeting = String::from("hello world");
    print!("{}",greeting);

    let greetings : String = String::from("Hello world");
    print!("{}", greetings);

    let char1 = greetings.chars().nth(1); 

    match char1 {
        Some(c) => println!("\n{}",c),
        None => println!("No character at index 1000")
    }  
}

// conditionals
fn conditionals() {
    let is_even = true;
    if is_even {
        println!("The number is even");
    } else if !is_even {
        println!("The number is odd");
    }
}

// loops
fn loops() {
    for i in 0..100 {
        println!("{}",i)
    }
}

// functions
fn functions() {
    let sentence = String::from("Mit Amin");
    let first_word = get_first_word(&sentence);  // Pass by reference to avoid moving ownership

    let a = 24;
    let b = 34;
    let answer = do_sum(a, b);
    println!("Sum of {} and {} is: {}", a, b, answer);  // Corrected: Added the third placeholder for `answer`

    let h = 51;
    for i in 0..h {
        println!("I am at: {}", i);
    }
    println!("First word is: {}", first_word);
}

fn do_sum(a: i32, b: i32) -> i32 {
    a + b  // No need for explicit `return` in Rust
}

fn get_first_word(sentence: &String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        if char == ' ' {  // Compare to space to get the first word
            break;
        }
        ans.push(char);
    }
    ans  // Return the result after the loop
}

// structs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn structs() {
    let mut username = String::from("Mit");
    username.push_str(" Amin");
    let user = User {
        active: true,
        username,
        email: String::from("mitamin@gmail.com"),
        sign_in_count: 2,
    };
    print!("The name is {} ", user.username);
}

// implementing structs
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32{
        return self.width * self.height
    }
}

fn implementing_structs() {
    let rect = Rect {
        width: 30,
        height: 40
    };
    println!("The area of rectangle is {}",rect.area());
}

// write a function is_even that takes a number as an input and returns true if it is even 

// i32 => -, + (with sign)
// u32 (without sign)
fn even() {
    println!("{}", is_even(20));
}

fn is_even(num:i32) -> bool{
    if num % 2 == 0 {
        return true;
    }
    return false;
}

// write a function that checks whether the number is fibonacci or not

fn fib() {
    println!("{}", fibonacci(8));
}
// 0 1 1 2 3 5 8 13 
fn fibonacci(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return  first;
    }
    if num == 1 {
        return 1;
    }
    for _ in 1..(num - 1)  {
        let temp  = second;
        second = second + first;
        first = temp;
    }
    return second;
}

// write a function get_string_length that takes a string as an input and returns its length

fn str() {
    let my_string = String::from("Mit Amin");
    let length = get_string_length_chars(&my_string);
    println!("THe number of characters in the string is : {}",length);
}

fn get_string_length_chars(s: &str) -> usize {
    s.chars().count()
}

// using struct square of a number 

struct Number {
    num : u32
}

impl Number {
    fn square(&self) -> u32 {
        self.num * self.num
    }
}

fn struct_example2() {
    let number = Number {
        num: 24,
    };
    println!("The square of the number is: {}",number.square());
}
