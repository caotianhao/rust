#[derive(Debug)]
enum Weekday {
    Friday,
    Saturday,
    Sunday,
}

fn main() {
    println!(
        "{:?} {:?} {:?}",
        Weekday::Friday,
        Weekday::Saturday,
        Weekday::Sunday
    );

    println!("get_discount(11)--{:?}", get_discount(11));
    println!("get_discount(111)--{:?}", get_discount(111));

    println!(
        "judge_week(Weekday::Sunday)--{}",
        judge_week(Weekday::Sunday)
    )
}

fn get_discount(price: i32) -> Option<i32> {
    if price > 100 {
        return Some(price);
    }
    None
}

fn judge_week(w: Weekday) -> i32 {
    match w {
        Weekday::Friday => 5,
        Weekday::Saturday => 6,
        Weekday::Sunday => 7,
    }
}
