//! Minimal Token enum for AsciiDoc lexer (Phase 1 POC)
//!
//! This module defines a subset of AsciiDoc tokens using the Logos lexer generator.
//! For the POC, we support:
//! - Headings (= through ======)
//! - Paragraph text
//! - Bold formatting (**)
//! - Italic formatting (_)
//! - Newlines and whitespace
//!
//! See `docs/design/features/core-parser.md` for the full token specification.

use logos::Logos;

/// Minimal token set for AsciiDoc POC
///
/// This enum represents the lexical tokens recognized by the AsciiDoc lexer.
/// Each token tracks its position in the source text via Logos' built-in span tracking.
///
/// # Examples
///
/// ```
/// use logos::Logos;
/// use doctora::token::Token;
///
/// let input = "= Heading\n\nThis is **bold**.";
/// let mut lex = Token::lexer(input);
///
/// assert_eq!(lex.next(), Some(Ok(Token::Heading1)));
/// ```
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t]+")] // Skip inline whitespace (spaces and tabs)
pub enum Token {
    // ===== Document Structure =====
    /// Level 1 heading (=)
    ///
    /// Matches "= " at the start of a line.
    /// Example: "= Document Title"
    #[token("=")]
    Heading1,

    /// Level 2 heading (==)
    ///
    /// Matches "== " at the start of a line.
    /// Example: "== Section Title"
    #[token("==")]
    Heading2,

    /// Level 3 heading (===)
    ///
    /// Matches "=== " at the start of a line.
    /// Example: "=== Subsection Title"
    #[token("===")]
    Heading3,

    /// Level 4 heading (====)
    ///
    /// Matches "==== " at the start of a line.
    #[token("====")]
    Heading4,

    /// Level 5 heading (=====)
    ///
    /// Matches "===== " at the start of a line.
    #[token("=====")]
    Heading5,

    /// Level 6 heading (======)
    ///
    /// Matches "====== " at the start of a line.
    /// Example: "====== Deep Subsection"
    #[token("======")]
    Heading6,

    // ===== Inline Formatting =====
    /// Bold delimiter (**)
    ///
    /// Used for bold text formatting.
    /// Example: "**bold text**"
    #[token("**")]
    BoldDelimiter,

    /// Italic delimiter (_)
    ///
    /// Used for italic text formatting.
    /// Example: "_italic text_"
    #[token("_")]
    ItalicDelimiter,

    // ===== Whitespace and Structure =====
    /// Single newline
    ///
    /// Marks the end of a line.
    #[token("\n")]
    Newline,

    /// Blank line (two or more consecutive newlines)
    ///
    /// Separates blocks in AsciiDoc (paragraphs, sections, etc.).
    /// This is significant whitespace in AsciiDoc.
    #[regex(r"\n\n+")]
    BlankLine,

    // ===== Content =====
    /// A word or sequence of non-special characters
    ///
    /// This matches any sequence of characters that aren't formatting delimiters,
    /// whitespace, or special characters.
    ///
    /// Note: The regex is ordered after all other tokens so specific patterns
    /// (like **) are matched first.
    #[regex(r"[^\s\*_=]+")]
    Word,
}

impl Token {
    /// Returns a human-readable description of the token for error messages
    pub fn description(&self) -> &'static str {
        match self {
            Token::Heading1 => "level 1 heading (=)",
            Token::Heading2 => "level 2 heading (==)",
            Token::Heading3 => "level 3 heading (===)",
            Token::Heading4 => "level 4 heading (====)",
            Token::Heading5 => "level 5 heading (=====)",
            Token::Heading6 => "level 6 heading (======)",
            Token::BoldDelimiter => "bold delimiter (**)",
            Token::ItalicDelimiter => "italic delimiter (_)",
            Token::Newline => "newline",
            Token::BlankLine => "blank line",
            Token::Word => "word",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    /// Helper function to collect all tokens from input
    fn lex_all(input: &str) -> Vec<Token> {
        Token::lexer(input)
            .filter_map(|result| result.ok())
            .collect()
    }

    /// Helper function to collect tokens with their spans
    fn lex_with_spans(input: &str) -> Vec<(Token, std::ops::Range<usize>)> {
        let mut lexer = Token::lexer(input);
        let mut result = Vec::new();

        while let Some(token_result) = lexer.next() {
            if let Ok(token) = token_result {
                result.push((token, lexer.span()));
            }
        }

        result
    }

    #[test]
    fn test_heading_levels() {
        assert_eq!(lex_all("="), vec![Token::Heading1]);
        assert_eq!(lex_all("=="), vec![Token::Heading2]);
        assert_eq!(lex_all("==="), vec![Token::Heading3]);
        assert_eq!(lex_all("===="), vec![Token::Heading4]);
        assert_eq!(lex_all("====="), vec![Token::Heading5]);
        assert_eq!(lex_all("======"), vec![Token::Heading6]);
    }

    #[test]
    fn test_heading_with_text() {
        let tokens = lex_all("= Document");
        assert_eq!(tokens, vec![Token::Heading1, Token::Word]);
    }

    #[test]
    fn test_bold_formatting() {
        let tokens = lex_all("**bold**");
        assert_eq!(
            tokens,
            vec![Token::BoldDelimiter, Token::Word, Token::BoldDelimiter]
        );
    }

    #[test]
    fn test_italic_formatting() {
        let tokens = lex_all("_italic_");
        assert_eq!(
            tokens,
            vec![Token::ItalicDelimiter, Token::Word, Token::ItalicDelimiter]
        );
    }

    #[test]
    fn test_mixed_formatting() {
        let tokens = lex_all("This is **bold** and _italic_.");
        assert_eq!(
            tokens,
            vec![
                Token::Word, // This
                Token::Word, // is
                Token::BoldDelimiter,
                Token::Word, // bold
                Token::BoldDelimiter,
                Token::Word, // and
                Token::ItalicDelimiter,
                Token::Word, // italic
                Token::ItalicDelimiter,
                Token::Word, // .
            ]
        );
    }

    #[test]
    fn test_newlines() {
        let tokens = lex_all("line1\nline2");
        assert_eq!(tokens, vec![Token::Word, Token::Newline, Token::Word]);
    }

    #[test]
    fn test_blank_lines() {
        let tokens = lex_all("para1\n\npara2");
        assert_eq!(tokens, vec![Token::Word, Token::BlankLine, Token::Word]);
    }

    #[test]
    fn test_example_document() {
        let input = "= Heading\n\nThis is **bold** and _italic_.";
        let tokens = lex_all(input);

        assert_eq!(
            tokens,
            vec![
                Token::Heading1,
                Token::Word, // Heading
                Token::BlankLine,
                Token::Word, // This
                Token::Word, // is
                Token::BoldDelimiter,
                Token::Word, // bold
                Token::BoldDelimiter,
                Token::Word, // and
                Token::ItalicDelimiter,
                Token::Word, // italic
                Token::ItalicDelimiter,
                Token::Word, // .
            ]
        );
    }

    #[test]
    fn test_span_tracking() {
        let input = "= Title";
        let tokens = lex_with_spans(input);

        assert_eq!(tokens.len(), 2);

        // Heading should be at position 0-1
        assert_eq!(tokens[0].0, Token::Heading1);
        assert_eq!(tokens[0].1, 0..1);

        // "Title" should be at position 2-7 (after space at 1)
        assert_eq!(tokens[1].0, Token::Word);
        assert_eq!(tokens[1].1, 2..7);
    }

    #[test]
    fn test_whitespace_skipping() {
        // Inline whitespace (spaces, tabs) should be skipped
        let tokens = lex_all("word1    word2");
        assert_eq!(tokens, vec![Token::Word, Token::Word]);

        let tokens = lex_all("word1\t\tword2");
        assert_eq!(tokens, vec![Token::Word, Token::Word]);
    }

    #[test]
    fn test_multiple_headings() {
        let input = "= H1\n== H2\n=== H3";
        let tokens = lex_all(input);

        assert_eq!(
            tokens,
            vec![
                Token::Heading1,
                Token::Word, // H1
                Token::Newline,
                Token::Heading2,
                Token::Word, // H2
                Token::Newline,
                Token::Heading3,
                Token::Word, // H3
            ]
        );
    }

    #[test]
    fn test_nested_formatting() {
        // Test that we can handle overlapping delimiters
        // (actual nesting semantics will be handled by parser)
        let tokens = lex_all("**bold _and italic_**");
        assert_eq!(
            tokens,
            vec![
                Token::BoldDelimiter,
                Token::Word, // bold
                Token::ItalicDelimiter,
                Token::Word, // and
                Token::Word, // italic
                Token::ItalicDelimiter,
                Token::BoldDelimiter,
            ]
        );
    }

    #[test]
    fn test_token_description() {
        assert_eq!(Token::Heading1.description(), "level 1 heading (=)");
        assert_eq!(Token::BoldDelimiter.description(), "bold delimiter (**)");
        assert_eq!(Token::ItalicDelimiter.description(), "italic delimiter (_)");
        assert_eq!(Token::Word.description(), "word");
    }

    #[test]
    fn test_empty_input() {
        let tokens = lex_all("");
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_only_whitespace() {
        let tokens = lex_all("   \t  ");
        assert_eq!(tokens, vec![]);
    }
}
