# Marky

> Work in progress

Converts Markdown `.md` documents into good-looking HTML

## Usage

```
Usage: marky [OPTIONS] [PATH]

Arguments:
  [PATH]  Read input from file

Options:
  -t, --theme <THEME>        Theme to use
  -s, --stdin                Read input from stdin
      --string <STRING>      Read input from string
  -l, --list-themes          List available themes
      --where-config         Print config path
  -o, --out <OUT>            Output file
  -H, --syntax-highlighting  Enable syntax highlighting
  -w, --watch                Enable file watcher
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
