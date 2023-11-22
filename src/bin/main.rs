use aoc::solutions;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run -- <year> <day>");
        std::process::exit(1);
    }
    solutions::run(&args[1], &args[2]);
}
