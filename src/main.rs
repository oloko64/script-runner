mod runner;

use owo_colors::OwoColorize;

use crate::runner::{Apps, HEADER_TEXT};

fn main() -> Result<(), std::io::Error> {
    let scripts_folder = std::env::args().nth(1).unwrap_or_else(|| {
        println!("{}", "No scripts folder provided".red());
        println!(
            "\nUsage: {} <scripts-folder>",
            std::env::args().next().unwrap()
        );
        println!("\n{}", "Exiting...".yellow());
        std::process::exit(1);
    });
    println!("{}", HEADER_TEXT.magenta());

    let mut apps = Apps::new(&scripts_folder);
    apps.load_apps();

    let selected = apps.prompt_user();
    println!();

    if let Some(selected) = selected {
        apps.execute(selected)?;
    } else {
        println!("{}", "No apps selected. Exiting...".yellow());
    }

    Ok(())
}
