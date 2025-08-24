use rand::Rng;

fn get_rand() -> i32 {
    rand::rng().random_range(1..=100)
}

fn main() {
    println!("hello");
    let n = get_rand();
    println!("随机数: {}", n);
}
