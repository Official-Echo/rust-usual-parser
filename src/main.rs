use anyhow::Result;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

fn main() -> Result<()> {
    let inputs = vec![
        "123",
        "45.67",
        "Arch is the best!",
        "-273.15",
        "Goodbye World!",
        "-42",
        "0.99",
        "Pest",
    ];

    for input in inputs {
        if let Ok(parsed_data) = Grammar::parse(Rule::field, input) {
            println!("Parsed '{}'.", input);
            dbg!(parsed_data);
            println!("");
        } else if let Err(e) = Grammar::parse(Rule::field, input) {
            println!("Threw error for '{}'.", input);
            dbg!(e);
            println!("");
        }
    }

    Ok(())
}
