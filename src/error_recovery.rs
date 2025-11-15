//! Custom error recovery layer for Winnow parser
//!
//! This module provides structured error handling and recovery capabilities
//! on top of Winnow's basic error reporting. The goal is to:
//! 1. Report multiple errors in a single parse (not just first error)
//! 2. Provide precise source locations with line/column information
//! 3. Generate helpful error messages with context and suggestions
//! 4. Integrate with `miette` for beautiful diagnostic output
//!
//! # Design Principles
//!
//! ## 1. Error Collection
//! Unlike Winnow's default fail-fast behavior, we want to collect ALL errors:
//! - Continue parsing after recoverable errors
//! - Accumulate errors in a collection
//! - Return both partial AST + error list
//!
//! ## 2. Error Types
//!
//! Different error categories require different recovery strategies:
//! - **Syntax Errors**: Unexpected tokens, missing delimiters
//! - **Structural Errors**: Invalid nesting, mismatched blocks
//! - **Semantic Errors**: Undefined references, invalid attributes
//!
//! ## 3. Recovery Strategies
//!
//! ### Synchronization Points
//! Resume parsing at known-good boundaries:
//! - Block boundaries (blank lines, headings)
//! - End of delimited sections
//! - Start of new structures
//!
//! ### Panic Mode Recovery
//! Skip tokens until synchronization point:
//! ```ignore
//! fn recover_to_block_boundary(input: &mut Input) {
//!     while !matches!(peek(input), Some(Token::BlankLine | Token::Heading*)) {
//!         advance(input);
//!     }
//! }
//! ```
//!
//! ### Error Productions
//! Insert placeholder nodes for invalid syntax:
//! ```ignore
//! Block::Error {
//!     message: "Unclosed bold delimiter".to_string(),
//!     span: 45..50,
//! }
//! ```
//!
//! ## 4. Miette Integration
//!
//! Use `miette` for beautiful error reporting:
//! ```ignore
//! #[derive(Error, Diagnostic, Debug)]
//! #[error("Unclosed bold delimiter")]
//! #[diagnostic(
//!     code(doctora::parser::unclosed_bold),
//!     help("Add closing `**` delimiter")
//! )]
//! struct UnclosedBoldError {
//!     #[source_code]
//!     src: NamedSource,
//!     #[label("Bold starts here")]
//!     start: SourceSpan,
//!     #[label("Expected closing `**` before here")]
//!     end: SourceSpan,
//! }
//! ```
//!
//! # Implementation Plan
//!
//! ## Phase 1: Basic Error Types (Current - v0.1.0)
//! - [ ] Define `ParseError` struct with location info
//! - [ ] Define error categories (syntax, structure, semantic)
//! - [ ] Convert Winnow errors to structured errors
//! - [ ] Single error reporting (fail-fast)
//!
//! ## Phase 2: Multiple Error Collection (v0.2.0)
//! - [ ] Add error collector to parser state
//! - [ ] Implement synchronization points
//! - [ ] Return `(Option<AST>, Vec<Error>)`
//! - [ ] Test with intentionally broken documents
//!
//! ## Phase 3: Advanced Recovery (v0.3.0)
//! - [ ] Implement panic mode recovery
//! - [ ] Add error productions (placeholder nodes)
//! - [ ] Smart suggestions based on context
//! - [ ] Fuzzy matching for typos
//!
//! ## Phase 4: Miette Integration (v0.3.0)
//! - [ ] Derive `Diagnostic` for all error types
//! - [ ] Add source code snippets to errors
//! - [ ] Color-coded error output
//! - [ ] Multi-error rendering
//!
//! # Example Error Output (Future)
//!
//! ```text
//! Error: Unclosed bold delimiter
//!    ┌─ document.adoc:12:45
//!    │
//! 12 │ This paragraph has **bold text that never closes.
//!    │                    ─┬ unclosed bold starts here
//!    │                     ╰─ help: Add closing `**` delimiter
//!    │
//!    = Note: Bold delimiters must be balanced within a paragraph
//! ```

use thiserror::Error;

/// Parser error with location information
///
/// This will be expanded in future versions to include:
/// - Span information (start/end byte positions)
/// - Line/column numbers
/// - Context snippets
/// - Suggested fixes
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Unexpected token encountered
    #[error("Unexpected token at position {position}: expected {expected}, got {got}")]
    UnexpectedToken {
        position: usize,
        expected: String,
        got: String,
    },

    /// Unclosed delimiter (bold, italic, etc.)
    #[error("Unclosed {delimiter} delimiter starting at position {start}")]
    UnclosedDelimiter { delimiter: String, start: usize },

    /// Invalid document structure
    #[error("Invalid structure: {message}")]
    InvalidStructure { message: String },

    /// End of input reached unexpectedly
    #[error("Unexpected end of input: {context}")]
    UnexpectedEOF { context: String },
}

/// Error recovery context
///
/// Maintains state during error recovery:
/// - Collected errors
/// - Current recovery strategy
/// - Synchronization points
///
/// **Status**: Design only - not yet implemented
#[allow(dead_code)]
pub struct ErrorRecovery {
    /// Accumulated errors during parsing
    errors: Vec<ParseError>,

    /// Whether to continue after errors
    fail_fast: bool,
}

#[allow(dead_code)]
impl ErrorRecovery {
    /// Create new error recovery context
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            fail_fast: true, // Default to fail-fast for now
        }
    }

    /// Record an error during parsing
    pub fn record_error(&mut self, error: ParseError) {
        self.errors.push(error);
    }

    /// Get all collected errors
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    /// Check if any errors were collected
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl Default for ErrorRecovery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_recovery_creation() {
        let recovery = ErrorRecovery::new();
        assert!(!recovery.has_errors());
        assert_eq!(recovery.errors().len(), 0);
    }

    #[test]
    fn test_record_error() {
        let mut recovery = ErrorRecovery::new();
        recovery.record_error(ParseError::UnexpectedEOF {
            context: "test".to_string(),
        });
        assert!(recovery.has_errors());
        assert_eq!(recovery.errors().len(), 1);
    }

    #[test]
    fn test_parse_error_display() {
        let error = ParseError::UnclosedDelimiter {
            delimiter: "**".to_string(),
            start: 42,
        };
        let message = error.to_string();
        assert!(message.contains("Unclosed"));
        assert!(message.contains("**"));
        assert!(message.contains("42"));
    }
}
