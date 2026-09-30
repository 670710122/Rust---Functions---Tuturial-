# Rust Functions Tuturial

## 9. PPL Perspective

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

ภาษา Rust ประกาศฟังก์ชันด้วย keyword fn ตามด้วยชื่อฟังก์ชัน วงเล็บสำหรับ parameters และ { } สำหรับส่วน body ของฟังก์ชัน
  1) Function พื้นฐาน
     ```rust
     fn function_name() {
        // คำสั่งที่ต้องการให้ function ทำงาน
     }
     ```
  2) Function ที่มี Parameters
  ```rust
      fn function_name(parameter: Type) {
         // function body
      }
  ```
    ((function parameter ของ Rust เป็นส่วนหนึ่งของ static type system = รู้ตั้งแต่ compile))
  3) Function ที่มี Return Value
     ```rust
     fn function_name() -> ReturnType {
        value
     }
     ```
### 9.2 Semantics

`เมื่อมีการเรียก Function ค่า arguments จะถูกผูกกับ parameters จากนั้นคำสั่งและ expressions ภายใน function body จะถูกประมวลผล และค่าของ expression สุดท้ายที่ไม่มี semicolon (;) สามารถใช้เป็น return value ได้โดยอัตโนมัติ นอกจากนี้สามารถใช้ return เมื่อต้องการคืนค่าออกจาก Function โดยตรงได้เช่นกัน`
ตัวอย่างที่ไม่ได้ใส่ semicolon (;)
```rust
    fn add_one(x: i32) -> i32 {
      x + 1       // คืนค่า i32
    }
```
Expression สุดท้ายจะเป็นค่าที่ Function คืนกลับโดยอัตโนมัติ

ตัวอย่างที่ใส่ semicolon (;)
```rust
    fn add_one(x: i32) -> i32 {
      x + 1;      // ได้ () แต่ต้องการ i32 → Error
    }
```
ผลลัพธ์ของ expression จะไม่ถูกใช้เป็นค่าที่ Function คืน ทำให้ body มีค่าเป็น () และเกิด error หาก Function กำหนดว่าต้องคืน i32
### 9.3 Type System

Rust เป็นภาษาแบบ Statically Typed หมายความว่า ชนิดข้อมูลของ parameters และ return value ของ function จะถูกตรวจสอบตอน Compile ก่อนโปรแกรมทำงาน โดยชนิดของ arguments ที่ส่งเข้า function และค่าที่ function คืนกลับต้องสอดคล้องกับชนิดที่ประกาศไว้ หากชนิดข้อมูลไม่ตรงกันจะเกิด Compile-time Error

### 9.4 Memory / Resource Management

Rust ผ่านระบบ Ownership และ Borrowing เมื่อส่งข้อมูลเข้า Function ค่าอาจถูก Move, Copy หรือ Borrow ขึ้นอยู่กับชนิดข้อมูลและวิธีการส่งค่า เมื่อเจ้าของข้อมูลออกจาก Scope Rust จะทำลายข้อมูลและคืน Resource โดยอัตโนมัติ ซึ่งช่วยลดปัญหาเกี่ยวกับหน่วยความจำ เช่น dangling references และช่วยให้จัดการหน่วยความจำได้อย่างปลอดภัยโดยไม่ต้องใช้ Garbage Collector

### 9.5 Abstraction / Other PPL Concepts

#### Abstraction
Function เป็น **Procedural Abstraction** (การรวมขั้นตอนการทำงานไว้ภายใต้ชื่อเดียว) ผู้เรียกสนใจเพียงว่า Function รับอะไรเข้าไป และคืนอะไรออกมา โดยไม่จำเป็นต้องรู้รายละเอียดภายในทุกขั้นตอน
#### Scope
Rust ใช้ **Lexical Scope หรือ Static Scope** คือ scope ของตัวแปรพิจารณาได้จากโครงสร้างของ source code
#### Binding
เมื่อมีการเรียก Function ค่า **arguments** จะถูก binding เข้ากับ **parameters** ของ Function เพื่อให้สามารถนำค่าเหล่านั้นไปใช้งานภายใน Function ได้
#### Paradigm
Rust เป็นภาษาแบบ Multi-paradigm รองรับทั้ง Imperative, Functional และ Object-oriented บางส่วน (ผ่าน struct, impl, trait) ในส่วนของ Functions นั้น Rust ถือว่า Function เป็น first-class value คือเก็บในตัวแปร ส่งเป็น argument และคืนเป็น return value ได้ รองรับ Closures ที่จับตัวแปรจากบริบทรอบข้างได้ และ Higher-order Functions เช่น map และ filter อย่างไรก็ตาม Rust ไม่ใช่ภาษา Functional แบบ pure เพราะ Function มี side effect ได้
#### Ownership & Borrowing
การเรียกใช้ Function ใน Rust มีความเกี่ยวข้องกับระบบ **Ownership** ของภาษา โดยเมื่อส่งค่าเข้าไปใน Function ค่านั้นอาจถูก ย้ายความเป็นเจ้าของ (Move), คัดลอก (Copy) หรือ ยืมไปใช้ (Borrow) ขึ้นอยู่กับชนิดข้อมูลและวิธีการส่งค่า โดยไม่จำเป็นต้องใช้ Garbage Collector.

### 9.6 Why Rust?

Rust ออกแบบ Functions ให้ทำงานร่วมกับ **Type System และ Ownership & Borrowing** เพื่อเพิ่มความปลอดภัย ความน่าเชื่อถือ และประสิทธิภาพของโปรแกรม
#### Safety
Rust ตรวจสอบชนิดข้อมูลของ **parameters และ return values** ตั้งแต่ Compile Time รวมถึงตรวจสอบกฎ Ownership และ Borrowing เมื่อมีการส่งข้อมูลระหว่าง Functions จึงช่วยป้องกันข้อผิดพลาดด้านชนิดข้อมูลและปัญหาการจัดการหน่วยความจำ
#### Reliability
กฎที่ชัดเจนเกี่ยวกับ **Scope, Type และ Ownership** ช่วยให้พฤติกรรมของ Function คาดเดาได้มากขึ้น และตรวจพบข้อผิดพลาดหลายประเภทก่อนโปรแกรมทำงานจริง
#### Performance
Rust สามารถส่งข้อมูลเข้า Function ได้ทั้งแบบ **Move, Copy และ Borrow** โดยการ Borrow ผ่าน reference ช่วยให้ Function เข้าถึงข้อมูลได้โดยไม่จำเป็นต้องคัดลอกข้อมูลทั้งหมด และ Rust สามารถจัดการหน่วยความจำได้โดยไม่ต้องพึ่ง Garbage Collector

---

## 10. Rust vs. Other Language

**Comparison Language:** `Python`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `ใช้ { } กำหนด block และมักใช้ ; ปิดท้าย statement ประกาศตัวแปรด้วย let` | `ใช้ indentation กำหนด block ไม่ใช้ { } และไม่จำเป็นต้องใช้ ; การประกาศตัวแปรทำได้โดยกำหนดค่าโดยตรง` |
| Semantics / Behavior | `ตรวจสอบข้อผิดพลาดตั้งแต่ Compile Time` | `ตรวจสอบข้อผิดพลาดขณะ Runtime` |
| Type System | `Statically Typed และ Strongly Typed` | `Dynamically Typed และ Strongly Typed` |
| Memory Management | `Ownership, Borrowing, Lifetimes และไม่ใช้ Garbage Collector` | `Automatic Memory Management โดยหลักผ่าน Reference Counting ร่วมกับ Garbage Collector` |
| Safety | `Memory Safety และ Thread Safety โดย compiler ตรวจสอบ ownership, borrowing และ lifetime ช่วยป้องกันปัญหา` | `จัดการ raw memory โดยตรงในโค้ด Python ทั่วไป` |

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

### Analysis

Rust เน้นความปลอดภัยและประสิทธิภาพ ด้วย Static Typing และระบบ Ownership ที่ตรวจสอบตั้งแต่ Compile Time จึงช่วยลดข้อผิดพลาดด้าน Memory ได้โดยไม่ต้องใช้ Garbage Collector ส่วน Python เน้นความเรียบง่ายและยืดหยุ่น ด้วย Dynamic Typing และ Automatic Memory Management ทำให้เขียนและพัฒนาโปรแกรมได้ง่ายกว่า แต่ข้อผิดพลาดบางอย่างอาจตรวจพบเมื่อ Runtime

---

**Comparison Language:** `Java`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `ใช้ { } กำหนด block และ ; ปิดท้าย statement ประกาศตัวแปรด้วย let` | `ใช้ { } กำหนด block และ ; ปิดท้าย statement โดยทั่วไปต้องระบุชนิดข้อมูล` |
| Semantics / Behavior | `ตรวจสอบข้อผิดพลาดตั้งแต่ Compile Time` | `Compile เป็น Bytecode และโดยทั่วไปทำงานผ่าน JVM` |
| Type System | `Statically Typed และ Strongly Typed` | `Statically Typed และ Strongly Typed และรองรับ Type Inference ใน local variables ด้วย var` |
| Memory Management | `Ownership, Borrowing, Lifetimes และไม่ใช้ Garbage Collector` | `ใช้ Garbage Collector (GC) จัดการ Memory ของ Object ที่ไม่ถูกใช้งานโดยอัตโนมัติ` |
| Safety | `Memory Safety และ Thread Safety โดย compiler ตรวจสอบ ownership, borrowing และ lifetime ช่วยป้องกันปัญหา` | `JVM และ GC ช่วยลดปัญหาการจัดการ Memory โดยตรง แต่ยังสามารถเกิดข้อผิดพลาดขณะ Runtime` |

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

### Analysis

Rust และ Java เป็นภาษาแบบ Statically Typed เหมือนกัน แต่แตกต่างกันชัดเจนด้านการจัดการ Memory
Rust ใช้ Ownership และ Borrowing เพื่อตรวจสอบและจัดการ Memory ตั้งแต่ Compile Time โดยไม่ใช้ Garbage Collector ส่วน Java ใช้ Garbage Collector จัดการ Memory ขณะ Runtime

---

**Comparison Language:** `C++`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `ใช้ { } กำหนด block และ ; ปิดท้าย statement ประกาศตัวแปรด้วย let` | `ใช้ { } กำหนด block และ ; ปิดท้าย statement ระบุชนิดข้อมูลตอนประกาศ` |
| Semantics / Behavior | `ตรวจสอบข้อผิดพลาดตั้งแต่ Compile Time` | `Programmer ต้องระมัดระวังข้อผิดพลาดเกี่ยวกับ Memory ด้วยตนเอง` |
| Type System | `Statically Typed และ Strongly Typed` | `Statically Typed และรองรับ Type Inference ผ่าน auto` |
| Memory Management | `Ownership, Borrowing, Lifetimes และไม่ใช้ Garbage Collector` | `รองรับทั้ง Automatic Storage, RAII, Smart Pointers และการจัดการ Dynamic Memory โดยตรง` |
| Safety | `Memory Safety และ Thread Safety โดย compiler ตรวจสอบ ownership, borrowing และ lifetime ช่วยป้องกันปัญหา` | `มีความยืดหยุ่นสูง แต่การใช้ Raw Pointer หรือจัดการ Memory ไม่ถูกต้องอาจทำให้เกิด Dangling Pointer, Use-after-free หรือ Memory Leak ได้` |

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

### Analysis

Rust และ C++ เป็นภาษาที่เน้น ประสิทธิภาพและการควบคุมทรัพยากร เช่นเดียวกัน แต่มีแนวทางด้าน Memory Safety แตกต่างกัน
Rust ใช้ Ownership, Borrowing และ Lifetime ให้ Compiler ตรวจสอบความปลอดภัยของ Memory ตั้งแต่ Compile Time ส่วน C++ ให้อิสระแก่ Programmer ในการจัดการ Memory และ Pointer มากกว่า จึงมีความยืดหยุ่นสูง แต่ต้องระมัดระวังข้อผิดพลาดด้าน Memory มากกว่า

---
