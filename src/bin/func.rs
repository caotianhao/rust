fn main() {
    let i1 = 16;
    println!("i1 * 2: {}", double_num(i1)); // 32
    println!("i1 {}", i1); // 16

    let mut i2 = 16;
    println!("i2 * 2: {}", double_num2(&mut i2)); // 32
    println!("i2 {}", i2); // 32

    let s1 = "Hello";
    show_name(s1);
    println!("s1: {}", s1);

    let s2 = String::from("Hello");
    show_name2(s2.clone());
    println!("s2: {}", s2);
}

fn double_num(mut n: i32) -> i32 {
    n = n * 2;
    n
}

fn double_num2(n: &mut i32) -> i32 {
    *n = *n * 2;
    *n
}

fn show_name(s: &str) {
    println!("s: {}", s);
}

fn show_name2(s: String) {
    println!("s: {}", s);
}
