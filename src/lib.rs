use std::fs::File;
use std::io::Write;
use std::process;

#[derive(Debug)]
enum Flags {
    AUTHOR(String),
    RAW,
}

pub struct Config {
    cmd: String,
    file_name: String,
    flags: Vec<Flags>,
}
impl Config {
    pub fn build(args: &mut [String]) -> Result<Config, &'static str> {
        if args.len() <= 1 {
            print_help();
        }

        let cmd = args[1].clone();
        let mut file_name = String::from("mini");
        let mut flags = Vec::<Flags>::new();
        let filtered_args: Vec<&String>;

        filtered_args = args.iter().filter(|a| !a.starts_with('-')).collect();

        println!("{:?}", filtered_args);
        println!("{:?}", filtered_args.len());
        println!("{:?}", flags);

        let mut cloned_args: Vec<String> = Vec::new();
        args.clone_into(&mut cloned_args);

        if filtered_args.len() <= 2 {
            match filtered_args[1].as_str() {
                "spawn" => {
                    println!("Spawning GDD with default names.");
                    return Ok(Config {
                        cmd,
                        file_name,
                        flags,
                    });
                }
                _ => {}
            }
            return Err("Not enough arguments.");
        }

        file_name = filtered_args[2].clone();

        for arg in cloned_args {
            if arg.starts_with('-') {
                match arg.to_lowercase().as_str() {
                    "--author" => {
                        let author_name = filtered_args
                            .iter()
                            .last()
                            .unwrap_or(&&"Mini Me".to_string())
                            .to_string();
                        if author_name == file_name {
                            file_name = "mini".to_string();
                            println!("Using default file name.")
                        }
                        flags.push(Flags::AUTHOR(author_name));
                    }
                    "--raw" => {
                        flags.push(Flags::RAW);
                    }
                    _ => {}
                }
            }
        }

        Ok(Config {
            cmd,
            file_name,
            flags,
        })
    }
}

pub fn run(config: Config) -> Result<(), String> {
    match config.cmd.as_str() {
        "spawn" => {
            let mut author: String = "Mini Me".to_string();
            let mut is_raw: bool = false;
            println!("File name: {}", config.file_name);

            // Check flags
            for flag in config.flags {
                match flag {
                    Flags::AUTHOR(name) => {
                        if name != config.file_name {
                            author = name;
                        } else {
                            println!("Author flag used but no author specified! Using default.");
                        }
                    }
                    Flags::RAW => {
                        println!("Raw flag used -- Generating plain markdown");
                        is_raw = true;
                    }
                }
            }
            spawn_file(&config.file_name, author, is_raw)?;
        }
        _ => {
            return Err("No such command.".to_string());
        }
    }

    Ok(())
}

fn spawn_file(file_name: &str, author: String, is_raw: bool) -> Result<(), String> {
    let path = format!("{file_name}_gdd.md");

    match File::create(path) {
        Ok(mut file) => {
            let mut template = generate_header(file_name, "Really cool game", author);
            let mut body = generate_template(is_raw);
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
      Create a markdown file with a template mini gdd with the <game_name> as a file name, if supplied. Otherwise, the file will be automatically named mini_gdd.md.\n
      flags:
      - --raw
      Spawns a MiniGDD as a plain markdown file.\n
      - --author
      Specify author(s) to be included in the document. If absent MiniGDD will use the default author name."
    );
    process::exit(0);
}

fn generate_header(title: &str, subtitle: &str, author: String) -> Vec<u8> {
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

fn generate_template(is_raw: bool) -> Vec<u8> {
    match is_raw {
        true => r#"
Short Description
===
Here you describe the general take of the game.
A succint explanation of the whole thing.
Target audience, genre, style, etc.

Story
===
Basic story line of the game. Note important places or landmarks.

Basic Game Loops
===
How the game is played.

1. Describe Game Loop
2. ...

Minimum Viable Product
===
List of all features needed for a Minimum Viable Product.

Stretch Goals
===
If we get here with time and energy, do these.
"#
        .as_bytes()
        .to_vec(),
        false => r#"
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
        .to_vec(),
    }
}
