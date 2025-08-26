fn main() {
    let mut arr = [String::from("a"), String::from("b"), String::from("c")];
    println!("main arr: {:?}", arr);
    show_arr(&arr);
    arr.reverse();
    println!("arr_rev: {:?}", arr);
}

fn show_arr(arr: &[String]) {
    println!("show arr: {:?}", arr);
}
