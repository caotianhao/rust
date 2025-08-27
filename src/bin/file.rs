use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() {
    // 打开
    let file = File::open("./src/test.txt");
    println!("{:?}", file);

    // 创建
    File::create("./src/test2.txt").expect("create failed");
    File::create("./src/test3.txt").expect("create failed");

    // 删除
    fs::remove_file("./src/test2.txt").expect("remove failed");
    fs::remove_file("./src/test3.txt").expect("remove failed");

    // 新建，写入
    let mut file2 = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./src/test2.txt")
        .expect("open failed");
    file2
        .write("Hello, world!".as_bytes())
        .expect("write failed");

    // 读取
    let mut file3 = File::open("./src/test.txt").expect("open failed");
    let mut contents = String::new();
    file3.read_to_string(&mut contents).expect("read failed");
    println!("{}", contents);
}
