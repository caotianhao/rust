use std::sync::atomic::{AtomicI8, Ordering};

const NAME: &str = "aa";
static NAME2: i64 = 15;
static NAME3: AtomicI8 = AtomicI8::new(12);

fn main() {
    println!("test for unbin folders");
    println!("{} {}", i64::MAX, i128::MAX);
    println!("{} {}", NAME, NAME2);

    println!("{}", NAME3.load(Ordering::SeqCst));
    NAME3.store(11, Ordering::SeqCst);
    println!("{}", NAME3.load(Ordering::SeqCst));
}
