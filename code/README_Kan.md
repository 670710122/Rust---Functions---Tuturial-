

## 7. Common Mistakes

### Mistake 1 — Function Overloading

**Problem**

บางครั้งอาจเกิดการสับสนในการเขียน และคิดว่าภาษา rust สามารถทำ `Function Overloading` ได้ (การสร้างฟังก์ชันชื่อเดียวกัน แต่รับ Parameter หรือมี Type ต่างกัน)  แต่ในความเป็นจริงแล้ว rust ไม่รองรับการทำ Function Overloading โดยตรง

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

**Why?**

* เพื่อป้องกันความสับสนของ `Type Inference` : หากมี Overloading อาจทำให้ Compiler สับสนหรือคาดเดาประเภทข้อมูลผิดจากที่ผู้เขียนตั้งใจ

* แนวทางใกล้เคียง : ใช้ `Generics` หรือ `Traits` แทน Function Overloading 

---

### Mistake 2 — Default Parameter

**Problem**

ภาษา rust ไม่รองรับการทำ `default parameter`(การกำหนดค่าเริ่มให้กับฟังก์ชัน)

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

**Why?**

* เพื่อความชัดเจน (Explicit over Implicit): Rust เน้นให้ผู้อ่านโค้ดมองเห็นสิ่งที่เกิดขึ้นอย่างชัดเจนที่สุด โดยไม่มีระบบซ่อนการทำงานอยู่เบื้องหลัง

* แนวทางใกล้เคียง : ใช้ `Option<T>` หรือ `Default Trait` แทน Default Parameters

---

## 8. Exercises

### Exercise 1 — Is_Even

**Problem**

จงเขียนฟังก์ชันชื่อ `is_even` ที่รับตัวเลขจำนวนเต็ม `(i32)` เข้ามา 1 ตัว แล้วทำการตรวจสอบว่าตัวเลขนั้นเป็น เลขคู่ (Even Number) หรือไม่
* หากเป็นเลขคู่ ให้พิมพ์คำว่า "`True`"
* หากเป็นเลขคี่ ให้พิมพ์คำว่า "`False`"

**Hint**

* สามารถตรวจสอบเลขคู่ โดยใช้ตัวดำเนินการ Modulo (%)

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
**Explanation**

* `fn is_even(number: i32)`  : สร้างฟังก์ชันชื่อ `is_even`  ที่รอรับค่าตัวเลขจำนวนเต็ม `(i32)` เข้ามาผ่านพารามิเตอร์ชื่อ `number`
* `if number % 2 == 0`  : นำเลขมาหารด้วย 2 ถ้าหารลงตัว เศษเป็น 0 หมายความว่าตัวเลขนั้นเป็นเลขคู่ ให้พิมพ์ `"True"` 
* `else`  : ถ้าหารไม่ลงตัว หมายความว่าตัวเลขนั้นเป็นเลขคี่ ให้พิมพ์ `"False"` 
* `main()`  : มีการประกาศตัวแปรที่เป็นตัวเลข และส่งตัวแปรไปเช็ค โดยส่ง 4 ได้ผลลัพธ์เป็น True และส่ง 7 ไปเช็ค ได้ผลลัพธ์เป็น `False`

---
### Exercise 2 — Discount Calculator

**Problem**

จงเขียนฟังก์ชันชื่อ `discount_calculate` เพื่อคำนวณราคาสุทธิหลังหักส่วนลด `return` ค่าเป็น `(f64)`โดยรับพารามิเตอร์ 2 ตัวคือ:
* `price` (`f64`): ราคาเต็มของสินค้า 
* `discount` (`Option<f64>`): เปอร์เซ็นต์ส่วนลด (อาจจะมีค่า หรือไม่มีค่าก็ได้)

เงื่อนไขการทำงาน:
* หากมีค่าส่วนลด `(Some)` และค่านั้น **อยู่ในช่วง $0.0$ ถึง $100.0$** ให้คำนวณราคาหลังหักส่วนลดเป็นทศนิยม 1 ตำแหน่ง
* หากไม่มีส่วนลด `(None)` หรือส่วนลด **ไม่อยู่ในช่วง $0.0$ ถึง $100.0$** ให้คืนค่าราคาเต็ม (`price`) กลับไป

**Hint**

* ใช้ `match` ในการแยกกรณีระหว่าง `Some(discount)` และ `None`
* ซ้อนคำสั่ง `if / else` ไว้ภายในบล็อก `Some` เพื่อเช็คว่าส่วนลดไม่ติดลบและไม่เกิน $100.0$
* ใน `println!` สามารถใช้ `{:.1}` เพื่อแสดงผลลัพธ์เป็นทศนิยม 1 ตำแหน่งได้

**Solution**

```rust
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
    let price = 100.0;
    println!("--- Exercise 2 ---");
    println!("Price 1: {:.1}", discount_calculate(price, Some(60.0))); // 40.0
    println!("Price 2: {:.1}", discount_calculate(price, None));       // 100.0
    println!("Price 3: {:.1}", discount_calculate(price, Some(150.0)));// 100.0
}
```

**Explanation**

* `match discount`
    * กรณี `Some(discount)`: มีการส่งค่าส่วนลดเข้ามา ให้ดึงตัวเลขนั้นออกมาตรวจสอบต่อใน`if / else`
    * กรณี `None`: ไม่มีส่วนลดส่งเข้ามา ให้คืนค่า `price` (ราคาเต็ม) ออกไปทันที

* `println!("Price 1: {:.1}", ...)`: `{:.1}` เป็นการกำหนดให้ตัวเลขแสดงจุดทศนิยม 1 ตำแหน่ง (เช่น 40.0, 100.0)

---



