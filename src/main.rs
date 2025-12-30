mod load_config;
mod types;

use clap::{Parser, Subcommand};
use std::{collections::HashMap, fs, path::Path, path::PathBuf, process};
use walkdir::WalkDir;
use xdg;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Cp {
        #[arg(required = true)]
        name: String,
        #[arg(short = 'o', long = "output")]
        output_path: Option<PathBuf>,
    },
    List {},
}

fn main() {
    let cli = Cli::parse();

    let xdg_dirs = xdg::BaseDirectories::with_prefix("tpfl");
    let config_path = match xdg_dirs.get_config_home() {
        Some(p) => p,
        None => {
            eprintln!("failed to get config dir");
            process::exit(1);
        }
    };

    if !config_path.is_dir() {
        match fs::create_dir_all(&config_path) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("couldn't create config dir");
                process::exit(1)
            }
        }
    }

    let mut configs: HashMap<String, types::Template> = HashMap::new();

    for entry in WalkDir::new(&config_path).follow_links(true) {
        let path = entry.unwrap().into_path();

        if path.extension() != Some("yaml".as_ref()) {
            continue;
        }

        let templates = match load_config::load_config(path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };

        for template in templates.templates {
            configs.insert(template.name.clone(), template);
        }
    }

    match cli.command {
        Commands::Cp { name, output_path } => {
            if configs.get(&name) == None {
                eprintln!("couldn't find {} in the config", &name);
                process::exit(1);
            }

            let template = configs.get(&name).unwrap();
            let new_path = Path::new(".").join(match output_path {
                Some(p) => p.to_str().unwrap().to_string(),
                None => template.file_name.clone(),
            });

            if template.file_type == "path" {
                match fs::copy(&template.path, &new_path) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                };
                eprintln!("copied {} to {}", template.path, new_path.to_str().unwrap());
            } else if template.file_type == "url" {
                eprintln!(
                    "started to download {} to {}",
                    &template.path,
                    &new_path.to_str().unwrap()
                );
                let mut response = match reqwest::blocking::get(template.path.clone()) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                };
                eprintln!(
                    "finished to download {} to {}",
                    &template.path,
                    &new_path.to_str().unwrap()
                );

                response = match response.error_for_status() {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                };

                let mut out = match fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(new_path)
                {
                    Ok(o) => o,
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                };

                match std::io::copy(&mut response, &mut out) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                };
            }
        }
        Commands::List {} => {
            for (name, template) in configs {
                println!(
                    "{} {} {} {}",
                    name, template.file_name, template.file_type, template.path
                );
            }
        }
    };
}
