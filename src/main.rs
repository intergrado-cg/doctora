use doctora::ast::{Block, Inline};
use doctora::parse_document;
use doctora::token::Token;
use logos::Logos;

fn main() {
    // Example AsciiDoc input
    let input = r#"= Document Title

This is a paragraph with **bold** text and _italic_ text.

== Section Heading

Another paragraph here."#;

    println!("AsciiDoc Parser Demo");
    println!("{}", "=".repeat(80));
    println!("\nInput:\n{}\n", input);

    // ===== Step 1: Tokenization =====
    println!("{}", "=".repeat(80));
    println!("STEP 1: TOKENIZATION (Logos)");
    println!("{}", "=".repeat(80));

    let mut lexer = Token::lexer(input);

    println!("\n{:<20} {:<20} {:>10}", "Token", "Text", "Span");
    println!("{}", "-".repeat(60));

    let mut token_count = 0;
    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => {
                token_count += 1;
                let span = lexer.span();
                let text = &input[span.clone()];
                let display_text = if text == "\n" {
                    "\\n".to_string()
                } else if text.starts_with("\n\n") {
                    format!("\\n\\n ({})", text.len() - 2)
                } else {
                    text.to_string()
                };
                println!(
                    "{:<20} {:<20} {:>10}",
                    format!("{:?}", token),
                    display_text,
                    format!("{}..{}", span.start, span.end)
                );
            }
            Err(()) => {
                let span = lexer.span();
                let text = &input[span.clone()];
                println!(
                    "{:<20} {:<20} {:>10}",
                    "ERROR",
                    text,
                    format!("{}..{}", span.start, span.end)
                );
            }
        }
    }

    println!("\nTotal tokens: {}", token_count);

    // ===== Step 2: Parsing =====
    println!("\n{}", "=".repeat(80));
    println!("STEP 2: PARSING (Winnow)");
    println!("{}", "=".repeat(80));

    match parse_document(input) {
        Ok(doc) => {
            println!("\nParsing successful!");
            println!("\nAST Structure:");
            println!("{}", "-".repeat(80));
            print_document(&doc, 0);
            println!("\n{}", "=".repeat(80));
            println!("Summary:");
            println!("  Total blocks: {}", doc.blocks.len());
            println!("{}", "=".repeat(80));
        }
        Err(error) => {
            println!("\nParsing failed:");
            println!("  {}", error);
        }
    }
}

/// Pretty-print the document AST
fn print_document(doc: &doctora::ast::Document, indent: usize) {
    for (i, block) in doc.blocks.iter().enumerate() {
        print_block(block, indent, i);
    }
}

/// Pretty-print a block with indentation
fn print_block(block: &Block, indent: usize, index: usize) {
    let indent_str = "  ".repeat(indent);

    match block {
        Block::Section {
            level,
            title,
            content,
        } => {
            println!(
                "{}Block {}: Section (level {})",
                indent_str, index, level
            );
            println!("{}  Title: {:?}", indent_str, title);
            if !content.is_empty() {
                println!("{}  Content: {} nested blocks", indent_str, content.len());
                for (i, nested) in content.iter().enumerate() {
                    print_block(nested, indent + 2, i);
                }
            } else {
                println!("{}  Content: (empty)", indent_str);
            }
        }
        Block::Paragraph { content } => {
            println!("{}Block {}: Paragraph", indent_str, index);
            println!(
                "{}  Inline nodes: {} items",
                indent_str,
                content.len()
            );
            for (i, inline) in content.iter().enumerate() {
                print_inline(inline, indent + 2, i);
            }
        }
    }
}

/// Pretty-print an inline node
fn print_inline(inline: &Inline, indent: usize, index: usize) {
    let indent_str = "  ".repeat(indent);

    match inline {
        Inline::Text(text) => {
            println!("{}Inline {}: Text({:?})", indent_str, index, text);
        }
        Inline::Bold(content) => {
            println!("{}Inline {}: Bold", indent_str, index);
            for (i, nested) in content.iter().enumerate() {
                print_inline(nested, indent + 1, i);
            }
        }
        Inline::Italic(content) => {
            println!("{}Inline {}: Italic", indent_str, index);
            for (i, nested) in content.iter().enumerate() {
                print_inline(nested, indent + 1, i);
            }
        }
    }
}
