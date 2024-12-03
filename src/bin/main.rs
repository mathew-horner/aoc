use aoc::solutions;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        println!("Usage: cargo run -- <year> <day> <part>");
        std::process::exit(1);
    }
    let year = format!("year_{}", args[1]);
    let day = format!("day{}", args[2]);
    let part = args[3].trim().parse::<u8>().unwrap();
    solutions::run(&year, &day, part);
}
