# md-preview

> md-preview is a command-line tool written in Rust that renders Markdown files 
> directly in your terminal, applying syntax highlighting and formatting. It
> leverages `pulldown-cmark` for robust Markdown parsing and `termcolor` for 
> rich, colored output.

## Features

Syntax Highlighting: Renders various Markdown elements with distinct colors, 
including headings, bold text, italicized text, strikethrough, blockquotes, and 
code blocks.

- **Table Rendering**: Supports rendering Markdown tables with proper alignment and borders.
- **Customizable Symbol Rendering**: Option to display or hide Markdown symbols (e.g., ** for bold, ## for headings).
- **Content Centering**: Adjust left-side spacing to visually center the output in your terminal.

## Installation

### Prerequisites

- Rust programming language (ensure you have rustup installed).

### Build from Source

1. Clone the repository:

```Bash
    git clone https://github.com/sid12c/md-preview.git
    cd md-preview
```

2. Build the project:

```Bash
cargo build --release
```

3. Run the executable:

- The executable will be located in target/release/term_md_renderer. You can run it directly:

```Bash
./target/release/md-preview <file.md>
```

- Or, you can add it to your PATH for easier access:

```Bash
cargo install --path .
```

This will install the md-preview executable into your Cargo bin directory (usually ~/.cargo/bin), making it available globally.

## Usage

    Usage: md-preview [OPTIONS] <FILE>

### Arguments:

  <FILE>  Path to the Markdown file

### Options:
  -s, --symbol           Turn markdown symbol rendering on

  -c, --center \<CENTER>  Increment left side space to center [default: 0]

  -h, --help             Print help

  -V, --version          Print version

## Examples

1. Render a Markdown file with default settings:

```Bash
md-preview README.md
```

2. Render a Markdown file and show Markdown symbols:

```Bash
md-preview MyDocument.md --symbol
```

3. Render a Markdown file and add left-side spacing to center the content:

```Bash
md-preview Notes.md --center 10
```

4. Combine options:

```Bash
md-preview Report.md -s -c 5
```

## Supported Markdown Elements

This renderer aims to support a wide range of Markdown elements, including:

- Headings (#, ##, etc.)
- Paragraphs
- Bold text (**text**)
- Italicized text (*text*)
- Strikethrough (~~text~~)
- Blockquotes (> quote)
- Code blocks (fenced and indented)
- Inline code (code)
- Lists (unordered)
- Horizontal rules (---)

## Contact

Sid Chaudhary - contact.sid.chaudhary@gmail.com
