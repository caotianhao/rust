fn main() {
    let double = |x| x * 2;
    println!("{}", double(2));

    let add = |a, b, c| a + b + c;
    println!("{}", add(1, 2, 3));

    let pr = || println!("Hello");
    pr();

    receive_closure(add);

    let do_res = do1(add, 5);
    println!("{:?}", do_res(5, 9));
}

fn receive_closure<T>(c: T)
where
    T: Fn(i32, i32, i32) -> i32,
{
    let res = c(2, 5, 9);
    println!("{:?}", res);
}

fn do1<F>(f: F, x: i32) -> impl Fn(i32, i32) -> i32
where
    F: Fn(i32, i32, i32) -> i32,
{
    move |a, b| f(x, a, b)
}
