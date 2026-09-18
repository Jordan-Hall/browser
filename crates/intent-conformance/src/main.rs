use intent_conformance::run_conformance;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let report = run_conformance()?;
    println!("{}", report.to_pretty_json()?);
    Ok(())
}
