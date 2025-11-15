//! Parser for converting AsciiDoc tokens to AST using Winnow
//!
//! This module uses Winnow 0.7 to parse a token stream (from Logos)
//! into an Abstract Syntax Tree. It mirrors the Chumsky parser structure
//! for fair comparison.
//!
//! # Architecture
//!
//! The parser is structured as a hierarchy:
//! - `document()` - Top-level parser, returns `Document`
//! - `block()` - Parses sections or paragraphs
//! - `section()` - Parses a heading and its nested content
//! - `paragraph()` - Parses paragraph text with inline formatting
//! - `inline()` - Parses inline content (text, bold, italic)

use crate::ast::{Block, Document, Inline};
use crate::token::Token;
use winnow::combinator::{alt, delimited, opt, repeat, terminated};
use winnow::prelude::*;
use winnow::token::any;

/// Input type for Winnow parser
type Input<'a> = &'a [Token];

/// Parse a complete AsciiDoc document
///
/// Entry point for parsing. Parses a sequence of blocks and returns a Document.
///
/// # Examples
///
/// ```
/// use doctora::parser_winnow::parse_document_winnow;
/// use doctora::token::Token;
///
/// let tokens = vec![
///     Token::Heading1,
///     Token::Word,
///     Token::BlankLine,
///     Token::Word,
/// ];
///
/// let result = parse_document_winnow(&tokens);
/// assert!(result.is_ok());
/// ```
pub fn parse_document_winnow(input: &[Token]) -> Result<Document, String> {
    let mut parser = terminated(repeat(0.., block), winnow::combinator::eof);

    parser
        .parse(input)
        .map(|blocks| Document::with_blocks(blocks))
        .map_err(|err| format!("Parse error: {:?}", err))
}

/// Parse a block-level element (section or paragraph)
fn block(input: &mut Input<'_>) -> winnow::Result<Block> {
    // Try to parse a section first, then fall back to paragraph
    alt((section, paragraph)).parse_next(input)
}

/// Parse a section (heading with optional nested content)
fn section(input: &mut Input<'_>) -> winnow::Result<Block> {
    // Parse heading marker and get level
    let level = heading_level.parse_next(input)?;

    // Parse heading title (one or more words)
    let _title_words: Vec<Token> = repeat(1.., token(Token::Word)).parse_next(input)?;

    // For POC, use placeholder title (same as Chumsky)
    let title = "Section".to_string();

    // Consume newline or blank line after heading
    alt((token(Token::Newline), token(Token::BlankLine))).parse_next(input)?;

    // Parse nested blocks
    let content: Vec<Block> = repeat(0.., block).parse_next(input)?;

    // Skip trailing blank lines
    let _: Vec<Token> = repeat(0.., token(Token::BlankLine)).parse_next(input)?;

    Ok(Block::Section {
        level,
        title,
        content,
    })
}

/// Parse heading level from heading token
fn heading_level(input: &mut Input<'_>) -> winnow::Result<u8> {
    any.verify_map(|token| match token {
        Token::Heading1 => Some(1u8),
        Token::Heading2 => Some(2u8),
        Token::Heading3 => Some(3u8),
        Token::Heading4 => Some(4u8),
        Token::Heading5 => Some(5u8),
        Token::Heading6 => Some(6u8),
        _ => None,
    })
    .parse_next(input)
}

/// Parse a paragraph (inline content until blank line)
fn paragraph(input: &mut Input<'_>) -> winnow::Result<Block> {
    // Parse one or more inline elements
    let content: Vec<Inline> = repeat(1.., inline).parse_next(input)?;

    // Optional trailing newline
    let _: Option<Token> = opt(token(Token::Newline)).parse_next(input)?;

    // Skip trailing blank lines
    let _: Vec<Token> = repeat(0.., token(Token::BlankLine)).parse_next(input)?;

    Ok(Block::Paragraph { content })
}

/// Parse inline content (text, bold, italic)
fn inline(input: &mut Input<'_>) -> winnow::Result<Inline> {
    alt((bold, italic, text)).parse_next(input)
}

/// Parse plain text (word token)
fn text(input: &mut Input<'_>) -> winnow::Result<Inline> {
    token(Token::Word)
        .map(|_| Inline::Text("word".to_string()))
        .parse_next(input)
}

/// Parse bold formatting: ** content **
fn bold(input: &mut Input<'_>) -> winnow::Result<Inline> {
    delimited(
        token(Token::BoldDelimiter),
        repeat::<_, _, Vec<Inline>, _, _>(1.., inline),
        token(Token::BoldDelimiter),
    )
    .map(Inline::Bold)
    .parse_next(input)
}

/// Parse italic formatting: _ content _
fn italic(input: &mut Input<'_>) -> winnow::Result<Inline> {
    delimited(
        token(Token::ItalicDelimiter),
        repeat::<_, _, Vec<Inline>, _, _>(1.., inline),
        token(Token::ItalicDelimiter),
    )
    .map(Inline::Italic)
    .parse_next(input)
}

/// Helper: Match a specific token
fn token<'a>(expected: Token) -> impl Parser<Input<'a>, Token, winnow::error::ContextError> {
    any.verify(move |t: &Token| *t == expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_empty_document() {
        let tokens = vec![];
        let result = parse_document_winnow(&tokens);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.blocks.len(), 0);
    }

    #[test]
    fn test_simple_paragraph() {
        // "word word"
        let tokens = vec![Token::Word, Token::Word];
        let result = parse_document_winnow(&tokens);
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
        let result = parse_document_winnow(&tokens);
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
        let result = parse_document_winnow(&tokens);
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
        let result = parse_document_winnow(&tokens);
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
        let result = parse_document_winnow(&tokens);
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
        let result = parse_document_winnow(&tokens);
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
        let result = parse_document_winnow(&tokens);
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
        let result = parse_document_winnow(&tokens);
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
        let result = parse_document_winnow(&tokens);
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
        let result = parse_document_winnow(&tokens);
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
    fn test_paragraph_with_newline() {
        // "word word\n"
        let tokens = vec![Token::Word, Token::Word, Token::Newline];
        let result = parse_document_winnow(&tokens);
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
        let result = parse_document_winnow(&tokens);
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
