fn main() {
    let books = vec![
        String::from("The Advent"),
        String::from("The Advents"),
        String::from("The Adv"),
        String::from("The Aty"),
        String::from("The Adg"),
        String::from("The Ads"),
    ];

    for book in books.iter() {
        print!("{}", book);
        let a = match book.as_str() {
            "The Advent" => "bingo",
            _ => "no bingo",
        };
        if a != "bingo" {
            print!(" -- aa {}", a);
        }
        println!();
    }

    let mut slice = vec![1, 2, 3, 4, 5, 6, 7];
    for i in 0..slice.len() {
        slice[i] += 1;
    }
    println!("slice: {:?}", slice);

    let mut slice2: Vec<String> = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
        String::from("e"),
    ];
    for i in 0..slice2.len() {
        slice2[i].push_str("END");
    }
    println!("slice2: {:?}", slice2);

    let mut slice3: Vec<String> = vec![String::from("a"), String::from("b"), String::from("c")];
    slice3 = add_end(slice3);
    println!("slice3: {:?}", slice3);
}

fn add_end(mut s: Vec<String>) -> Vec<String> {
    for i in 0..s.len() {
        s[i].push_str("END");
    }
    s
}
