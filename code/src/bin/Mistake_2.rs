fn greet(name: &str, msg: Option<&str>) {
    let greeting = match msg {
        Some(msg) => msg,
        None => "Hello",
    };
    println!("{}, {}!", greeting, name);
}

//mistake 2
// fn greet(name: &str, msg: &str = "Hello") {
//     println!("{}, {}!",msg,name);
// }
fn main() {
    greet("Alice", None); // Hello, Alice!               
    greet("Bob", Some("Good morning"));  // Good morning, Bob!
}