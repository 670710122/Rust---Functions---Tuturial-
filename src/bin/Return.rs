
// 1. Function ที่ Return ค่า
fn add(x: i32, y: i32) -> i32 {
    x + y
}
// 2. Return เป็น boolean
fn is_adult(age: i32) -> bool {
    age >= 18
}
// 3. Return เป็น decimal
fn calculate_price(price: f64, quantity: f64) -> f64 {
    price * quantity
}
// 4. ใช้ return เพื่อออกจาก Function ก่อน
fn check_age(age: i32) -> bool {
    if age < 18 {
        return false;
    }
    true
}
fn main() {
    // รับค่าที่ Return มาเก็บไว้
    let result = add(10, 20);
    println!("Result = {}", result);

    // เอาค่า Return ไปคำนวณต่อ
    let final_result = add(10, 20) * 2;
    println!("Final Result = {}", final_result);

    // Return bool
    let result = is_adult(20);
    println!("Is adult? {}", result);

    // Return decimal
    let price = calculate_price(99.5, 2.0);
    println!("Price = {}", price);

    // ใช้ return
    println!("Age 15: {}", check_age(15));
    println!("Age 20: {}", check_age(20));

//Parameter = ช่องรับข้อมูล
//Argument  = ข้อมูลที่ส่งเข้าไป
}