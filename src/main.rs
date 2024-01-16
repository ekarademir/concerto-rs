mod parser;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let unparsed = fs::read_to_string("samples/one.cto")?;

    parser::parse(&unparsed)?;

    Ok(())
}
