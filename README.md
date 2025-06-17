# MiniGDD CLI
Generate a minimal, presenterm-compatible, Game Design Document template on the current directory.
It is recommended to have `presenterm`.

## How to
Create the template file with the supplied name.

`minigdd spawn <game_name>`

If there's no name, the file will be generated with a default one.

Open file with your text editor of choice and edit it's contents.

Run presenterm with your new mini-gdd:

`presenterm <mini-gdd file>`

Check `presenterm` [docs](https://mfontanini.github.io/presenterm/) if you want to customize the document further.

## Mini next steps
1. Add flags to customize the generated file a little better
  - [ ] Author
  - [ ] Subtitle
  - [ ] "Raw" (Not presenterm-compatible)

# Installation
Right now the only way to install it is building it. You'll need `cargo` for that.

For now, I recommend installing `presenterm` if you don't have it, otherwise the formatting would be a hindrance more than anything. Eventually I'll add a --raw flag to the CLI to generate a simpler (not "presenterm-ready") .md file.

Just clone the repo, `cd` into it and run `cargo install --path "."`.

This should install the CLI as `minigdd`.

Run `minigdd`, if the help text shows up, everything is working as it should.
