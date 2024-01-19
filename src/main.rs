use std::env;
use std::path::Path;

use anyhow::Result as Anyhow;
use chrono::Datelike;

use config::Config;
use prompt::Prompt;
use renderer::render_to_pdf;

mod config;
mod prompt;
mod renderer;

fn main() -> Anyhow<()> {
    let config_path = if cfg!(debug_assertions) {
        Path::new(&env::var("CARGO_MANIFEST_DIR")?).join("target/invoice.toml")
    } else {
        Path::new(&env::var("HOME")?).join(".config/invoice/invoice.toml")
    };

    let config = Config::open(&config_path)?;
    config.validate();

    let prompt = Prompt::ask(&config)?;

    let file_name = format!(
        "{}-{}-{}.pdf",
        prompt.invoice_number,
        prompt.service_date.month(),
        prompt.service_date.year(),
    );

    println!("\n\nRendering PDF...");
    render_to_pdf(&prompt, &file_name)?;
    println!("Done. File saved to ./{file_name}");

    Ok(())
}
