use std::collections::{HashMap, HashSet};

fn main() {
    let mut v1 = Vec::new();
    v1.push("0");
    v1.push("1");
    v1.push("2");
    v1.push("3");
    v1.push("4");

    println!("v1.len() {}, v1 {:?}", v1.len(), v1);
    println!("{}", v1.contains(&"2"));
    v1.remove(2);
    println!("{}", v1.contains(&"2"));
    println!("v1.len() {}, v1 {:?}", v1.len(), v1);
    println!("---------------------------------------------------------");

    let mut h1 = HashMap::new();
    h1.insert("a", 1);
    h1.insert("b", 2);
    println!("h1.len() {}, h1 {:?}", h1.len(), h1);

    let mut h2 = HashSet::new();
    h2.insert("a");
    h2.insert("b");
    println!("h2.len() {}, h2 {:?}", h2.len(), h2);

    let a = h2.remove("a");
    let b = h2.remove("ba");
    println!("a: {:?}, b: {:?}", a, b);
}
