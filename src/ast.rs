//! Minimal AST types for AsciiDoc POC
//!
//! This module defines a simplified Abstract Syntax Tree for Phase 1 proof-of-concept.
//! The full AST specification is in `docs/design/features/core-parser.md`.
//!
//! # Structure
//!
//! The AST is hierarchical:
//! - `Document` contains a vector of `Block` nodes
//! - `Block` can be a `Section` (heading with nested blocks) or `Paragraph`
//! - `Paragraph` contains a vector of `Inline` nodes
//! - `Inline` can be plain `Text`, `Bold`, or `Italic` formatting

/// Root document node
///
/// Represents the entire parsed AsciiDoc document.
///
/// # Examples
///
/// ```
/// use doctora::ast::{Document, Block, Inline};
///
/// let doc = Document {
///     blocks: vec![
///         Block::Section {
///             level: 1,
///             title: "Document Title".to_string(),
///             content: vec![],
///         },
///     ],
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    /// Top-level blocks in the document
    pub blocks: Vec<Block>,
}

impl Document {
    /// Creates a new empty document
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    /// Creates a document with the given blocks
    pub fn with_blocks(blocks: Vec<Block>) -> Self {
        Self { blocks }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

/// Block-level AST nodes
///
/// Blocks represent structural elements like sections and paragraphs.
/// In AsciiDoc, blocks are separated by blank lines.
///
/// # Examples
///
/// ```
/// use doctora::ast::{Block, Inline};
///
/// // Section with nested content
/// let section = Block::Section {
///     level: 2,
///     title: "Section Title".to_string(),
///     content: vec![
///         Block::Paragraph {
///             content: vec![Inline::Text("Paragraph text".to_string())],
///         },
///     ],
/// };
///
/// // Simple paragraph
/// let para = Block::Paragraph {
///     content: vec![
///         Inline::Text("Some ".to_string()),
///         Inline::Bold(vec![Inline::Text("bold".to_string())]),
///         Inline::Text(" text".to_string()),
///     ],
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    /// Section (heading with nested content)
    ///
    /// In AsciiDoc, sections are created by headings (= through ======).
    /// Sections can contain nested blocks and subsections.
    Section {
        /// Heading level (1-6, where 1 is the top level)
        level: u8,
        /// Section title (plain text for POC)
        title: String,
        /// Nested blocks (paragraphs, subsections, etc.)
        content: Vec<Block>,
    },

    /// Paragraph (text with inline formatting)
    ///
    /// Paragraphs contain inline content (text, bold, italic).
    /// Multiple consecutive non-blank lines form a single paragraph.
    Paragraph {
        /// Inline content (text and formatting)
        content: Vec<Inline>,
    },
}

/// Inline-level AST nodes
///
/// Inline nodes represent text content and formatting within paragraphs.
///
/// # Examples
///
/// ```
/// use doctora::ast::Inline;
///
/// // Plain text
/// let text = Inline::Text("Hello".to_string());
///
/// // Bold text
/// let bold = Inline::Bold(vec![
///     Inline::Text("bold text".to_string()),
/// ]);
///
/// // Nested formatting: bold text with italic inside
/// let nested = Inline::Bold(vec![
///     Inline::Text("bold ".to_string()),
///     Inline::Italic(vec![Inline::Text("and italic".to_string())]),
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    /// Plain text content
    Text(String),

    /// Bold formatting (**)
    ///
    /// Contains the formatted inline content.
    /// Can be nested with other formatting.
    Bold(Vec<Inline>),

    /// Italic formatting (_)
    ///
    /// Contains the formatted inline content.
    /// Can be nested with other formatting.
    Italic(Vec<Inline>),
}

impl Inline {
    /// Checks if this inline node is text
    pub fn is_text(&self) -> bool {
        matches!(self, Inline::Text(_))
    }

    /// Checks if this inline node is bold
    pub fn is_bold(&self) -> bool {
        matches!(self, Inline::Bold(_))
    }

    /// Checks if this inline node is italic
    pub fn is_italic(&self) -> bool {
        matches!(self, Inline::Italic(_))
    }

    /// Extracts text content if this is a Text node
    pub fn as_text(&self) -> Option<&str> {
        if let Inline::Text(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new();
        assert_eq!(doc.blocks.len(), 0);

        let doc = Document::with_blocks(vec![Block::Paragraph {
            content: vec![Inline::Text("Test".to_string())],
        }]);
        assert_eq!(doc.blocks.len(), 1);
    }

    #[test]
    fn test_document_default() {
        let doc = Document::default();
        assert_eq!(doc.blocks.len(), 0);
    }

    #[test]
    fn test_block_paragraph() {
        let para = Block::Paragraph {
            content: vec![Inline::Text("Test".to_string())],
        };

        if let Block::Paragraph { content } = para {
            assert_eq!(content.len(), 1);
        } else {
            panic!("Expected Paragraph");
        }
    }

    #[test]
    fn test_block_section() {
        let section = Block::Section {
            level: 1,
            title: "Title".to_string(),
            content: vec![],
        };

        if let Block::Section { level, title, content } = section {
            assert_eq!(level, 1);
            assert_eq!(title, "Title");
            assert_eq!(content.len(), 0);
        } else {
            panic!("Expected Section");
        }
    }

    #[test]
    fn test_nested_section() {
        let section = Block::Section {
            level: 1,
            title: "Main".to_string(),
            content: vec![
                Block::Paragraph {
                    content: vec![Inline::Text("Para".to_string())],
                },
                Block::Section {
                    level: 2,
                    title: "Sub".to_string(),
                    content: vec![],
                },
            ],
        };

        if let Block::Section { level, content, .. } = section {
            assert_eq!(level, 1);
            assert_eq!(content.len(), 2);
        } else {
            panic!("Expected Section");
        }
    }

    #[test]
    fn test_inline_text() {
        let text = Inline::Text("Hello".to_string());
        assert!(text.is_text());
        assert!(!text.is_bold());
        assert!(!text.is_italic());
        assert_eq!(text.as_text(), Some("Hello"));
    }

    #[test]
    fn test_inline_bold() {
        let bold = Inline::Bold(vec![Inline::Text("Bold".to_string())]);
        assert!(!bold.is_text());
        assert!(bold.is_bold());
        assert!(!bold.is_italic());
        assert_eq!(bold.as_text(), None);
    }

    #[test]
    fn test_inline_italic() {
        let italic = Inline::Italic(vec![Inline::Text("Italic".to_string())]);
        assert!(!italic.is_text());
        assert!(!italic.is_bold());
        assert!(italic.is_italic());
    }

    #[test]
    fn test_nested_inline() {
        let nested = Inline::Bold(vec![
            Inline::Text("Bold ".to_string()),
            Inline::Italic(vec![Inline::Text("and italic".to_string())]),
        ]);

        if let Inline::Bold(content) = nested {
            assert_eq!(content.len(), 2);
            assert!(content[0].is_text());
            assert!(content[1].is_italic());
        } else {
            panic!("Expected Bold");
        }
    }

    #[test]
    fn test_complex_paragraph() {
        let para = Block::Paragraph {
            content: vec![
                Inline::Text("This is ".to_string()),
                Inline::Bold(vec![Inline::Text("bold".to_string())]),
                Inline::Text(" and ".to_string()),
                Inline::Italic(vec![Inline::Text("italic".to_string())]),
                Inline::Text(".".to_string()),
            ],
        };

        if let Block::Paragraph { content } = para {
            assert_eq!(content.len(), 5);
        } else {
            panic!("Expected Paragraph");
        }
    }
}
