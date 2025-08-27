fn main() {
    let mut s = vec![
        String::from("Hello"),
        String::from("world"),
        String::from("!"),
    ];

    for i in s.iter() {
        println!("{}", i);
    }
    println!("{:?}", s.clone());

    for i in s.iter_mut() {
        i.push_str("++++");
    }
    println!("{:?}", s);

    for i in s.into_iter() {
        println!("{}", i);
    }
    // println!("{:?}",s); // 不可打印 s, into 会拿走所有权
}
