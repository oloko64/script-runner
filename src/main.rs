mod runner;

use owo_colors::OwoColorize;

use runner::{Apps, HEADER_TEXT};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scripts_folder = std::env::args().nth(1).ok_or_else(|| {
        format!(
            "Usage: {} <scripts_folder>",
            std::env::args().next().unwrap()
        )
    })?;
    println!("v{}", env!("CARGO_PKG_VERSION"));
    println!("{}", HEADER_TEXT.magenta());

    let mut apps = Apps::new(&scripts_folder);
    apps.load_apps().map_err(|err| err.red().to_string())?;

    let selected = apps.prompt_user();
    println!();

    if let Some(selected) = selected {
        if let Err(err) = apps.execute(selected) {
            println!("{}", err.red());
        }
    } else {
        println!("{}", "No scripts selected. Exiting...".yellow());
    }

    Ok(())
}
