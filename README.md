# Marky

> Work in progress

Converts Markdown `.md` documents into good-looking HTML

## Usage

```
Markdown to HTML converter with beautiful themes

Usage: marky [OPTIONS] [PATH]

Arguments:
  [PATH]  Read input from file

Options:
  -t, --theme <THEME>        Theme to use
      --stdin                Read input from stdin
      --string <STRING>      Read input from string
  -l, --list-themes          List available themes
      --where-config         Print config path
  -o, --out <OUT>            Output file
      --stdout               Output to stdout
  -H, --syntax-highlighting  Enable syntax highligting
  -w, --watch                Enable file watcher
  -O, --open                 Open output file in the default app
  -h, --help                 Print help
  -V, --version              Print version
```

## Install

```bash
cargo install marky
```

## Build

```bash
cargo install --path .
```
