## 3. Introduction

`Function ในภาษา Rust คือ ชุดคำสั่งย่อยที่ประกาศด้วย fn ใช้จัดระเบียบตรรกะและสามารถเรียกใช้งานซ้ำได้`<br>
`ช่วยให้โค้ดเป็นระเบียบ อ่านง่าย ลดความซ้ำซ้อน และปลอดภัยด้วยระบบตรวจสอบชนิดข้อมูล (Type System) ที่เข้มงวด`<br>
`แก้ปัญหาโค้ดซ้ำซ้อน (Duplication), โค้ดซับซ้อนแก้ไขยาก และข้อผิดพลาดจากการส่งชนิดข้อมูลผิดพลาดตั้งแต่ตอนคอมไพล์`

---

## 4. Key Concepts

### 4.1 `[การประกาศฟังก์ชัน (Function Declaration)]`

**คำอธิบาย**

`ใช้คีย์เวิร์ด fn ตามด้วยชื่อฟังก์ชัน` <br>
`ต้องระบุชนิดข้อมูล (Data Type) ของพารามิเตอร์ทุกตัวอย่างชัดเจนเสมอ` <br>
`หากฟังก์ชันมีการคืนค่า (Return value) ต้องใส่เครื่องหมาย -> ตามด้วยชนิดข้อมูลที่คืนค่า`

**ตัวอย่าง**

```rust
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```

**Explanation**

`fn add_numbers(a: i32, b: i32) -> i32: ประกาศฟังก์ชันชื่อ add_numbers รับพารามิเตอร์ a และ b ชนิด i32 และกำหนดให้คืนค่า (Return) ออกมาเป็นชนิด i32` <br>
`a + b: เป็นการคืนค่าผลบวกของตัวแปร a และ b`

---

### 4.2 `Statement vs Expression (หัวใจสำคัญของ Rust)`

`ภาษา Rust แยกแยะระหว่าง Statement และ Expression อย่างชัดเจน ซึ่งส่งผลต่อการคืนค่าของฟังก์ชัน:` <br><br>

`Statement: คือคำสั่งที่ทำงานบางอย่างแต่ ไม่คืนค่า มักจะลงท้ายด้วยเครื่องหมายเซมิโคลอน ;`<br>
`Expression: คือนิพจน์ที่ ประเมินผลและคืนค่าออกมา (เช่น 5 + 6, การเรียกฟังก์ชัน, หรือแม้แต่บล็อกโค้ด { let x = 3; x + 1 })` <br><br>

`กฎการ Return: บรรทัดสุดท้ายของฟังก์ชันใน Rust หาก ไม่มีเซมิโคลอน (;) ต่อท้าย จะถือว่าเป็น Expression และถูกใช้เป็นค่า Return ของฟังก์ชันนั้นทันที (ไม่ต้องใช้คำว่า return) แต่ถ้าใส่ ; จะกลายเป็น Statement ที่ไม่คืนค่า (ซึ่งถ้าฟังก์ชันต้องการคืนค่า i32 แต่ดันใส่ ; จะเกิด Error ทันที)`

---

### 4.3 `[การคืนค่าหลายค่า (Multiple Return Values)]`

`Rust ไม่มี Syntax พิเศษสำหรับการคืนค่าหลายค่าโดยตรง แต่เราสามารถใช้ Tuple หรือ Struct เพื่อคืนค่ามากกว่าหนึ่งค่าได้อย่างง่ายดาย`

```rust
fn get_user_info() -> (String, u32) {

    ("Alice".to_string(), 30)
}
```

---

### 4.4 `[การจัดการ Ownership และ Borrowing กับ Function]`

`เนื่องจาก Rust ใช้ระบบ Memory Management แบบ Ownership การส่งตัวแปร (เช่น String หรือ Vector) เข้าไปในฟังก์ชัน จะมีผลต่อสิทธิ์การใช้งานตัวแปรนั้น:` <br><br>

`Move: ถ้าส่งตัวแปรเข้าไปตรงๆ (เช่น print_string(s)) สิทธิ์ความเป็นเจ้าของจะย้าย (Move) เข้าไปในฟังก์ชัน ตัวแปรเดิมข้างนอกจะใช้งานต่อไม่ได้` <br>
`Borrowing (References): หากต้องการให้ฟังก์ชันยืมไปใช้งานเฉยๆ โดยที่ข้างนอกยังใช้ต่อได้ ให้ส่งผ่าน Reference (& หรือ &mut) แทน`

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

---

### 4.5 `Diverging Functions (ฟังก์ชันที่ไม่เคยคืนค่า)`

`Rust มีฟังก์ชันพิเศษที่เรียกว่า Diverging function ซึ่งใช้เครื่องหมาย ! เป็น Return type หมายความว่าฟังก์ชันนี้ทำงานแล้วไม่มีวันจบปกติ (เช่น ฟังก์ชันที่โปรแกรมจะพังหรือวนลูปไม่รู้จบ)`

```rust
fn panic_error() -> ! {
    panic!("Crash program!");
}
```

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `fn name(param: Type) -> ReturnType` | `การประกาศฟังก์ชัน กำหนดชื่อ พารามิเตอร์พร้อม Type และชนิดข้อมูลที่ต้องคืนค่า` | `fn add(a: i32, b: i32) -> i32` |
| `บรรทัดสุดท้าย ไม่มี เซมิโคลอน (;)` | `ใช้เป็น Expression เพื่อคืนค่าอัตโนมัติ (Implicit Return) โดยไม่ต้องใช้คำสั่ง return` | `fn square(x: i32) -> i32 { x * x }` |
| `Return Type เป็นเครื่องหมายตกใจ -> !` | `Diverging Function ฟังก์ชันที่ไม่เคยคืนค่าปกติกลับมา (เช่น Panic หรือ Infinite Loop)` | `fn fail() -> ! { panic!("Error"); }` |
| `&mut T` | `Mutable Reference: การส่ง Reference เข้าฟังก์ชันเพื่อให้ฟังก์ชันสามารถแก้ไขค่าของตัวแปรต้นฉบับได้` | `fn update(s: &mut String) { s.push_str("!"); }` |
| `pub fn / pub(crate) fn` | `Visibility (การมองเห็น): กำหนดสิทธิ์การเข้าถึงฟังก์ชันจากภายนอกโมดูลหรือ crate` | `pub fn calculate() {}` |
| `dyn Trait / impl Trait` | `Generics & Traits: การสร้างฟังก์ชันที่รับหรือคืนค่าได้หลาย Type แบบ Polymorphism` | `fn print_item<T: Display>(item: T) {}` |
| `Closures (\|params\| body)` | `Anonymous Functions: ฟังก์ชันไม่มีชื่อที่สามารถจับตัวแปรจาก Environment รอบข้างได้` | `let add = \|a, b\| a + b;` |

### Important Rules

1. `Type Annotation is Mandatory: พารามิเตอร์ทุกตัวของฟังก์ชันในภาษา Rust ต้องระบุ Data Type เสมอ (คอมไพเลอร์จะไม่ช่วยเดา Type ให้เหมือนกับตัวแปรทั่วไปที่ใช้ let)`
2. `Statement vs Expression: ห้ามใส่เซมิโคลอน (;) ที่บรรทัดสุดท้ายของฟังก์ชันหากต้องการให้บรรทัดนั้นเป็นค่า Return ถ้าใส่จะกลายเป็น Statement และทำให้เกิดคอมไพล์เออร์เออร์เรอร์หากฟังก์ชันนั้นกำหนด Return Type ไว้`
3. `Ownership Transfer by Default: การส่งค่าตัวแปรประเภทที่ไม่ใช่ Primitive (เช่น String หรือ Vector) เข้าไปในฟังก์ชันแบบปกติจะทำให้เกิดการย้ายสิทธิ์ (Move) และไม่สามารถนำตัวแปรนั้นกลับมาใช้ซ้ำข้างนอกได้ เว้นแต่จะใช้ Reference (&) เพื่อยืมค่าแทน`
4. `Method vs Function: ฟังก์ชันที่ถูกผูกไว้กับ Struct หรือ Enum (ประกาศภายในบล็อก impl) ซึ่งจะมีพารามิเตอร์ตัวแรกเป็น self, &self, หรือ &mut self`
5. `Higher-Order Functions: ความสามารถในการรับฟังก์ชันอื่นเป็นพารามิเตอร์ หรือการคืนค่าฟังก์ชันออกจากฟังก์ชัน (มักใช้คู่กับ Closures และ Iterator เช่น .map() หรือ .filter())`

---
