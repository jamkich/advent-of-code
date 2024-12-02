mod days;
mod utils;
fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("give day number as argument");
    match day.as_str() {
        "1" => days::day01::run(),
        "2" => days::day02::run(),
        _ => eprintln!("Day {} not exist", day),
    }
}
