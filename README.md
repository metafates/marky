# Marky

Markdown Magician 🧙

**Features**

- Hot reload previewing 🔥
- Conversion to **HTML** / **PDF**  🏭
- Themes! ✨
- Extensions - Math, diagrams, syntax-highlighting 🧩

> **Note** When converting to PDF it will automatically download a suitable
> [headless chrome](https://chromium.googlesource.com/chromium/src/+/lkgr/headless/README.md) binary if one is not present on your system.
> Everything is automated!

<!--toc:start-->
- [Examples](#examples)
- [Install](#install)
- [Help](#help)
- [Build](#build)
- [Screenshots](#screenshots)
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

Start a local preview server with hot-reload 

```bash
marky doc.md --live
```

Enable extensions

```bash
# Or use --all to enable all
marky doc.md --math --diagrams --highlight
```

Include local images as base64 encoded (beta)

```bash
# default: none, possible values: none, local, remote, all
marky doc.md --include-images local
```

Select and use a different theme with fzf

```bash
marky doc.md --theme $(marky --themes | fzf)
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
      --themes                  List available themes
      --where-config            Print config path
  -o, --out <OUT>               Output file
      --stdout                  Output to stdout
  -H, --highlight               Enable syntax highligting with highlight.js
  -M, --math                    Enable math rendering with KaTeX
  -D, --diagrams                Enable UML diagrams rendering with Mermaid
  -A, --all                     Enable all extra renderers
  -w, --watch                   Recompile file on save
  -l, --live                    Live preview in the browser
      --port <PORT>             Port of the live server [default: 8080]
  -O, --open                    Open output file in the default app
  -p, --pdf                     Saves document as PDF, will auto-download headless-chrome
  -h, --help                    Print help
  -V, --version                 Print version
```

## Build

```bash
git clone https://github.com/metafates/marky.git
cd marky
cargo install --path .
```

## Screenshots

Some examples...

```bash
marky README.md --theme sakura # default theme
```
![sakura](https://user-images.githubusercontent.com/62389790/216391306-ecd73229-6342-4a79-8f7f-5f632a231a6f.png)

```bash
marky README.md --theme air
```
![air](https://user-images.githubusercontent.com/62389790/216391415-46ca090a-801d-423e-a523-dc3e59ed1f77.png)

```bash
marky README.md --theme retro
```
![retro](https://user-images.githubusercontent.com/62389790/216391465-ddfff1ad-3cd6-43b8-a193-fc9c664ec018.png)


See `marky --themes` to show all available themes.

You can also your own themes, but it's not documented yet... 😴
