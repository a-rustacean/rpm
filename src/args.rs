use crate::{
    Config,
    Project
};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ProjectArgs {
    /// action to perform
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Action {
    /// create a new project
    New(NewCommand),
    /// mark a project as completed or incomplete
    Mark(MarkCommand),
    /// set config of project manager
    Set(SetCommand),
    /// analyze the working directory and update projects.json
    Analyze,
    /// list projects
    List(ListCommand),
}

#[derive(Debug, Args, Clone)]
pub struct NewCommand {
    /// name of the new project
    pub name: String,
    /// template to use while creating new project
    #[arg(long, short)]
    pub template: Option<String>,
}

#[derive(Debug, Args, Clone)]
pub struct MarkCommand {
    /// name of the project
    pub name: String,
    /// what to mark
    #[command(subcommand)]
    pub mark_action: MarkAction,
}

#[derive(Debug, Args, Clone)]
pub struct SetCommand {
    /// property of config to change
    #[command(subcommand)]
    pub subcommand: SetSubCommand,
}

#[derive(Debug, Args, Clone)]
pub struct ListCommand {
    /// filter to apply
    #[command(subcommand)]
    pub filter: Option<ListFilter>
}

#[derive(Debug, Subcommand, Clone)]
pub enum MarkAction {
    /// the project is completed
    Completed,
    /// the project is incomplete
    Incomplete,
}

#[derive(Debug, Subcommand, Clone)]
pub enum SetSubCommand {
    /// working directory
    Workdir { dir: PathBuf },
    /// dirwctory where all the templates are stored
    TemplatesDir { dir: PathBuf },
}

impl SetSubCommand {
    pub fn apply(self, config: &mut Config) {
        match self {
            Self::Workdir { dir } => {
                config.workdir = dir;
            }
            Self::TemplatesDir { dir } => config.templates_dir = dir,
        };
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum ListFilter {
    /// list completed projects only
    Completed,
    /// list incomplete projects only
    Incomplete
}

impl ListFilter {
    pub fn passes(&self, project: &Project) -> bool {
        project.completed == match self {
            Self::Completed => true,
            Self::Incomplete => false
        }
    }
}
