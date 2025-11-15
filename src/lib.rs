//! Doctor A - A modular AsciiDoc parser and processor written in Rust
//!
//! This library provides a high-performance AsciiDoc parser that separates
//! parsing from processing, enabling pluggable output formats.
//!
//! # Architecture
//!
//! ```text
//! AsciiDoc Input → Core Parser → AST → Processor Layer → Output Formats
//!                  (Lexer→Parser)      (Plugin System)   (HTML, PDF, etc.)
//! ```
//!
//! # Examples
//!
//! Parse an AsciiDoc document:
//!
//! ```
//! use doctora::parse_document;
//!
//! let input = "= Hello, AsciiDoc!\n\nThis is **bold** and _italic_.";
//! let result = parse_document(input);
//!
//! match result {
//!     Ok(doc) => println!("Parsed {} blocks", doc.blocks.len()),
//!     Err(errors) => eprintln!("Parse errors: {:?}", errors),
//! }
//! ```
//!
//! # Modules
//!
//! - [`token`] - Lexical tokens for AsciiDoc (Logos-based lexer)
//! - [`ast`] - Abstract Syntax Tree types
//! - [`parser_winnow`] - Winnow-based parser for building AST from tokens
//! - [`error_recovery`] - Custom error recovery layer (design/future implementation)

pub mod ast;
pub mod error_recovery;
pub mod parser_winnow;
pub mod token;

use ast::Document;
use logos::Logos;
use token::Token;

/// Parse an AsciiDoc document from text input
///
/// This is the main entry point for parsing AsciiDoc. It performs two steps:
/// 1. Lexical analysis: Convert text to tokens using Logos
/// 2. Parsing: Build an AST from tokens using Winnow
///
/// # Arguments
///
/// * `input` - AsciiDoc source text
///
/// # Returns
///
/// Returns `Ok(Document)` on success, or `Err(String)` if parsing fails.
///
/// # Examples
///
/// ```
/// use doctora::parse_document;
///
/// let input = r#"= Document Title
///
/// This is a paragraph with **bold** text.
///
/// == Section
///
/// Another paragraph with _italic_ text.
/// "#;
///
/// let doc = parse_document(input).expect("parse failed");
/// assert_eq!(doc.blocks.len(), 1); // One top-level section
/// ```
///
/// # Errors
///
/// Parse errors include:
/// - Unexpected tokens
/// - Unclosed formatting delimiters
/// - Invalid document structure
///
/// # Performance
///
/// Uses Winnow parser (16-45% faster than alternatives) with zero-copy design
/// for optimal performance on documents of all sizes.
pub fn parse_document(input: &str) -> Result<Document, String> {
    // Step 1: Lex the input into tokens
    let tokens: Vec<Token> = Token::lexer(input)
        .filter_map(|result| result.ok()) // Skip lexer errors for now
        .collect();

    // Step 2: Parse tokens into AST using Winnow
    parser_winnow::parse_document_winnow(&tokens)
}
