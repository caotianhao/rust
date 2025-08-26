fn main() {
    let t1 = (15, String::from("test"));
    println!("main: {:?}", t1);
    println!("t1.1 = {}", t1.1);
    show_tuple(t1.clone());

    // 解构
    let (s1, s2) = t1;
    println!("{}-{}", s1, s2);
}

fn show_tuple(t: (i32, String)) {
    println!("show_tuple: {:?}", t);
}
