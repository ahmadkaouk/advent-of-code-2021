use aoc_2021::{day01, day02};
fn main() {
    let solutions = [day01::main, day02::main];
    for (day, solution) in solutions.iter().enumerate() {
        println!("------ Day {} ------", day + 1);
        solution()
    }
}
