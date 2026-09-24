
// 1. Function ที่มี 2 Parameters
fn introduce(name: &str, age: i32) {
    println!("Name: {}", name);
    println!("Age: {}", age);
}

// 2. Function ที่มี 3 Parameters
fn student_info(name: &str, age: i32, student: bool) {
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Student: {}", student);
}

// 3. Multiple Parameters + Return
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// 4. คำนวณราคาสินค้า
fn calculate_total(price: f64, quantity: i32) -> f64 {
    price * quantity as f64
}

// 5. ค่าเฉลี่ย
fn average(a: f64, b: f64, c: f64) -> f64 {
    (a + b + c) / 3.0
}

fn main() {

    // 1. ส่ง 2 Arguments
    introduce("John", 20);

    println!("----------------");

    // 2. ส่ง 3 Arguments
    student_info("Alice", 21, true);

    println!("----------------");

    // 3. Multiple Parameters + Return
    let result = add(10, 20);
    println!("10 + 20 = {}", result);

    println!("----------------");

    // 4. คำนวณราคา
    let total = calculate_total(100.0, 3);
    println!("Total price = {}", total);

    println!("----------------");

    // 5. ค่าเฉลี่ย
    let avg = average(10.0, 20.0, 30.0);
    println!("Average = {}", avg);

}