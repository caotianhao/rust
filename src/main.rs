mod enum_exe;

fn main() {
    println!("a {}", add(1, 6));
    println!("Hello, world!");

    let sum = add(1, 660);
    println!("aa {}", sum);

    exe1();
    exe2();
    exe3();
    exe4();

    enum_exe::del();
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn exe1() {
    let a;
    a = 2;
    println!("{}", a)
}

fn exe2() {
    let a = 'a';
    let b = "eh";
    let c = (1, 2.3, "aa");

    let d = [1, 2, 3, 4, 5];

    println!("{}", a);
    println!("{}", b);
    println!("{:?}", c);
    println!("{:?}", d)
}

fn exe3() {
    let s1: String = String::from("hello");
    let s2 = "hello";

    println!("{}", s1 == s2); // true

    let s3 = s1.clone();
    println!("{}", s3);
    println!("{}", s1)
}

fn exe4() {
    let mut s = enum_exe::Student {
        name: String::from(""),
        age: 0,
    };

    s.age = 22;
    s.name = String::from("maa");

    println!("{:?}", s);
}
