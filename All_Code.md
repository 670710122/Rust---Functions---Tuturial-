# All Code in Rust Functions Tuturial
## 4. Key Concepts

### 4.1 `[การประกาศฟังก์ชัน (Function Declaration)]`

```rust
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```

### 4.3 `[การคืนค่าหลายค่า (Multiple Return Values)]`

```rust
fn get_user_info() -> (String, u32) {

    ("Alice".to_string(), 30)
}
```

### 4.4 `[การจัดการ Ownership และ Borrowing กับ Function]`

```rust
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let my_string = String::from("Hello");
    print_length(&my_string);
    println!("{}", my_string);
}
```

### 4.5 `Diverging Functions (ฟังก์ชันที่ไม่เคยคืนค่า)`

```rust
fn panic_error() -> ! {
    panic!("Crash program!");
}
```

## 6. Runnable Code Examples

### Example 1 — `Factorial`
```rust
fn main() {
    let x = factorial(5);
    print!("{}", x)
}

fn factorial(num: i32) -> i32 {
    let mut result = 1;
    if num == 0 {
        return result;
    }
    else {
        for i in 1..=num {
            result *= i
        }
        return result;
    }
}
```

### Example 2 — `[discount]`

```rust
use std::io;
#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
}

fn create_discount(discount: f64) -> impl Fn(f64) -> f64 {
    move |price| price * (1.0 - discount)
}
fn calculate(
    products: &[Product],
    operation: impl Fn(f64) -> f64,
) {
    for product in products {
        let new_price = operation(product.price);

        println!(
            "{} : {:.2} -> {:.2}",
            product.name,
            product.price,
            new_price
        );
    }
}
fn input(message: &str) -> String {
    let mut value: String = String::new();
    println!("{}", message);
    io::stdin().read_line(&mut value).unwrap();
    value.trim().to_string()
}

fn main() {
    let name1 = input("Product 1 name:");
    let price1: f64 = input("Product 1 price:").parse().unwrap();
    let name2 = input("Product 2 name:");
    let price2: f64 = input("Product 2 price:").parse().unwrap();
    let name3 = input("Product 3 name:");
    let price3: f64 = input("Product 3 price:").parse().unwrap();

    let discount: f64 = input("Discount (%):").parse().unwrap();

    let products = [
        Product {
            name: name1,
            price: price1,
        },
        Product {
            name: name2,
            price: price2,
        },
        Product {
            name: name3,
            price: price3,
        },
    ];

    let discount_fn = create_discount(discount / 100.0);

    calculate(&products, discount_fn);
}


//let mut input = String::new();
//io::stdin().read_line(&mut input).unwrap();
//let age: i32 = input.trim().parse().unwrap();


```

## 7. Common Mistakes

### Mistake 1 — Function Overloading

**Incorrect Code**

```rust
fn add(x:i32 , y:i32) -> i32{
    x + y
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn main() {
    let sum1 = add(2,5);
    let sum2 = add(2.2,5.5);
    println!("sum1 = {}",sum1);
    println!("sum2 = {}",sum2);
}
```

**Correct Code**

```rust
fn add_i32(x:i32 , y:i32) -> i32{
    x + y
}

fn add_f64(x: f64, y: f64) -> f64 {
    x + y
}

fn main() {
    let sum1 = add_i32(2,5);
    let sum2 = add_f64(2.2,5.5);
    println!("sum1 = {}",sum1); // sum1 = 7
    println!("sum2 = {}",sum2); // sum2 = 7.7
}
```

### Mistake 2 — Default Parameter

**Incorrect Code**

```rust
fn greet(name: &str, msg: &str = "Hello") {
    println!("{}, {}!",msg,name);
}
fn main() {
    greet("Alice", None);               
    greet("Bob", Some("Good morning")); 
}
```

**Correct Code**

```rust
fn greet(name: &str, msg: Option<&str>) {
    let greeting = match msg {
        Some(msg) => msg,
        None => "Hello",
    };
    println!("{}, {}!", greeting, name);
}
fn main() {
    greet("Alice", None); // Hello, Alice!    
    greet("Bob", Some("Good morning")); // Good morning, Bob!
}
```

## 8. Exercises

### Exercise 1 — Is_Even

**Solution**

```rust
fn is_even(number: i32){
    if number % 2 == 0 {
        println!("True");
    } else {
        println!("False");
    }
}
fn main() {
    let num1 = 4;
    let num2 = 7;

    is_even(num1); // True
    is_even(num2); // False
}
```

### Exercise 2 — Grade_Check

**Solution**

```rust
fn grade_check(score: i32){
    if score >= 80 {
        println!("Exellent");
    }else if score >= 40 {
        println!("Pass");
    } else {
        println!("Fail");
    }
}
fn main() {
    grade_check(39); // Fail
    grade_check(40); // Pass
    grade_check(90); // Exellent
}
```

## 9. PPL Perspective

### 9.1 Syntax

2) Function ที่มี Parameters
```rust
      // Create a function
      fn say_hello() {
        println!("Hello from a function!");
      }

      say_hello(); // Call the function
```

3) Function ที่มี Return Value
```rust
     fn add(a: i32, b: i32) -> i32 {
      return a + b;
     }

     let sum = add(3, 4);
     println!("Sum is: {}", sum);
```

### 9.2 Semantics

```rust
    fn add_one(x: i32) -> i32 {
      x + 1
    }

    fn main() {
      let result = add_one(5);
      println!("{}", result);
    }
```

### 9.5 Abstraction / Other PPL Concepts

#### Abstraction
```rust
    fn average(s: &[f64]) -> f64 {
      s.iter().sum::<f64>() / s.len() as f64
    }
    // เรียกใช้: average(&[80.0, 90.0]) ไม่ต้องรู้ขั้นตอนภายใน
```
#### Scope
```rust
    let x = 10;
    {
        let y = 5;              // y อยู่แค่ใน block นี้
        println!("{}", x + y);
    }
    let x = x * 2;              // shadowing
    let show = || println!("{}", x); // closure เข้าถึง x ได้ (fn ซ้อนทำไม่ได้)
```
#### Binding
```rust
    fn add(a: i32, b: i32) -> i32 { a + b } // a, b = parameters
    let r = add(3, 4);                      // 3, 4 = arguments ถูกผูกเข้ากับ a, b

    // Static vs Dynamic binding
    fn f_static<T: Speak>(x: &T) { x.speak() }  // ผูกตอน compile
    fn f_dyn(x: &dyn Speak) { x.speak() }       // ผูกตอนรันไทม์ (vtable)
```
#### Paradigm
```rust
  // Imperative
  let mut sum = 0;
  for i in 1..=5 { sum += i; }

  // Functional: first-class function, closure, higher-order
  fn make_adder(n: i32) -> impl Fn(i32) -> i32 { move |x| x + n }
  let add5 = make_adder(5);

  let v: Vec<i32> = (1..=5).filter(|x| x % 2 == 1).map(|x| x * x).collect(); // [1, 9, 25]
```
#### Ownership & Borrowing
```rust
  fn take(s: String) {
    println!("take: {}", s);
  }

  fn len(s: &String) -> usize {
    s.len()
  }

  fn append(s: &mut String) {
    s.push('!');
  }

  fn main() {
    let a = String::from("hi");
    let n = 5;

    take(a);          // Move: a ใช้ต่อไม่ได้

    let m = n;        // Copy: n ยังใช้ได้ เพราะ i32 เป็น Copy
    println!("n = {}, m = {}", n, m);

    let mut b = String::from("hi");

    let length = len(&b);     // Immutable Borrow
    println!("length = {}", length);

    append(&mut b);           // Mutable Borrow
    println!("b = {}", b);
  }
```

## 10. Rust vs. Other Language

### Rust Example

```rust
  fn main() {
    let name: String = String::from("Rust");
    let length: usize = get_length(&name);

    println!("Language: {}", name);
    println!("Length: {}", length);
  }

  fn get_length(text: &String) -> usize {
    text.len()
  }
```

### `[Other Language]` Example

```python
  def get_length(text):
    return len(text)

  name = "Python"
  length = get_length(name)

  print("Language:", name)
  print("Length:", length)
```
### Rust Example

```rust
  fn main() {
    let name = String::from("Rust");
    print_name(&name);
    println!("{}", name);
  }

  fn print_name(name: &String) {
    println!("{}", name);
  }
```

### `[Other Language]` Example

```java
  public class Main {
      static void printName(String name) {
        System.out.println(name);
      }

      public static void main(String[] args) {
        String name = "Java"; printName(name);
        System.out.println(name);
      }
  }
```
### Rust Example

```rust
  fn main() {
    let name = String::from("Rust");
    print_name(&name);
    println!("{}", name);
  }

  fn print_name(name: &String) {
    println!("{}", name);
  }
```

### `[Other Language]` Example

```c++
  #include <iostream>
  #include <string>
  using namespace std;

  void printName(const string& name) {
    cout << name << endl;
  }

  int main() {
    string name = "C++";
    printName(name); cout << name << endl;
    return 0;
  }
```
