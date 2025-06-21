use std::fs::File;
use std::io::Write;
use std::process;

#[derive(Debug, Clone)]
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

        let filtered_args: Vec<&String> = args.iter().filter(|a| !a.starts_with('-')).collect();
        let flag_args: Vec<&String> = args.iter().filter(|f| f.starts_with('-')).collect();

        if filtered_args.len() >= 3 {
            file_name = filtered_args[2].clone();
        }

        for arg in flag_args {
            match arg.to_lowercase().as_str() {
                "--author" => {
                    let flag_idx = args.iter().position(|a| a == arg).unwrap();
                    let author_name;

                    match args.get(flag_idx + 1) {
                        Some(name) => author_name = name.clone(),
                        None => return Err("Author flag used but no author found"),
                    }

                    flags.push(Flags::AUTHOR(author_name));
                }
                "--raw" => {
                    flags.push(Flags::RAW);
                }
                _ => {}
            }
        }

        Ok(Config {
            cmd,
            file_name,
            flags,
        })
    }
}

pub fn run(mut config: Config) -> Result<(), String> {
    match config.cmd.as_str() {
        "spawn" => {
            let mut author: String = "Author".to_string();
            let mut is_raw: bool = false;

            for flag in &config.flags {
                match flag {
                    Flags::AUTHOR(name) => {
                        if name != &config.cmd {
                            author = name.clone();

                            if author == config.file_name {
                                config.file_name = "mini".to_string()
                            }
                            println!("Author name: {author}");
                        } // else {
                          //     println!("Author flag used but no author specified! Using default.");
                          // }
                    }
                    Flags::RAW => {
                        if config.flags.len() < 2 {
                            println!("Using default author name.")
                        }
                        println!("Generating plain markdown");
                        is_raw = true;
                    }
                }
            }

            println!("File name: {}_gdd.md", config.file_name);

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
      - spawn <game_name> [flags]
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
