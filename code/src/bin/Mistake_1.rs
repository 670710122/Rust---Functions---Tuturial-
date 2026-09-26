fn add_i32(x:i32 , y:i32) -> i32{
    x + y
}

fn add_f64(x: f64, y: f64) -> f64 {
    x + y
}
// mistake 1
// fn add(x:i32 , y:i32) -> i32{
//     x + y
// }

// fn add(x: f64, y: f64) -> f64 {
//     x + y
// }

fn main() {
    let sum1 = add_i32(2,5);
    let sum2 = add_f64(2.2,5.5);
    println!("sum1 = {}",sum1); // sum1 = 7
    println!("sum2 = {}",sum2); // sum2 = 7.7
}


