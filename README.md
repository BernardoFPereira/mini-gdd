# MiniGDD CLI
![demo](https://github.com/user-attachments/assets/6d4f4de6-ff39-478a-a0bf-4b1cc7bf81db)

Generate a minimal, presenterm-compatible, Game Design Document template on the current directory.
It is recommended to have `presenterm`.

## How to
Create the template file with the supplied name.

`minigdd spawn <game_name>`

If there's no name, the file will be generated with a default one.

Open file with your text editor of choice and edit it's contents.

Run presenterm with your new mini-gdd:

`presenterm <mini-gdd_file>`

Check `presenterm` [docs](https://mfontanini.github.io/presenterm/) if you want to customize the document further.

## Mini next steps
1. Add flags to customize the generated file a little better
  - [x] Author
  - [ ] Subtitle
  - [x] Raw (plain markdown)

# Installation
Right now the only way to install it is building it. You'll need `cargo` for that.

If you don't have `presenterm`,  use the `--raw` flag to generate a plain .md file. 
I want to eventually add config to set user-defined defaults.

Just clone the repo, `cd` into it and run `cargo install --path "."`.

This should install the CLI as `minigdd`.

Run `minigdd`, if the help text shows up, everything is working as it should.
