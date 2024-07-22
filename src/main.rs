use std::fs;
use tokio::process::Command;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, name = "action")]
enum CreateCli {
    #[command(about = "Generate bot invite given application ID and permissions string")]
    Generate {
        // id of the application to invite
        #[arg(long, short, help = "Bot ID")]
        id: String,
        // permission value to invite the bot with
        #[arg(long, short, help = "Permissions string describing the invite permissions for the bot")]
        permissions: String
    },
    #[command(alias = "create", about = "Create a new project directory")]
    New {
        #[arg(help = "Name of the project directory to create")]
        path: String,
        #[arg(long, short, default_value = "js", help = "Which language to use for the project (js or ts)")]
        language: String,
    },
    #[command(about = "Initialize current directory as a project")]
    Init {
        #[arg(long, short, default_value = "js", help = "Which language to use for the project (js or ts)")]
        language: String
    },
}

use CreateCli::*;

#[derive(Parser, Debug, Clone)]
struct Args {
}

#[tokio::main]
async fn main() {
    println!("Hello, {}!", std::env::var("USER").unwrap());
    let arguments = CreateCli::parse();
    match arguments.clone() {
        New { path, language } => {
            if let Ok(_) = fs::read_dir(path.clone()) {
                eprintln!(r#"Directory "{path}" already exists"#);
                std::process::exit(1);
            }

            if let Err(e) = fs::create_dir(path.clone()) {
                eprintln!(r#"Error creating "{path}": {e}"#);
                std::process::exit(1);
            }

            if let Err(e) = std::env::set_current_dir(path.clone()) {
                eprintln!(r#"Error entering directory "{path}": {e}"#);
                std::process::exit(1);
            }

            let extension =
                if matches!(language.to_ascii_lowercase().as_str(), "js" | "javascript") { "js" } else
                if matches!(language.to_ascii_lowercase().as_str(), "ts" | "typescript") { "ts" } else
                { unimplemented!("Unsupported language \"{language}\" ") };

            let js_template = include_str!("../templates/js/index.js");
            let ts_template = include_str!("../templates/ts/index.ts");

            if let Err(e) = fs::write(format!("index.{extension}"),
                match extension {
                    "ts" => ts_template,
                    "js" => js_template,
                    _ => unimplemented!()
                }
            ) {
                eprintln!("Unable to create index.{extension}: {e}");
            }
        },
        _ => ()
    };

    match arguments {
        New { language, .. } | Init { language, .. } => {
            let cmd = match language.to_ascii_lowercase().as_str() {
                "ts" | "typescript" => "bun",
                "js" | "javascript" => "npm",
                x => {
                    eprint!("Language {x} not currently supported");
                    std::process::exit(1);
                }
            };

            Command::new(cmd).args(["init", "-y", &format!("--init-author-name {}", std::env::var("USER").expect("Unable to read username"))]).kill_on_drop(true).spawn().unwrap().wait().await.unwrap();
            Command::new(cmd).args(["install", "discord.js", "dotenv"]).kill_on_drop(true).spawn().unwrap().wait().await.unwrap();
        },
        Generate { id, permissions } => println!("Invite: https://discord.com/oauth2/authorize?client_id={id}&scope=bot&permissions={permissions}")
    }
}
