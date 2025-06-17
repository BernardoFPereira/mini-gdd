use std::fs::File;
use std::io::Write;
use std::process;

pub struct Config {
    cmd: String,
    file_name: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 1 {
            print_help();
        }

        let cmd = args[1].clone();
        let file_name: String;

        if args.len() <= 2 {
            match args[1].as_str() {
                "spawn" => {
                    file_name = "mini_gdd".to_string();
                    return Ok(Config { cmd, file_name });
                }
                _ => {}
            }
            return Err("Not enough arguments");
        }

        file_name = args[2].clone();

        Ok(Config { cmd, file_name })
    }
}

pub fn run(config: Config) -> Result<(), String> {
    println!("Comand to execute: {:?}", config.cmd);
    println!("file_path provided: {:?}", config.file_name);

    println!("{:?}", config.cmd);
    match config.cmd.as_str() {
        "spawn" => {
            println!("Spawning gdd at {:?}", config.file_name);
            spawn_file(&config.file_name)?;
        }
        // "clean" => {
        //     println!("GDD Restored");
        // }
        // "delete" => {
        //     println!("Deleting gdd at {:?}", config.path);
        // }
        _ => {
            return Err("No such command.".to_string());
        }
    }

    Ok(())
}

fn spawn_file(file_name: &str) -> Result<(), String> {
    let path = format!("{file_name}.md");

    match File::create(path) {
        Ok(mut file) => {
            let mut template = generate_header(file_name, "Really cool game", "Author");
            let mut body = generate_template();
            template.append(&mut body);

            if let Ok(_) = file.write_all(&template) {
                println!("Mini-GDD spawned.");
            };
        }
        Err(_) => {
            return Err("Could not spawn file!".to_string());
        }
    }

    Ok(())
}

fn print_help() {
    println!(
    "MiniGDD
      commands:
      - spawn <game_name>
      Create a markdown file with a template mini gdd with the <game_name> as a file name, if supplied. Otherwise, the file will be automatically named mini_gdd.md\n"
    );
    process::exit(0);
}

fn generate_header(title: &str, subtitle: &str, author: &str) -> Vec<u8> {
    format!(
        r#"---
title: {title}
sub_title: {subtitle}
author: {author}
---"#
    )
    .as_bytes()
    .to_vec()
}

fn generate_template() -> Vec<u8> {
    r#"
Short Description
===
Here you describe the general take of the game.
A succint explanation of the whole thing.
Target audience, genre, style, etc.

<!-- end_slide -->

Story
===
Basic story line of the game. Note important places or landmarks.

<!-- end_slide -->

Basic Game Loops
===
How the game is played.

1. Describe Game Loop
2. ...

<!-- end_slide -->

Minimum Viable Product
===
List of all features needed for a Minimum Viable Product.

<!-- end_slide -->

Stretch Goals
===
If we get here with time and energy, do these.
"#
    .as_bytes()
    .to_vec()
}
