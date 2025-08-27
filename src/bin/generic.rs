#[derive(Debug)]
struct Gen<Ta> {
    v: Ta,
}

#[derive(Debug)]
struct Book {
    name: String,
    author: String,
    price: u32,
}

trait BookTrait {
    fn get_name(&self) -> String;
    fn get_author(&self) -> String;
    fn get_price(&self) -> u32;
}

trait Cao {
    fn give(&self) -> String;
    fn give2() -> String;
}

impl BookTrait for Book {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_author(&self) -> String {
        self.author.clone()
    }

    fn get_price(&self) -> u32 {
        self.price
    }
}

impl Cao for Book {
    fn give(&self) -> String {
        String::from("Give")
    }

    fn give2() -> String {
        String::from("Give2")
    }
}

fn show_book<T: Cao>(t: T) {
    println!("{}", t.give());
    println!("{}", Book::give2());
}

fn main() {
    let a = Gen { v: 7 };
    let b = Gen { v: "7" };
    let c = Gen {
        v: String::from("7"),
    };

    println!("{:?},{:?},{:?}", a, b, c);
    println!("{}", a.v);
    println!("--------------------------------------------------");

    let book1 = Book {
        name: String::from("MyBook"),
        author: String::from("Me"),
        price: 10,
    };

    println!("{}", book1.get_name());
    println!("{}", book1.get_author());
    println!("{}", book1.get_price());

    show_book(book1);
}
