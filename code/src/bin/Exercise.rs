// exercise 1 Is even
fn is_even(number: i32){
    if number % 2 == 0 {
        println!("True");
    } else {
        println!("False");
    }
}

// exercise 2 Discount Calculator
fn discount_calculate(price: f64, discount: Option<f64>) -> f64 {
    match discount {
        Some(discount) => {
            if discount >= 0.0 && discount <= 100.0 { 
                price * (1.0 - discount / 100.0)
            }else{ 
                price
            }
        }
        None => price, 
    }
}

fn main() {
      // --- Exercise 1 ---
    println!("\n--- Exercise 1 ---");
    let num1 = 4;
    let num2 = 7;

    is_even(num1); // True
    is_even(num2); // False
    // --- Exercise 2 ---
    let price = 100.0;
    println!("--- Exercise 2 ---");
    println!("Price 1: {:.1}", discount_calculate(price, Some(60.0))); // 40.0
    println!("Price 2: {:.1}", discount_calculate(price, None));       // 100.0
    println!("Price 3: {:.1}", discount_calculate(price, Some(150.0)));// 100.0
}