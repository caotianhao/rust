fn main() {
    println!("a {}", add(1, 6));
    println!("Hello, world!");

    let sum = add(1, 660);
    println!("aa {}", sum)
}

fn add(a: i64, b: i64) -> i64 {
    a +  b
}
