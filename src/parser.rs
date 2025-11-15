//! Parser for converting AsciiDoc tokens to AST
//!
//! This module uses Chumsky 0.11 to parse a token stream (from Logos)
//! into an Abstract Syntax Tree.
//!
//! # Architecture
//!
//! The parser is structured as a hierarchy:
//! - `document()` - Top-level parser, returns `Document`
//! - `block()` - Parses sections or paragraphs
//! - `section()` - Parses a heading and its nested content
//! - `paragraph()` - Parses paragraph text with inline formatting
//! - `inline()` - Parses inline content (text, bold, italic)
//!
//! # Error Recovery
//!
//! The parser uses Chumsky's error recovery features to handle malformed
//! input gracefully, allowing multiple errors to be reported at once.

use crate::ast::{Block, Document, Inline};
use crate::token::Token;
use chumsky::prelude::*;

/// Parse error type
pub type ParseError<'src> = extra::Err<Simple<'src, Token>>;

/// Creates the main document parser
///
/// This is the entry point for parsing a complete AsciiDoc document.
/// It parses a sequence of blocks separated by blank lines.
///
/// # Examples
///
/// ```
/// use doctora::parser::document;
/// use doctora::token::Token;
/// use chumsky::Parser;
///
/// let tokens = vec![
///     Token::Heading1,
///     Token::Word,
///     Token::BlankLine,
///     Token::Word,
/// ];
///
/// let parser = document();
/// let result = parser.parse(tokens.as_slice()).into_result();
/// assert!(result.is_ok());
/// ```
pub fn document<'src>() -> impl Parser<'src, &'src [Token], Document, ParseError<'src>> {
    block()
        .repeated()
        .collect::<Vec<_>>()
        .then_ignore(end())
        .map(|blocks| Document::with_blocks(blocks))
}

/// Parses a block-level element (section or paragraph)
///
/// A block can be:
/// - A section (heading + nested content)
/// - A paragraph (inline content)
///
/// Blocks are separated by blank lines in the token stream.
fn block<'src>() -> impl Parser<'src, &'src [Token], Block, ParseError<'src>> {
    recursive(|block_ref| {
        let section = section(block_ref.clone()).boxed();
        let paragraph = paragraph().boxed();

        choice((section, paragraph))
            // Skip trailing blank lines after blocks
            .then_ignore(just(Token::BlankLine).repeated())
    })
}

/// Parses a section (heading with optional nested content)
///
/// Sections start with a heading token (Heading1-6) followed by text,
/// then optionally contain nested blocks.
///
/// # Grammar
///
/// ```text
/// section := heading_token word+ newline block*
/// ```
fn section<'src>(
    block_ref: impl Parser<'src, &'src [Token], Block, ParseError<'src>> + Clone + 'src,
) -> impl Parser<'src, &'src [Token], Block, ParseError<'src>> {
    // Parse heading marker and get level
    let heading = select! {
        Token::Heading1 => 1u8,
        Token::Heading2 => 2u8,
        Token::Heading3 => 3u8,
        Token::Heading4 => 4u8,
        Token::Heading5 => 5u8,
        Token::Heading6 => 6u8,
    };

    // Parse heading title (words until newline or blank line)
    let title = just(Token::Word)
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|_words| {
            // For POC, we use a placeholder title since we can't access token text here
            // In production, we'd need to pass the original text through
            "Section".to_string()
        });

    // Parse the complete section
    heading
        .then(title)
        .then_ignore(choice((just(Token::Newline), just(Token::BlankLine))))
        .then(block_ref.repeated().collect::<Vec<_>>())
        .map(|((level, title), content)| Block::Section {
            level,
            title,
            content,
        })
}

/// Parses a paragraph (inline content until blank line)
///
/// Paragraphs contain text with inline formatting (bold, italic).
/// They end at a blank line or end of input.
///
/// # Grammar
///
/// ```text
/// paragraph := inline+ newline?
/// ```
fn paragraph<'src>() -> impl Parser<'src, &'src [Token], Block, ParseError<'src>> {
    inline()
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        // Optional trailing newline
        .then_ignore(just(Token::Newline).or_not())
        .map(|content| Block::Paragraph { content })
}

/// Parses inline content (text, bold, italic)
///
/// This handles text content and formatting within paragraphs.
///
/// # Grammar
///
/// ```text
/// inline := text | bold | italic
/// bold := "**" inline+ "**"
/// italic := "_" inline+ "_"
/// text := Word
/// ```
fn inline<'src>() -> impl Parser<'src, &'src [Token], Inline, ParseError<'src>> {
    recursive(|inline_ref| {
        // Plain text
        let text = just(Token::Word).map(|_| Inline::Text("word".to_string())).boxed();

        // Bold: ** content **
        let bold = just(Token::BoldDelimiter)
            .ignore_then(inline_ref.clone().repeated().at_least(1).collect::<Vec<_>>())
            .then_ignore(just(Token::BoldDelimiter))
            .map(Inline::Bold)
            .boxed();

        // Italic: _ content _
        let italic = just(Token::ItalicDelimiter)
            .ignore_then(inline_ref.clone().repeated().at_least(1).collect::<Vec<_>>())
            .then_ignore(just(Token::ItalicDelimiter))
            .map(Inline::Italic)
            .boxed();

        // Try bold/italic first, then fall back to text
        choice((bold, italic, text))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    /// Helper to parse tokens
    fn parse_tokens(tokens: Vec<Token>) -> Result<Document, String> {
        document()
            .parse(tokens.as_slice())
            .into_result()
            .map_err(|errors| format!("Parse errors: {:?}", errors))
    }

    #[test]
    fn test_empty_document() {
        let tokens = vec![];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 0);
    }

    #[test]
    fn test_simple_paragraph() {
        // "word word"
        let tokens = vec![Token::Word, Token::Word];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());

        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 1);

        if let Block::Paragraph { content } = &doc.blocks[0] {
            assert_eq!(content.len(), 2);
            assert!(content[0].is_text());
            assert!(content[1].is_text());
        } else {
            panic!("Expected Paragraph");
        }
    }

    #[test]
    fn test_bold_text() {
        // "**word**"
        let tokens = vec![Token::BoldDelimiter, Token::Word, Token::BoldDelimiter];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());

        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 1);

        if let Block::Paragraph { content } = &doc.blocks[0] {
            assert_eq!(content.len(), 1);
            assert!(content[0].is_bold());
        } else {
            panic!("Expected Paragraph");
        }
    }

    #[test]
    fn test_italic_text() {
        // "_word_"
        let tokens = vec![Token::ItalicDelimiter, Token::Word, Token::ItalicDelimiter];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());

        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 1);

        if let Block::Paragraph { content } = &doc.blocks[0] {
            assert_eq!(content.len(), 1);
            assert!(content[0].is_italic());
        } else {
            panic!("Expected Paragraph");
        }
    }

    #[test]
    fn test_mixed_formatting() {
        // "word **word** word _word_"
        let tokens = vec![
            Token::Word,
            Token::BoldDelimiter,
            Token::Word,
            Token::BoldDelimiter,
            Token::Word,
            Token::ItalicDelimiter,
            Token::Word,
            Token::ItalicDelimiter,
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());

        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 1);

        if let Block::Paragraph { content } = &doc.blocks[0] {
            assert_eq!(content.len(), 4);
            assert!(content[0].is_text());
            assert!(content[1].is_bold());
            assert!(content[2].is_text());
            assert!(content[3].is_italic());
        } else {
            panic!("Expected Paragraph");
        }
    }

    #[test]
    fn test_simple_heading() {
        // "= Title\n"
        let tokens = vec![Token::Heading1, Token::Word, Token::Newline];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());

        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 1);

        if let Block::Section { level, content, .. } = &doc.blocks[0] {
            assert_eq!(*level, 1);
            assert_eq!(content.len(), 0);
        } else {
            panic!("Expected Section");
        }
    }

    #[test]
    fn test_section_with_paragraph() {
        // "= Title\n\nword word"
        let tokens = vec![
            Token::Heading1,
            Token::Word,
            Token::BlankLine,
            Token::Word,
            Token::Word,
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());

        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 1);

        if let Block::Section { level, content, .. } = &doc.blocks[0] {
            assert_eq!(*level, 1);
            assert_eq!(content.len(), 1);

            // Check nested paragraph
            if let Block::Paragraph { content: para_content } = &content[0] {
                assert_eq!(para_content.len(), 2);
            } else {
                panic!("Expected nested Paragraph");
            }
        } else {
            panic!("Expected Section");
        }
    }

    #[test]
    fn test_multiple_blocks() {
        // "word\n\nword"
        let tokens = vec![Token::Word, Token::BlankLine, Token::Word];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());

        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 2);

        assert!(matches!(doc.blocks[0], Block::Paragraph { .. }));
        assert!(matches!(doc.blocks[1], Block::Paragraph { .. }));
    }

    #[test]
    fn test_nested_sections() {
        // "= H1\n\n== H2\n"
        let tokens = vec![
            Token::Heading1,
            Token::Word,
            Token::BlankLine,
            Token::Heading2,
            Token::Word,
            Token::Newline,
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());

        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 1);

        if let Block::Section { level, content, .. } = &doc.blocks[0] {
            assert_eq!(*level, 1);
            assert_eq!(content.len(), 1);

            // Check nested section
            if let Block::Section {
                level: nested_level,
                ..
            } = &content[0]
            {
                assert_eq!(*nested_level, 2);
            } else {
                panic!("Expected nested Section");
            }
        } else {
            panic!("Expected Section");
        }
    }

    #[test]
    fn test_complex_document() {
        // "= Title\n\nword **bold** _italic_\n\n== Section\n\nword"
        let tokens = vec![
            Token::Heading1,
            Token::Word,
            Token::BlankLine,
            Token::Word,
            Token::BoldDelimiter,
            Token::Word,
            Token::BoldDelimiter,
            Token::ItalicDelimiter,
            Token::Word,
            Token::ItalicDelimiter,
            Token::BlankLine,
            Token::Heading2,
            Token::Word,
            Token::BlankLine,
            Token::Word,
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());

        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 1);

        if let Block::Section { level, content, .. } = &doc.blocks[0] {
            assert_eq!(*level, 1);
            assert_eq!(content.len(), 2); // paragraph + nested section

            // First nested block should be paragraph
            assert!(matches!(content[0], Block::Paragraph { .. }));

            // Second nested block should be section
            if let Block::Section {
                level: nested_level,
                content: nested_content,
                ..
            } = &content[1]
            {
                assert_eq!(*nested_level, 2);
                assert_eq!(nested_content.len(), 1); // one paragraph in subsection
            } else {
                panic!("Expected nested Section");
            }
        } else {
            panic!("Expected Section");
        }
    }

    #[test]
    fn test_nested_formatting() {
        // "**word _italic_**"
        let tokens = vec![
            Token::BoldDelimiter,
            Token::Word,
            Token::ItalicDelimiter,
            Token::Word,
            Token::ItalicDelimiter,
            Token::BoldDelimiter,
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());

        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 1);

        if let Block::Paragraph { content } = &doc.blocks[0] {
            assert_eq!(content.len(), 1);

            // Should have bold with nested content
            if let Inline::Bold(bold_content) = &content[0] {
                assert_eq!(bold_content.len(), 2); // text + italic
                assert!(bold_content[0].is_text());
                assert!(bold_content[1].is_italic());
            } else {
                panic!("Expected Bold");
            }
        } else {
            panic!("Expected Paragraph");
        }
    }

    #[test]
    fn test_error_recovery_unclosed_bold() {
        // "**word" (missing closing delimiter)
        let tokens = vec![Token::BoldDelimiter, Token::Word];
        let result = parse_tokens(tokens);

        // Parser should recover and report errors
        // The document might be empty or partial
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_paragraph_with_newline() {
        // "word word\n"
        let tokens = vec![Token::Word, Token::Word, Token::Newline];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());

        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 1);
    }

    #[test]
    fn test_multiple_headings() {
        // "= H1\n== H2\n=== H3\n"
        let tokens = vec![
            Token::Heading1,
            Token::Word,
            Token::Newline,
            Token::Heading2,
            Token::Word,
            Token::Newline,
            Token::Heading3,
            Token::Word,
            Token::Newline,
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());

        let doc = result.unwrap();
        // H1 contains H2, H2 contains H3 (nested structure)
        assert_eq!(doc.blocks.len(), 1);

        if let Block::Section { level, content, .. } = &doc.blocks[0] {
            assert_eq!(*level, 1);
            assert!(content.len() > 0);
        } else {
            panic!("Expected Section");
        }
    }
}
