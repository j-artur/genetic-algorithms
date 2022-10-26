use termion::{
    color::{Blue, Cyan, Fg, Green, Magenta, Reset, White},
    style::{self, Bold},
};

use crate::{config::function, fittest_pair, types::Pair};

pub fn print_info(n: u64, population: &Vec<Pair>) {
    println!("{}Generation: {}{}{}", Fg(Blue), Bold, n, style::Reset);
    println!(
        "{}[{}]{}",
        Fg(Cyan),
        population
            .iter()
            .map(|p| format!("( {} , {} )", p.x().value(), p.y().value()))
            .collect::<Vec<String>>()
            .join(", "),
        Fg(Reset)
    );
    println!(
        "{}[{}]{}",
        Fg(Cyan),
        population
            .iter()
            .map(|p| format!("({},{})", p.x().bits(), p.y().bits()))
            .collect::<Vec<String>>()
            .join(", "),
        Fg(Reset)
    );
    let fittest_pair = fittest_pair(population);
    println!(
        "{}Fittest pair: {}{}{}",
        Fg(Green),
        Bold,
        fittest_pair,
        style::Reset,
    );
    println!(
        "{}f{} = {}{}{}",
        Fg(Magenta),
        fittest_pair,
        Bold,
        function(fittest_pair.x(), fittest_pair.y()),
        style::Reset
    );
    println!("{}{}{}", Fg(White), "-".repeat(110), Fg(Reset));
}
