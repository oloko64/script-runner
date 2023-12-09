use dialoguer::{theme::ColorfulTheme, MultiSelect};
use owo_colors::OwoColorize;
use std::thread;
use walkdir::WalkDir;

pub const HEADER_TEXT: &str = r"
                    (o_
        _o)         |  . :             (o-
    \\\__/      \\\_|  : :.        \\\_\
    <____).....<_____).:.::.......<_____).
";

pub struct Apps<'a> {
    apps: Vec<App>,
    scripts_folder: &'a str,
}

struct App {
    path: String,
    name: String,
}

impl App {
    fn new(path: String, name: String) -> App {
        App { path, name }
    }
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<'a> Apps<'a> {
    pub fn new(path: &'a str) -> Apps<'a> {
        Apps {
            apps: Vec::new(),
            scripts_folder: path,
        }
    }

    pub fn load_apps(&mut self) -> Result<(), &'static str> {
        let file_entries = WalkDir::new(self.scripts_folder)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| {
                e.ok().and_then(|entry| {
                    if entry.file_type().is_file()
                        && entry.path().extension()?.to_str()?.to_lowercase() == "sh"
                    {
                        Some(entry)
                    } else {
                        None
                    }
                })
            });

        for entry in file_entries {
            let path = entry
                .path()
                .to_str()
                .map(ToString::to_string)
                .ok_or("Failed to convert path to string")?;
            let name = entry
                .file_name()
                .to_str()
                .map(ToString::to_string)
                .ok_or("Failed to convert file name to string")?;

            self.apps.push(App::new(path, name));
        }

        Ok(())
    }

    pub fn prompt_user(&self) -> Option<Vec<usize>> {
        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick the scripts that you want to run | press `q` to exit")
            .items(&self.apps[..])
            .interact_opt()
            .unwrap();

        match selections {
            Some(selections) if selections.is_empty() => None,
            Some(selections) => Some(selections),
            None => None,
        }
    }

    pub fn execute(&self, index_to_execute: Vec<usize>) -> Result<(), std::io::Error> {
        thread::scope(|s| {
            let mut threads = Vec::new();
            for index in index_to_execute {
                let t = s.spawn(move || -> Result<(), std::io::Error> {
                    let App { path, name } = &self.apps[index];
                    println!(
                        "{} {}",
                        "Started execution of".green(),
                        name.magenta().bold()
                    );
                    let mut child = std::process::Command::new(path).spawn()?;
                    if child.wait()?.success() {
                        println!(
                            "{} {}",
                            "Successfully executed".green(),
                            name.magenta().bold()
                        );
                    } else {
                        println!("{} {}", "Failed to execute".red(), name.magenta().bold());
                    }

                    Ok(())
                });

                threads.push(t);
            }

            for thread in threads {
                thread.join().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to join thread")
                })??;
            }

            Ok(())
        })
    }
}
