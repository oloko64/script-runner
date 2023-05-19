use dialoguer::{theme::ColorfulTheme, MultiSelect};
use owo_colors::OwoColorize;
use std::sync::Arc;
use walkdir::WalkDir;

pub struct Apps {
    apps: Vec<App>,
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

impl Apps {
    pub fn new() -> Apps {
        Apps { apps: Vec::new() }
    }

    pub fn load_apps(&mut self) -> Option<()> {
        let file_entries = WalkDir::new("./apps")
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
            let path = entry.path().to_str()?.to_string();
            let name = entry.file_name().to_str()?.to_string();

            self.apps.push(App::new(path, name));
        }

        Some(())
    }

    pub fn prompt_user(&self) -> Option<Vec<usize>> {
        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick the scripts that you want to run")
            .items(&self.apps[..])
            .interact()
            .unwrap();

        if selections.is_empty() {
            None
        } else {
            Some(selections)
        }
    }

    pub fn execute(self, index_to_execute: Vec<usize>) -> Result<(), std::io::Error> {
        let apps = Arc::new(self.apps);
        let mut threads = Vec::new();

        for index in index_to_execute {
            let apps = Arc::clone(&apps);

            let thread = std::thread::spawn(move || -> Result<(), std::io::Error> {
                let app = &apps[index];
                let App { path, name } = app;
                println!("{} {}", "Started execution of".green(), name.magenta());
                let mut child = std::process::Command::new(path).spawn()?;
                if child.wait()?.success() {
                    println!("{} {}", "Successfully executed".green(), name.magenta());
                } else {
                    println!("{} {}", "Failed to execute".red(), name.magenta());
                }

                Ok(())
            });

            threads.push(thread);
        }

        for thread in threads {
            thread.join().unwrap()?;
        }

        Ok(())
    }
}
