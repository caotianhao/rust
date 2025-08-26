fn main() {
    let s1 = "s1";
    let s2 = String::new();
    let mut s3 = String::from("s3");

    println!("s1: {}, s1.len: {}", s1, s1.len());
    println!("s2: {}, s2.len: {}", s2, s2.len());
    println!("s3: {}, s3.len: {}", s3, s3.len());

    s3.push_str("45678"); // 追加
    println!("s3 push: {}", s3);
    let t = s3.pop(); // 删除并返回
    println!("t: {:?}, s3 pop: {}", t, s3);

    let mut s4 = String::from("abcdefghijklmnop");
    s4.truncate(14); // 保留前 14 个字符
    println!("s4: {}", s4);
    s4.drain(2..4); // 删除 2,3
    println!("s4 drain: {}", s4);
    println!("s4: replace {}", s4.replace("be", "BE"));

    let s5 = "s5".to_string();
    let s6 = "s6".to_string();
    let s7 = s5.clone() + &s6;
    println!("s5: {}, s6: {}, s7: {}", s5, s6, s7);

    let i1 = 2;
    let res1 = match i1 {
        1 => "1",
        2 => "2",
        2..=6 => "23456",
        _ => "other",
    };
    println!("i1: {}, res1: {}", i1, res1);
}
