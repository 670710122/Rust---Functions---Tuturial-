# Rust Functions Tuturial

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

`[Topic นี้เกี่ยวข้องกับ syntax อย่างไร]`
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
  4) Function ที่มี Return Value
     ```rust
     fn function_name() -> ReturnType {
        value
     }
     ```
### 9.2 Semantics

`[คำสั่ง/construct เหล่านี้มีความหมายหรือพฤติกรรมอย่างไร]`
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

`[เกี่ยวข้องกับ type system อย่างไร ถ้ามี]`

### 9.4 Memory / Resource Management

`[เกี่ยวข้องกับ memory หรือ resource management อย่างไร ถ้ามี]`
Rust เป็นภาษาแบบ Statically Typed หมายความว่า ชนิดข้อมูลของ parameters และ return value ของ function จะถูกตรวจสอบตอน Compile ก่อนโปรแกรมทำงาน โดยชนิดของ arguments ที่ส่งเข้า function และค่าที่ function คืนกลับต้องสอดคล้องกับชนิดที่ประกาศไว้ หากชนิดข้อมูลไม่ตรงกันจะเกิด Compile-time Error

### 9.5 Abstraction / Other PPL Concepts

`[อธิบาย abstraction, scope, binding, paradigm หรือแนวคิด PPL อื่นที่เกี่ยวข้อง]`
#### Abstraction
Function เป็น **Procedural Abstraction** คือการรวมขั้นตอนการทำงานไว้ภายใต้ชื่อเดียว ผู้เรียกสนใจเพียงว่า Function รับอะไรเข้าไป และคืนอะไรออกมา โดยไม่จำเป็นต้องรู้รายละเอียดภายในทุกขั้นตอน
#### Scope
Rust ใช้ **Lexical Scope หรือ Static Scope** กล่าวคือ scope ของตัวแปรพิจารณาได้จากโครงสร้างของ source code
#### Binding
เมื่อมีการเรียก Function ค่า **arguments** จะถูกผูก (bind) เข้ากับ **parameters** ของ Function เพื่อให้สามารถนำค่าเหล่านั้นไปใช้งานภายใน Function ได้
#### Paradigm
Rust เป็นภาษาแบบ **Multi-paradigm** ซึ่งรองรับแนวทางการเขียนโปรแกรมหลายรูปแบบ เช่น Imperative Programming และ Functional Programming ในส่วนของ Functions นั้น Rust รองรับแนวคิดแบบ Functional เช่น การใช้ Function เพื่อรับและคืนค่า รวมถึง Closures และ Higher-order Functions ทำให้สามารถเลือกแนวทางการเขียนโปรแกรมให้เหมาะสมกับงานได้
#### Other : Ownership & Borrowing
การเรียกใช้ Function ใน Rust มีความเกี่ยวข้องกับระบบ **Ownership** ของภาษา โดยเมื่อส่งค่าเข้าไปใน Function ค่านั้นอาจถูก ย้ายความเป็นเจ้าของ (Move), คัดลอก (Copy) หรือ ยืมไปใช้ (Borrow) ขึ้นอยู่กับชนิดข้อมูลและวิธีการส่งค่า ซึ่งกลไกเหล่านี้ช่วยให้ Rust สามารถจัดการหน่วยความจำได้อย่างปลอดภัย (Memory Safety) โดยไม่จำเป็นต้องใช้ Garbage Collector.

### 9.6 Why Rust?

`[Rust ใช้แนวคิดนี้เพื่อเพิ่ม safety, reliability หรือ performance อย่างไร]`
Rust ออกแบบ Functions ให้ทำงานร่วมกับ **Type System และ Ownership & Borrowing** เพื่อเพิ่มความปลอดภัย ความน่าเชื่อถือ และประสิทธิภาพของโปรแกรม
#### Safety
Rust ตรวจสอบชนิดข้อมูลของ **parameters และ return values** ตั้งแต่ Compile Time รวมถึงตรวจสอบกฎ Ownership และ Borrowing เมื่อมีการส่งข้อมูลระหว่าง Functions จึงช่วยป้องกันข้อผิดพลาดด้านชนิดข้อมูลและปัญหาการจัดการหน่วยความจำ
#### Reliability
กฎที่ชัดเจนเกี่ยวกับ **Scope, Type และ Ownership** ช่วยให้พฤติกรรมของ Function คาดเดาได้มากขึ้น และตรวจพบข้อผิดพลาดหลายประเภทก่อนโปรแกรมทำงานจริง
**Performance**
Rust สามารถส่งข้อมูลเข้า Function ได้ทั้งแบบ **Move, Copy และ Borrow** โดยการ Borrow ผ่าน reference ช่วยให้ Function เข้าถึงข้อมูลได้โดยไม่จำเป็นต้องคัดลอกข้อมูลทั้งหมด และ Rust สามารถจัดการหน่วยความจำได้โดยไม่ต้องพึ่ง Garbage Collector

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java / Kotlin / ...]`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `[อธิบาย]` | `[อธิบาย]` |
| Semantics / Behavior | `[อธิบาย]` | `[อธิบาย]` |
| Type System | `[อธิบาย]` | `[อธิบาย]` |
| Memory Management | `[อธิบาย]` | `[อธิบาย]` |
| Safety | `[อธิบาย]` | `[อธิบาย]` |

### Rust Example

```rust
// Rust code
```

### `[Other Language]` Example

```python
# Other language code
```

### Analysis

`[อธิบายความแตกต่างที่สำคัญ และเหตุผลด้านการออกแบบภาษา]`

---

