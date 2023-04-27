mod args;
use args::{Action, MarkAction, MarkCommand, NewCommand, ProjectArgs, SetCommand, ListCommand};
use clap::Parser;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
    process::{exit, Command},
};

fn save_project_config<P: AsRef<Path>>(config: &ProjectConfig, path: P) {
    let string = serde_json::to_string(config).unwrap_or_else(|err| {
        eprintln!("ERROR: Unable to parse project config: {}", err);
        exit(1)
    });
    fs::write(path, string).unwrap_or_else(|err| {
        eprintln!("ERROR: Unable to save project config: {}", err);
        exit(1)
    });
}

fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        eprintln!("ERROR: failed to copy {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}

fn create_template_project(config: Config, project_name: &str, template: String) {
    let template_path = config.templates_dir.join(format!("{}-template", template));
    if !template_path.exists() {
        eprintln!(
            "ERROR: Unable to find template {} at {}",
            template,
            config.templates_dir.display()
        );
        exit(1)
    }
    let path_to_project = config.workdir.join(project_name);
    copy(template_path, &path_to_project).unwrap_or_else(|err| {
        eprintln!(
            "ERROR: Unable to create new project {}: {}",
            project_name, err
        );
        exit(1)
    });
    println!("Created new project at {}", path_to_project.display());
}

fn handle_new_command(config: Config, mut project_config: ProjectConfig, command: NewCommand) {
    let path_to_project = config.workdir.join(&command.name);
    if path_to_project.exists() {
        eprintln!(
            "ERROR: A project named {} already exists at {}",
            command.name,
            config.workdir.display()
        );
        exit(1)
    };
    let template = command.template.unwrap_or("bin".to_string());
    if template != "lib" && template != "bin" {
        let workdir = config.workdir.clone();
        create_template_project(config, &command.name, template);
        project_config.insert(command.name.clone(), Project::new(&command.name));
        save_project_config(&project_config, workdir.join("projects.json"));
        return;
    }
    fs::create_dir(&path_to_project).unwrap_or_else(|err| {
        eprintln!(
            "ERROR: Unable to create new project {}: {}",
            command.name, err
        );
        exit(1)
    });
    fs::write(
        path_to_project.join("Cargo.toml"),
        format!(
            r#"[package]
name = "{}"
version = "0.1.0"
authors = ["Dilshad <dilshadplayingminecraft@outlook.com>"]
edition = "2021"

[dependencies]"#,
            command.name
        ),
    )
    .unwrap_or_else(|err| {
        eprintln!(
            "ERROR: Unable to create new project {}: {}",
            command.name, err
        );
        exit(1)
    });
    fs::write(path_to_project.join(".gitignore"), "/target").unwrap_or_else(|err| {
        eprintln!(
            "ERROR: Unable to create new project {}: {}",
            command.name, err
        );
        exit(1)
    });
    fs::create_dir(path_to_project.join("src")).unwrap_or_else(|err| {
        eprintln!(
            "ERROR: Unable to create new project {}: {}",
            command.name, err
        );
        exit(1)
    });
    if template == "bin" {
        fs::write(
            path_to_project.join("src/main.rs"),
            format!(
                r#"fn main() {{
    println!("Hello, from {}");
}}"#,
                command.name
            ),
        )
    } else {
        fs::write(
            path_to_project.join("src/lib.rs"),
            format!(
                r#"fn add(a: i32, b: i32) -> i32 {{
    a + b
}}

#[cfg(test)]
mod tests {{
    use super::*;
    #[test]
    fn test_add() {{
        let result = add(2, 2);
        assert_eq!(result, 4);
    }}
}}"#
            ),
        )
    }
    .unwrap_or_else(|err| {
        eprintln!(
            "ERROR: Unable to create new project {}: {}",
            command.name, err
        );
        exit(1)
    });
    println!("Created new project at {}", path_to_project.display());
    project_config.insert(command.name.clone(), Project::new(&command.name));
    save_project_config(&project_config, config.workdir.join("projects.json"));
    Command::new("git")
        .arg("init")
        .current_dir(path_to_project)
        .output()
        .unwrap_or_else(|err| {
            println!("WARNING: Unable to initialize git: {}", err);
            exit(0)
        });
}

fn handle_set_command(mut config: Config, command: SetCommand, config_path: PathBuf) {
    command.subcommand.apply(&mut config);
    let string = serde_json::to_string(&config).unwrap();
    fs::write(config_path, string).unwrap_or_else(|err| {
        eprintln!("ERROR: Unable to update config: {}", err);
        exit(1)
    });
}

fn handle_mark_command(config: Config, mut project_config: ProjectConfig, command: MarkCommand) {
    if !project_config.contains_key(&command.name) {
        let project_path = config.workdir.join(&command.name);
        if !project_path.exists() {
            eprintln!(
                "ERROR: No project named {} found at {}",
                command.name,
                config.workdir.display()
            );
            exit(1)
        };
        println!("Adding {} to project config ...", command.name.clone());
    } else {
        project_config.remove(&command.name);
    }
    project_config.insert(
        command.name.clone(),
        Project {
            name: command.name,
            completed: match command.mark_action {
                MarkAction::Completed => true,
                MarkAction::Incomplete => false,
            },
        },
    );
    save_project_config(&project_config, config.workdir.join("projects.json"));
}

fn is_rust_project<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().join("Cargo.toml").exists()
}

fn handle_analyze_command(config: Config, mut project_config: ProjectConfig) {
    let paths = fs::read_dir(&config.workdir).unwrap_or_else(|err| {
        eprintln!("ERROR: Unable to read working directory: {}", err);
        exit(1)
    });
    for path in paths {
        if let Err(err) = path {
            eprintln!("ERROR: Unable to read a file or directory: {}", err);
        } else {
            let path = path.unwrap().path();
            if is_rust_project(&path) {
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                if !project_config.contains_key(&name) {
                    project_config.insert(name.clone(), Project::new(name));
                }
            }
        }
    }
    for key in project_config.clone().keys() {
        let path = config.workdir.join(key);
        if !path.exists() || !is_rust_project(&path) {
            project_config.remove(key);
        }
    }
    save_project_config(&project_config, config.workdir.join("projects.json"));
}

fn handle_list_command(project_config: ProjectConfig, command: ListCommand) {
    for key in project_config.keys() {
        if let Some(ref filter) = command.filter {
            if !filter.passes(project_config.get(&key.clone()).unwrap()) {
                continue;
            }
        }
        println!("{}", key);
    }
}

fn reset_config(home: &PathBuf) -> std::io::Result<()> {
    let deafult_config = format!(
        r#"{{"workdir": "{}", "templates_dir": "{}"}}"#,
        home.join("Devs").display(),
        home.join("Templates").display()
    );
    fs::write(home.join(".rpmrc.json"), deafult_config)
}

fn reset_project_config(workdir: &PathBuf) -> std::io::Result<()> {
    let default_config = "{}";
    fs::write(workdir.join("projects.json"), default_config)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    workdir: PathBuf,
    templates_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    name: String,
    completed: bool,
}

type ProjectConfig = HashMap<String, Project>;

impl Project {
    fn new<T: AsRef<str>>(name: T) -> Self {
        Self {
            name: name.as_ref().to_string(),
            completed: false,
        }
    }
}

fn main() {
    let args = ProjectArgs::parse();
    let home = home_dir().unwrap_or_else(|| {
        eprintln!("ERROR: Unable to read var $HOME");
        exit(1)
    });
    let config_path = home.join(".rpmrc.json");
    if !config_path.exists() {
        reset_config(&home).unwrap_or_else(|err| {
            eprintln!(
                "ERROR: Unable initialize config at {}: {}",
                config_path.display(),
                err
            );
            exit(1)
        });
    };
    let config_file = File::open(&config_path).unwrap();
    let config: Config = serde_json::from_reader(config_file).unwrap_or_else(|err| {
        eprintln!(
            "ERROR: Unable to parse config file {}: {}",
            config_path.display(),
            err
        );
        exit(1)
    });
    if !config.workdir.exists() {
        fs::create_dir_all(&config.workdir).unwrap_or_else(|err| {
            eprintln!("ERROR: Unable to create working directory: {}", err);
            exit(1)
        });
    }
    let project_config_path = config.workdir.join("projects.json");
    if !project_config_path.exists() {
        reset_project_config(&config.workdir).unwrap_or_else(|err| {
            eprintln!(
                "ERROR: Unable to initialize project config at {}: {}",
                project_config_path.display(),
                err
            );
            exit(1)
        });
    };
    let project_config_file = File::open(&project_config_path).unwrap();
    let project_config: ProjectConfig = serde_json::from_reader(project_config_file)
        .unwrap_or_else(|err| {
            eprintln!(
                "ERROR: Unable to parse project config file {}: {}",
                project_config_path.display(),
                err
            );
            exit(1)
        });
    match args.action {
        Action::New(new_command) => handle_new_command(config, project_config, new_command),
        Action::Set(set_command) => handle_set_command(config, set_command, config_path),
        Action::Mark(mark_command) => handle_mark_command(config, project_config, mark_command),
        Action::Analyze => handle_analyze_command(config, project_config),
        Action::List(list_command) => handle_list_command(project_config, list_command)
    };
}
