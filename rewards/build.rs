use std::error::Error;

use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    EmitBuilder::builder().all_git().emit()?;
    Ok(())
}
