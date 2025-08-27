use std::error::Error;
use std::thread;
use std::time::Duration;

fn is_even(num: i32) -> Result<(), Box<dyn Error>> {
    if num % 2 == 0 {
        return Ok(());
    }
    Err(String::from("Not even"))?
}

fn main() {
    let th = thread::spawn(|| {
        for i in 0..10 {
            println!("thread {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 0..10 {
        println!("main {}", i);
        thread::sleep(Duration::from_millis(50));
    }

    th.join().unwrap();
    println!("---------------------------------------------------------");

    match is_even(1) {
        Ok(_) => {}
        Err(e) => println!("1 {}", e),
    }

    match is_even(2) {
        Ok(_) => {}
        Err(e) => println!("2 {}", e),
    }
}
