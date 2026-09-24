fn total_score(math: f64, english: f64, programming: f64) -> f64 {
    math + english + programming
}

fn grade(average: f64) -> char {
    if average >= 80.0 {
        'A'
    } else if average >= 70.0 {
        'B'
    } else if average >= 60.0 {
        'C'
    } else {
        'F'
    }
}

fn show_result(name: &str, total: f64, grade: char) {
    println!("Student: {}", name);
    println!("Total: {:.2}", total);
    println!("Grade: {}", grade);
}

use std::io;

fn main() {
    let mut name = String::new();

    println!("Enter your name:");

    io::stdin().read_line(&mut name).unwrap();

    println!("Hello, {}", name);
}