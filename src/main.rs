mod runner;

use owo_colors::OwoColorize;

use crate::runner::Apps;

const HEADER_TEXT: &str = r#"
                    (o_
        _o)         |  . :             (o-
    \\\__/      \\\_|  : :.        \\\_\
    <____).....<_____).:.::.......<_____).
"#;

fn main() -> Result<(), std::io::Error> {
    println!("{}", HEADER_TEXT.magenta());

    let mut apps = Apps::new();
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
