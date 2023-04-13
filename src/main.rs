use clap::{Parser, Subcommand, Args};
use std::{process, fs, path::PathBuf};
use dirs;
use toml;

#[derive(Parser)]
#[command(name = "temp-master")]
#[command(author = "rikkaka <dsywh123@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "A simple tool to manage your templates", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
    Remove(RemoveArgs),
    List(ListArgs),
    Update(UpdateArgs),
    Clone(CloneArgs),
}

#[derive(Args)]
struct AddArgs {
    path: String,

    #[arg(short, long)]
    rename: Option<String>,
}

#[derive(Args)]
struct RemoveArgs {
    name: String,
}

#[derive(Args)]
struct ListArgs {}

#[derive(Args)]
struct UpdateArgs {
    path: String,
    name: String,

    #[arg(short, long)]
    rename: Option<String>,
}

#[derive(Args)]
struct CloneArgs {
    name: String,
}

struct Master {
    config_file: std::path::PathBuf,
    templates_dir: std::path::PathBuf,
    config: toml::Value,
}

impl Master {
    fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap();
        let root_dir = home_dir.join(".temp-master");
        if !root_dir.exists() {
            std::fs::create_dir(&root_dir).unwrap();
        }
        let config_file = root_dir.join("config.toml");
        if !config_file.exists() {
            std::fs::File::create(&config_file).unwrap();
        }
        let config = std::fs::read_to_string(&config_file).unwrap();
        let mut config: toml::Value = toml::from_str(&config).unwrap();
        let templates_dir = root_dir.join("templates");
        if !templates_dir.exists() {
            std::fs::create_dir(&templates_dir).unwrap();
        }
        Self {
            config_file,
            templates_dir,
            config,
        }
    }

    fn save(&self) {
        let config = toml::to_string(&self.config).unwrap();
        std::fs::write(&self.config_file, config).unwrap();
    }

    // Copy template to template
    fn add(&mut self, add_args: AddArgs) -> Result<String, Box<dyn std::error::Error>> {
        // check if path exists
        let path = std::path::PathBuf::from(&add_args.path);
        let path = std::fs::canonicalize(&path)?;
        if !path.exists() {
            eprintln!("Template path not exists");
            process::exit(1);
        }

        // move template in the working dir to self.templates_dir
        let name = add_args.rename.unwrap_or(path.file_name().unwrap().to_str().unwrap().to_string());
        let target_file = self.templates_dir.join(&name);
        if target_file.exists() {
            eprintln!("Template name already exists");
            process::exit(1);
        }
        match path.is_dir() {
            true => {
                let mut options = fs_extra::dir::CopyOptions::new();
                options.copy_inside = true;
                fs_extra::dir::copy(&path, &target_file, &options).unwrap();
            }
            false => {
                fs::copy(&path, &target_file).unwrap();
            }
        }
        Ok(String::from(format!("Template {} added", name)))
    }

    fn remove(&mut self, remove_args: RemoveArgs) -> Result<String, Box<dyn std::error::Error>> {
        let target_dir = self.templates_dir.join(&remove_args.name);
        if !target_dir.exists() {
            eprintln!("Template name not exists");
            process::exit(1);
        }
        match target_dir.is_dir() {
            true => {
                fs::remove_dir_all(&target_dir).unwrap();
            }
            false => {
                fs::remove_file(&target_dir).unwrap();
            }
        }
        Ok(String::from(format!("Template {} removed", &remove_args.name)))
    }

    // check templates_dir to get all templates
    fn list_temps(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut list_temps = ListTemps::new();
        list_temps.search_templates(&self.templates_dir);
        Ok(list_temps.get_list_for_print())
    }

    fn update(&mut self, update_args: UpdateArgs) -> Result<String, Box<dyn std::error::Error>> {
        self.remove(RemoveArgs {
            name: update_args.name.clone(),
        })?;
        self.add(AddArgs {
            path: update_args.path,
            rename: update_args.rename,
        })?;
        Ok(String::from(format!("Template {} updated", update_args.name)))
    }

    fn clone(&mut self, clone_args: CloneArgs) -> Result<String, Box<dyn std::error::Error>> {
        let template_dir = self.templates_dir.join(&clone_args.name);
        if !template_dir.exists() {
            eprintln!("Template name not exists");
            process::exit(1);
        }
        
        match template_dir.is_dir() {
            true => {
                let mut options = fs_extra::dir::CopyOptions::new();
                options.copy_inside = true;
                fs_extra::dir::copy(&template_dir, &std::env::current_dir()?, &options).unwrap();
            }
            false => {
                fs::copy(&template_dir, &std::env::current_dir()?).unwrap();
            }
        }
        Ok(String::from(format!("Template {} cloned", &clone_args.name)))
    }
}

struct ListTemps {
    templasts: Vec<String>,
}

impl ListTemps {
    fn new() -> Self {
        Self {
            templasts: Vec::new(),
        }
    }

    // - template1
    // - template2
    // ...
    fn get_list_for_print(&self) -> String {
        let mut list_str = String::new();
        for temp in &self.templasts {
            list_str.push_str(format!(" - {}\n", temp).as_str());
        }
        list_str.trim_end().to_string()
    }

    fn search_templates(&mut self, templates_dir: &PathBuf) {
        for entry in std::fs::read_dir(templates_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            self.templasts.push(path.file_name().unwrap().to_str().unwrap().to_string());
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut master = Master::new();
    let cli = Cli::parse();
    let result_str = match cli.command {
        Commands::Add(add_args) => master.add(add_args)?,
        Commands::Remove(remove_args) => master.remove(remove_args)?,
        Commands::List(_) => master.list_temps()?,
        Commands::Update(update_args) => master.update(update_args)?,
        Commands::Clone(clone_args) => master.clone(clone_args)?,
    };
    println!("{}", result_str);
    Ok(())
}