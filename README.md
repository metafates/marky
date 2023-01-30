# Marky

Converts Markdown documents into themed HTML pages with support
for code syntax highlighting, LaTeX and Mermaid diagrams.

Supports PDF conversion via headless chromium.

## Screenshots

> To be added...

## Usage

```
Markdown to HTML converter with beautiful themes

Usage: marky [OPTIONS] [PATH]

Arguments:
  [PATH]  Read input from file

Options:
      --completion <GENERATOR>  [possible values: bash, elvish, fish, powershell, zsh]
  -t, --theme <THEME>           Theme to use
      --stdin                   Read input from stdin
      --string <STRING>         Read input from string
  -l, --list-themes             List available themes
      --where-config            Print config path
  -o, --out <OUT>               Output file
      --stdout                  Output to stdout
  -H, --highlight               Enable syntax highligting with highlight.js
  -M, --math                    Enable math rendering with KaTeX
  -D, --diagrams                Enable UML diagrams rendering with Mermaid
  -A, --all                     Enable all extra renderers
  -w, --watch                   Enable file watcher
  -O, --open                    Open output file in the default app
  -p, --pdf                     Saves document as PDF using headless chrome
  -h, --help                    Print help
  -V, --version                 Print version
```

## Install

```bash
cargo install marky
```

## Build

```bash
cargo install --path .
```
