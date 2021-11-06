use color_eyre::{Report, Result};

// use factorio_optimizer::*;

fn main() -> Result<(), Report> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "FULL");
    }
    color_eyre::install()?;

    Ok(())
}
