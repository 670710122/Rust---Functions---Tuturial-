// 1. Parameter แบบ String
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 2. Parameter แบบ Integer
fn print_age(age: i32) {
    println!("Age: {}", age);
}

// 3. Parameter หลายตัว
fn introduce(name: &str, age: i32) {
    println!("Name: {}", name);
    println!("Age: {}", age);
}

// 4. Parameter สำหรับคำนวณ
fn add(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn main() {
    greet("John");

    print_age(20);

    introduce("Alice", 21);
    
    add(10, 20);
}