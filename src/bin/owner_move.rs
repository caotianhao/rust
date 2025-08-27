#[derive(Debug)]
struct Student<'a> {
    name: String,
    age: u8,
    address: &'a str,
}

fn main() {
    let s1 = Student {
        name: String::from("Alice"),
        age: 10,
        address: "add",
    };
    println!("s1.address {}", s1.address);
    println!("s1 = {:?}", s1);

    show_student(&s1);
    println!("************************************************");

    println!("s1.get_student() {:?}", s1.get_student());
    println!("s1.get_name() {}", s1.get_name());
    println!("Student::get_age(&s1) {}", Student::get_age(&s1));
    println!("Student::get_address(&s1) {}", Student::get_address(&s1));
}

fn show_student(s: &Student) {
    println!("show student: {:?}", s);
}

impl<'a> Student<'a> {
    fn get_student(&self) -> &Student<'a> {
        &self
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(s: &Student) -> u8 {
        s.age
    }

    fn get_address(s: &Student<'a>) -> &'a str {
        s.address
    }
}
