# Boosh
### An interactive shell for Linux

<p align="center">
    <img src="https://github.com/Echinoidea/boosh/blob/master/docs/github/images/boosh-logo.png" alt="boosh logo" width="256" height="256"/>
</p>

<p align="center">
    <img src="https://github.com/Echinoidea/boosh/blob/master/docs/github/images/screenshot.png" alt="screenshot" width="910" height="1040"/>
</p>

Work in progress but is currently functional as a basic interactive shell.

## Features
- Basic shell functionality
- Configurable prompt
- Color support
- cd jump history, cd ~, cd -

## Work in progress
- Pipes
- Config file and boosh path
- More built-in functions like ls, pwd, alias, etc
- Boosh scripting
- Raw input for C-c, C-a, C-e, C-l, etc

## Installation
- Requires cargo
- Add ~/.cargo/bin to PATH (or execute boosh bin manually from here)
- `cargo install --git https://github.com/Echinoidea/boosh.git`
- `boosh`
