use termion::{
    color::{Blue, Cyan, Fg, Green, Magenta, Reset, White},
    style::{self, Bold},
};

use crate::{config::function, fittest_pair, types::Pair};

pub fn print_pairs(pairs: &Vec<Pair>) {
    println!(
        "{}[{}]{}",
        Fg(Cyan),
        pairs
            .iter()
            .map(|p| format!("( {} , {} )", p.x().value(), p.y().value()))
            .collect::<Vec<String>>()
            .join(", "),
        Fg(Reset)
    );
    println!(
        "{}[{}]{}",
        Fg(Cyan),
        pairs
            .iter()
            .map(|p| format!("({},{})", p.x().bits(), p.y().bits()))
            .collect::<Vec<String>>()
            .join(", "),
        Fg(Reset)
    );
}

pub fn print_info(n: u64, parents: &Vec<Pair>, population: &Vec<Pair>) {
    println!("{}Generation: {}{}{}", Fg(Blue), Bold, n, style::Reset);
    if !parents.is_empty() {
        println!("{}Parents from old generation:{}", Fg(Green), style::Reset);
        print_pairs(parents);
        println!("{}New population:{}", Fg(Green), style::Reset);
    } else {
        println!("{}Population:{}", Fg(Green), style::Reset);
    }
    print_pairs(population);
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
