use std::{fs, io::{self, Write}};
use clap::Parser;
// Import TagEnd alongside Tag
use pulldown_cmark::{Parser as MarkdownParser, Event, Tag, CodeBlockKind, TagEnd}; // Add TagEnd here

// 1. Argument Parsing with Clap
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Markdown file
    #[arg(short, long)]
    file: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // 2. File Reading
    let markdown_input = fs::read_to_string(&args.file)
        .expect(&format!("Could not read file: {}", args.file));

    // 3. Markdown Parsing
    let parser = MarkdownParser::new(&markdown_input);

    // 4. Terminal Rendering - This is the core logic
    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => print!(""), // Newline before paragraph, managed by text
                Tag::Heading { level, .. } => {
                    // Simple representation of headings
                    print!("{}", "#".repeat(level as usize));
                    print!(" ");
                },
                Tag::Strong => print!("**"), // Bold text
                Tag::Emphasis => print!("*"), // Italic text
                Tag::Strikethrough => print!("~~"), // Strikethrough
                Tag::BlockQuote(_) => print!("\n> "), // Blockquote
                Tag::CodeBlock(kind) => { // Use 'kind' instead of 'info' for clarity, matches variant name
                    // Simple code block representation
                    let lang_str = match kind {
                        CodeBlockKind::Fenced(lang) => lang.to_string(),
                        CodeBlockKind::Indented => String::new(), // No language for indented code blocks
                    };
                    println!("\n```{}", lang_str);
                },
                Tag::List(start_index) => {
                    // Unordered list
                    if start_index.is_none() { // Unordered list
                        print!("");
                    } else { // Ordered list
                         print!("");
                    }
                },
                Tag::Item => print!("- "), // List item
                Tag::Link { dest_url, .. } => print!("["),
                Tag::Image { dest_url, .. } => print!("!["),
                // Add more tags as needed
                _ => {} // Ignore other tags for now
            },
            // FIX: Match on TagEnd variants for Event::End
            Event::End(tag_end) => match tag_end { // Changed `tag` to `tag_end` for clarity
                TagEnd::Paragraph => println!("\n"), // End paragraph with a newline
                TagEnd::Heading { .. } => println!("\n"), // End heading with newline
                TagEnd::Strong => print!("**"),
                TagEnd::Emphasis => print!("*"),
                TagEnd::Strikethrough => print!("~~"),
                TagEnd::BlockQuote(_) => print!("\n"), // TagEnd::BlockQuote is a unit variant
                TagEnd::CodeBlock => println!("```\n"), // TagEnd::CodeBlock is a unit variant
                TagEnd::List(_) => println!("\n"), // TagEnd::List is a unit variant
                // FIX: Link and Image in TagEnd don't carry dest_url, just the end of the tag
                TagEnd::Link => print!(")"), // Just close the link parenthesis
                TagEnd::Image => print!(")"), // Just close the image parenthesis
                _ => {}
            },
            Event::Text(text) => print!("{}", text),
            Event::Code(code) => print!("`{}`", code), // For inline code, wrapped in backticks
            Event::SoftBreak => print!(" "), // Single newline within a paragraph
            Event::HardBreak => println!("\n"), // Force a new line
            Event::Rule => println!("\n---\n"), // Horizontal rule
            Event::FootnoteReference(name) => print!("[^{}]", name),
            // Add more event types as needed
            _ => {} // Ignore other events like HTML, FootnoteDefinition, etc.
        }
        io::stdout().flush()?; // Flush output to ensure it appears immediately
    }

    Ok(())
}
