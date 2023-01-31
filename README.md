# Marky

Converts Markdown documents into themed HTML pages with support
for code syntax highlighting, LaTeX and Mermaid diagrams.

Supports PDF conversion via headless chromium.

<!--toc:start-->
- [Examples](#examples)
- [Install](#install)
- [Help](#help)
- [Build](#build)
<!--toc:end-->

## Examples

Convert `doc.md` to `doc.html`
```bash
marky doc.md
```

Convert to PDF
```bash
marky doc.md --pdf
```

Start a live file watcher (will recompile your document on each save)
```bash
marky doc.md --watch
```

Enable extensions
```bash
# Or use --all to enable all
marky doc.md --math --diagrams --highlight
```

Select and use a different theme with fzf
```bash
marky doc.md --theme $(marky --list-themes | fzf)
```

Pipe from stdout and open compiled file
```bash
cat doc.md | marky --out doc.html --open
```

> See `--help` for more info

## Install

Install using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```bash
cargo install marky
```

## Help

```
Usage: marky [OPTIONS] [PATH]

Arguments:
  [PATH]  Read input from file

Options:
      --completion <GENERATOR>  [possible values: bash, elvish, fish, powershell, zsh]
  -t, --theme <THEME>           Theme to use
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

## Build

```bash
cargo install --path .
```
