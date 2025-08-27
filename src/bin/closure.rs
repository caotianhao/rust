fn main() {
    let double = |x| x * 2;
    println!("{}", double(2));

    let add = |a, b, c| a + b + c;
    println!("{}", add(1, 2, 3));

    let pr = || println!("Hello");
    pr();
}
