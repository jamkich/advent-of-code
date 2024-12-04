mod days;
mod utils;
fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("give day number as argument");
    match day.as_str() {
        "1" => days::day01::run(),
        "2" => days::day02::run(),
        "3" => days::day03::run(),
        "4" => days::day04::run(),
        // "5" => days::day05::run(),
        // "6" => days::day06::run(),
        // "7" => days::day07::run(),
        // "8" => days::day08::run(),
        // "9" => days::day9::run(),
        // "10" => days::day10::run(),
        _ => eprintln!("Day {} not exist", day),
    }
}
