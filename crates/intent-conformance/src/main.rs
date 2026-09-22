use intent_conformance::{render_conformance_result, run_conformance};
use std::process::ExitCode;

fn main() -> ExitCode {
    let rendered = match render_conformance_result(run_conformance()) {
        Ok(rendered) => rendered,
        Err(error) => {
            eprintln!("failed to serialize conformance result: {error}");
            return ExitCode::FAILURE;
        }
    };
    println!("{}", rendered.json());
    if rendered.passed() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
