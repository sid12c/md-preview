use std::{fs, io::{self, Write}};
use clap::Parser;
use pulldown_cmark::{Parser as MarkdownParser, Event, Tag, CodeBlockKind, TagEnd, Options, Alignment};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// 1. Argument Parsing with Clap
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Markdown file
    #[arg(value_name = "FILE")]
    file: String,

    /// Turn markdown symbol rendering on
    #[arg(short, long)]
    symbol: bool,

    /// Increment left side space to center
    #[arg(short, long, default_value_t = 0)]
    center: usize,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // 2. File Reading
    let markdown_input = fs::read_to_string(&args.file)
        .expect(&format!("Could not read file: {}", args.file));

    // 3. Markdown Parsing
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    let parser = MarkdownParser::new_ext(&markdown_input, options);

    // Initialize a StandardStream for stdout with automatic color detection
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    // --- ColorSpec Definitions (remain the same) ---
    let mut heading_color = ColorSpec::new();
    heading_color.set_fg(Some(Color::Blue)).set_bold(true);

    let mut strong_color = ColorSpec::new();
    strong_color.set_fg(Some(Color::Yellow));

    let mut emphasis_color = ColorSpec::new();
    emphasis_color.set_fg(Some(Color::Green));

    let mut strikethrough_color = ColorSpec::new();
    strikethrough_color.set_fg(Some(Color::Red));

    let mut blockquote_color = ColorSpec::new();
    blockquote_color.set_fg(Some(Color::Magenta));

    let mut code_color = ColorSpec::new();
    code_color.set_fg(Some(Color::Cyan));

    let mut fence_color = ColorSpec::new();
    fence_color.set_fg(Some(Color::Ansi256(8))); // Dark gray / Bright Black

    let mut rule_color = ColorSpec::new();
    rule_color.set_fg(Some(Color::Ansi256(8))); // Dark gray / Bright Black

    let mut table_header_color = ColorSpec::new();
    table_header_color.set_fg(Some(Color::Ansi256(4))).set_bold(true); // White bold for table headers

    let mut table_border_color = ColorSpec::new();
    table_border_color.set_fg(Some(Color::Ansi256(4)));
    // --- End ColorSpec Definitions ---

    let mut text_level = 0;
    let mut in_code_block = false;
    let mut in_block_quote = false;
    let mut first_row = false;
    let mut in_code = false;
    let mut no_tab = false;
    let mut in_list = false;
    let mut in_table = false;
    let mut table_alignments: Vec<Alignment> = Vec::new();
    let mut current_row_cells: Vec<String> = Vec::new();
    let mut is_header_row = false;
    let mut column_widths: Vec<usize> = Vec::new();

    // 4. Terminal Rendering - This is the core logic with termcolor
    for event in parser {
        match event {
            Event::Start(tag) => {
                stdout.reset()?;
                match tag {
                    Tag::Paragraph => (),
                    Tag::Heading { level, .. } => {
                        no_tab = true;
                        text_level = level as usize - 1 + args.center;
                        writeln!(stdout)?;
                        let hash_prefix = "#".repeat(text_level + 1);
                        let tab_prefix = "\t".repeat(text_level);
                        stdout.set_color(&heading_color)?;
                        // write!(stdout, "{}", tab_prefix)?;
                        if args.symbol {
                            write!(stdout, "{}{} ", tab_prefix, hash_prefix)?;
                        } else
                        {
                            write!(stdout, "{}", tab_prefix)?;
                        }
                    },
                    Tag::Strong => {
                        no_tab = true;
                        stdout.set_color(&strong_color)?;
                        if args.symbol {
                            write!(stdout, "**")?;
                        }
                    },
                    Tag::Emphasis => {
                        no_tab = true;
                        stdout.set_color(&emphasis_color)?;
                        if args.symbol {
                            write!(stdout, "*")?;
                        }
                    },
                    Tag::Strikethrough => {
                        no_tab = true;
                        stdout.set_color(&strikethrough_color)?;
                        if args.symbol {
                            write!(stdout, "~~")?;
                        }
                    },
                    Tag::BlockQuote(_) => {
                        in_block_quote = true;
                        first_row = true;
                        // no_tab = true;
                        let tab_prefix = "\t".repeat(text_level);
                        stdout.set_color(&blockquote_color)?;
                        write!(stdout, "\n{}> ", tab_prefix)?;
                    },
                    Tag::CodeBlock(kind) => {
                        in_code_block = true;
                        let lang_str = match kind {
                            CodeBlockKind::Fenced(lang) => lang.to_string(),
                            CodeBlockKind::Indented => String::new(),
                        };
                        if args.symbol {
                            // writeln!(stdout)?; // Newline before code block
                        
                            let tab_prefix = "\t".repeat(text_level);
                            write!(stdout, "{}", tab_prefix)?;
                            stdout.set_color(&fence_color)?; // Set fence color
                        
                            write!(stdout, "```")?;
                            stdout.set_color(&code_color)?; // Set code color for language
                            write!(stdout, "{}", lang_str)?;
                        writeln!(stdout)?; // Newline after language info
                        } else {
                            stdout.set_color(&code_color)?; // Set code color for language
                        }
                    },
                    Tag::List(_) => {},
                    Tag::Item => {
                        in_list = true;
                        let tab_prefix = "\t".repeat(text_level);
                        write!(stdout, "{}", tab_prefix)?;
                        write!(stdout, "- ")?;
                    },
                    Tag::Link { .. } => write!(stdout, "[")?,
                    Tag::Image { .. } => write!(stdout, "![")?,
                    Tag::Table(alignments) => {
                        in_table = true;
                        table_alignments = alignments;
                        column_widths.clear(); // Clear previous table's widths
                        current_row_cells.clear(); // Clear any lingering cell data
                        writeln!(stdout)?; // Newline before table
                    },
                    Tag::TableHead => {
                        is_header_row = true;
                    },
                    Tag::TableRow => {
                        current_row_cells.clear(); // Start a new row, clear previous cells
                    },
                    Tag::TableCell => {
                    },
                    _ => {}
                }
            },
            Event::End(tag_end) => {
                match tag_end {
                    TagEnd::Paragraph => writeln!(stdout)?,
                    TagEnd::Heading { .. } => {
                        writeln!(stdout)?; // Newline for the end of the heading
                        stdout.reset()?; // Reset color after the heading
                        no_tab = false;
                    },
                    TagEnd::Strong => {
                        if args.symbol {
                            write!(stdout, "**")?;
                        }
                        stdout.reset()?;
                    },
                    TagEnd::Emphasis => {
                        if args.symbol {
                            write!(stdout, "*")?;
                        }
                        stdout.reset()?;
                    },
                    TagEnd::Strikethrough => {
                        if args.symbol {
                            write!(stdout, "~~")?;
                        }
                        stdout.reset()?;
                    },
                    TagEnd::BlockQuote(_) => {
                        writeln!(stdout)?;
                        in_block_quote = false;
                        first_row = false;
                    },  
                    TagEnd::CodeBlock => {
                        let tab_prefix = "\t".repeat(text_level);
                        write!(stdout, "{}", tab_prefix)?;
                        stdout.set_color(&fence_color)?;
                        if args.symbol {
                            write!(stdout, "```")?;
                        }
                        writeln!(stdout)?;
                        in_code_block = false;
                    },
                    TagEnd::List(_) => writeln!(stdout)?,
                    TagEnd::Item => {
                        writeln!(stdout)?;
                        in_list = false;
                    },
                    TagEnd::Link => write!(stdout, ")")?,
                    TagEnd::Image => write!(stdout, ")")?,
                    TagEnd::TableCell => {
                        // A cell has ended, add its accumulated content to current_row_cells
                        // We need to capture the text for the cell. This means `Event::Text`
                        // should append to `current_row_cells` if `in_table` is true.
                        // For now, let's assume `Event::Text` builds a string directly into `current_row_cells`.
                        // This will require a minor refactor to Event::Text
                    },
                    TagEnd::TableRow => {
                        // A row has ended. Now we can format and print it.
                        // First, calculate column widths if this is the header row
                        for (i, cell_content) in current_row_cells.iter().enumerate() {
                                if i >= column_widths.len() {
                                    column_widths.push(0);
                                }
                                column_widths[i] = column_widths[i].max(cell_content.len());
                            }
                        // if is_header_row {
                        //     for (i, cell_content) in current_row_cells.iter().enumerate() {
                        //         if i >= column_widths.len() {
                        //             column_widths.push(0);
                        //         }
                        //         column_widths[i] = column_widths[i].max(cell_content.len());
                        //     }
                        // } else {
                        //     // For body rows, ensure column_widths is populated if it's the first data row
                        //     // or if header was skipped. Better to calculate overall width after collecting all data
                        //     // or just ensure column_widths is based on header + max content.
                        //     // For simplicity, let's assume header defines widths for now.
                        //     for (i, cell_content) in current_row_cells.iter().enumerate() {
                        //         if i >= column_widths.len() {
                        //              // This can happen if the table has no header, or varying cell counts.
                        //              // For robustness, expand `column_widths` if necessary.
                        //              column_widths.push(0);
                        //         }
                        //         column_widths[i] = column_widths[i].max(cell_content.len());
                        //     }
                        // }

                        // Print the row
                        stdout.set_color(&table_border_color)?;
                        write!(stdout, "|")?;
                        stdout.reset()?; // Reset color after the border

                        for (i, cell_content) in current_row_cells.iter().enumerate() {
                            let width = *column_widths.get(i).unwrap_or(&0);
                            let formatted_cell = match table_alignments.get(i) {
                                Some(Alignment::Left) => format!("{:<width$}", cell_content),
                                Some(Alignment::Center) => format!("{:^width$}", cell_content),
                                Some(Alignment::Right) => format!("{:>width$}", cell_content),
                                _ => format!("{:<width$}", cell_content), // Default to left
                            };
                            if is_header_row {
                                stdout.set_color(&table_header_color)?;
                                write!(stdout, "{}", formatted_cell)?;
                            } else {
                                write!(stdout, "{}", formatted_cell)?;
                            }
                            stdout.set_color(&table_border_color)?;
                            write!(stdout, "|")?;
                            stdout.reset()?;
                        }
                        writeln!(stdout)?;

                        if is_header_row {
                            // Print the header separator line
                            stdout.set_color(&table_border_color)?;
                            write!(stdout, "|")?;
                            for (i, &width) in column_widths.iter().enumerate() {
                                let separator = match table_alignments.get(i) {
                                    Some(Alignment::Left) => format!(":{:-<width$}", ""),
                                    Some(Alignment::Center) => format!(":{:-^width$}", ""),
                                    Some(Alignment::Right) => format!("{:-<width$}:", ""),
                                    _ => format!("{:-<width$}", ""), // Default
                                };
                                write!(stdout, "{}", separator)?;
                                write!(stdout, "|")?;
                            }
                            writeln!(stdout)?;
                            stdout.reset()?;
                            is_header_row = false; // Reset for subsequent rows
                        }
                    },
                    TagEnd::TableHead => {
                        // The header has ended, table body will follow
                    },
                    TagEnd::Table => {
                        in_table = false;
                        table_alignments.clear();
                        column_widths.clear();
                        writeln!(stdout)?; // Add a newline after the table
                    },
                    _ => {}
                }
                // Important: Reset color after any closing tag that might have applied specific styling
                // We're moving this to be handled by individual End tags where necessary, or implicitly by the next Start tag.
                // For safety, let's keep it here for now if no specific reset happened in the match arm.
                // Or, better, strategically reset in each End arm.
            },
            Event::Text(text) => {
                if in_table {
                    // When in a table, accumulate text for the current cell
                    // This assumes that all text events for a single cell come consecutively
                    // and that `TableCell` start/end delineate cells.
                    // This is a simplification; a robust solution might involve a temporary
                    // buffer for cell content.
                    if current_row_cells.is_empty() { // Need to re-think this.
                        // If it's the first text in a new cell, or after a cell end.
                        write!(stdout, "{}", text)?;
                        current_row_cells.push(text.to_string());
                    } else {
                        // Append to the last cell
                        let last_idx = current_row_cells.len() - 1;
                        current_row_cells[last_idx].push_str(&text);
                    }
                } else {
                    if !in_list && !no_tab && !in_block_quote && !in_code{
                        let tab_prefix = "\t".repeat(text_level);
                        write!(stdout, "{}", tab_prefix)?;
                    }
                    if in_block_quote && first_row && !in_code {
                        first_row = false;
                    } else if in_block_quote && !no_tab && !in_code {
                        let tab_prefix = "\t".repeat(text_level);
                        write!(stdout, "{}  ", tab_prefix)?;
                    } 
                    if in_code {
                        in_code = false;
                        write!(stdout, "~", )?;
                    }
                    write!(stdout, "{}", text)?;
                }
            },
            Event::Code(code) => {
                if in_table {
                    // Handle inline code within tables if needed, currently not accumulating
                    // This adds complexity as `current_row_cells` stores `String`, and `Code` events
                    // also carry content. For a simple CLI, we might just print it or convert to string.
                    if current_row_cells.is_empty() {
                         current_row_cells.push(format!("`{}`", code));
                    } else {
                        let last_idx = current_row_cells.len() - 1;
                        current_row_cells[last_idx].push_str(&format!("`{}`", code));
                    }
                } else {
                    if in_code_block {
                        let tab_prefix = "\t".repeat(text_level);
                        write!(stdout, "{}", tab_prefix)?;
                    } else {
                        in_code = true;
                    }
                    stdout.set_color(&code_color)?;
                    if args.symbol {
                        write!(stdout, "`{}`", code)?;
                    } else {
                        write!(stdout, "{}", code)?;
                    }
                    stdout.reset()?;
                }
            },
            Event::SoftBreak => {
                if in_table {
                    // Soft breaks within table cells usually mean space
                    if let Some(last_cell) = current_row_cells.last_mut() {
                        last_cell.push(' ');
                    }
                } else {
                    writeln!(stdout)?;
                    in_list= false;
                    // write!(stdout, " ")?;
                }
            },
            Event::HardBreak => {
                if in_table {
                    // Hard breaks within table cells might mean a newline, or be ignored
                    if let Some(last_cell) = current_row_cells.last_mut() {
                        last_cell.push('\n'); // Or just a space depending on desired rendering
                    }
                } else {
                    writeln!(stdout)?;
                }
            },
            Event::Rule => {
                writeln!(stdout)?;
                stdout.set_color(&rule_color)?;
                let rule = "---".repeat(text_level + 1);
                let tab_prefix = "\t".repeat(text_level);
                write!(stdout, "{}{}", tab_prefix, rule)?;
                writeln!(stdout)?;
                stdout.reset()?;
            },
            Event::FootnoteReference(name) => write!(stdout, "[^{}]", name)?,
            _ => {}
        }
        stdout.flush()?;
    }

    // Reset colors one last time at the end of the entire parsing process
    stdout.reset()?;
    Ok(())
}