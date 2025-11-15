# Feature: Core AsciiDoc Parser

## Overview
The Core Parser is the foundation of Doctor A, responsible for transforming raw AsciiDoc text into a validated Abstract Syntax Tree (AST). It uses Logos for high-performance lexical analysis and Chumsky for parser combinators with sophisticated error recovery, enabling multiple syntax errors to be reported in a single pass while maintaining excellent performance.

## Status
- **Status**: Draft
- **Priority**: Critical
- **Effort**: X-Large
- **Target Version**: v0.1.0 (MVP subset), v0.2.0 (full features)
- **Owner**: TBD
- **Created**: 2025-11-14
- **Last Updated**: 2025-11-14

## Business Value
The Core Parser is the essential foundation for all Doctor A functionality. Without it, no document processing can occur. Its quality directly impacts:
- **User Experience**: Error recovery allows document authors to see all syntax issues at once, dramatically improving authoring workflow
- **Performance**: Fast parsing enables real-time preview and batch processing of large documentation sets
- **Correctness**: Validated AST ensures processors receive well-formed input, preventing downstream errors
- **Extensibility**: Clean AST representation enables custom processors to transform documents without re-parsing

## User Stories
- As a **document author**, I want to see all syntax errors in my AsciiDoc file at once, so that I can fix them efficiently without multiple parse attempts
- As a **documentation maintainer**, I want fast parsing of large documentation sets, so that I can generate output formats quickly in CI/CD pipelines
- As a **developer**, I want a well-typed AST, so that I can write custom processors with confidence that the structure is valid
- As a **tool developer**, I want precise error locations with context, so that I can build language servers and editor integrations with excellent diagnostics

## Requirements

### Functional Requirements
1. The system shall tokenize AsciiDoc text into a well-defined set of tokens representing all AsciiDoc syntax elements
2. The system shall parse token streams into a validated Abstract Syntax Tree (AST) conforming to AsciiDoc language specification
3. The system shall support context-sensitive parsing for attributes, conditional directives, and stateful block nesting
4. The system shall detect and report multiple syntax errors with precise line/column locations and helpful context
5. The system shall handle AsciiDoc attribute substitution with multi-stage processing and dependency resolution
6. The system shall support conditional inclusion/exclusion via ifdef/ifndef directives evaluated during parsing
7. The system shall parse nested delimited blocks with dynamic delimiter length requirements
8. The system shall parse complex table structures including nested tables and AsciiDoc cell content
9. The system shall resolve include directives by accessing file system during parsing
10. The system shall produce a strongly-typed AST suitable for multiple output format processors

### Non-Functional Requirements

#### Performance
- Parse 1MB AsciiDoc document in < 50ms (20 MB/s minimum throughput)
- Lexing throughput: 500+ MB/s (target: 1,000+ MB/s with Logos optimization)
- Memory usage: < 10x input size for AST representation
- Zero-copy parsing where possible to minimize allocations
- Streaming support for partial document parsing (future enhancement)

#### Security
- No unsafe code in parser core (maintain Rust memory safety)
- Bounded recursion depth to prevent stack overflow attacks
- Resource limits on include directive depth (prevent infinite include loops)
- Validation of file paths for include directives (prevent path traversal)
- Sanitization of attribute values to prevent injection attacks in processors

#### Scalability
- Handle documents up to 100MB in size without degradation
- Support concurrent parsing of multiple documents (thread-safe design)
- Enable incremental re-parsing for editor integration (future with tree-sitter)

#### Usability
- Error messages include source code context (3 lines before/after)
- Suggestions for common syntax mistakes (e.g., "did you mean `==` instead of `=`?")
- Color-coded error output in terminal (via miette)
- Machine-readable error format for tooling integration (JSON output option)

#### Reliability
- Never panic in library code (all errors returned as Result types)
- Graceful degradation: partial AST generation even with syntax errors
- Comprehensive test coverage: >95% line coverage for parser core
- Fuzzing integration to discover edge cases and potential crashes
- Property-based testing for parser invariants

## Technical Design

### Architecture
The Core Parser is the first layer in Doctor A's three-layer architecture (see `architecture.md`):

```
Input Layer (File I/O)
    ↓
Parser Layer (THIS FEATURE)
    ├─→ Lexer (Logos)
    ├─→ Parser (Chumsky)
    ├─→ AST Builder (integrated with Chumsky)
    └─→ Validator (post-parse pass)
    ↓
AST Layer
    ↓
Processor Layer (HTML, PDF, etc.)
```

The parser is isolated from both input and output concerns, focusing solely on correct transformation from text to AST.

### Components

#### Component 1: Lexer (Logos-based)
- **Responsibility**: Tokenize raw AsciiDoc text into classified tokens with span information
- **Location**: `src/parser/lexer.rs`, `src/parser/tokens.rs`
- **Dependencies**: `logos` (v0.15+)
- **Interfaces**:
  - `Token` enum with Logos derive macro
  - `pub fn lex(input: &str) -> impl Iterator<Item = (Token, Span)>`

#### Component 2: Parser (Chumsky-based)
- **Responsibility**: Build AST from token stream using parser combinators with error recovery
- **Location**: `src/parser/parser.rs`, `src/parser/combinators/`
- **Dependencies**: `chumsky` (v1.0.0-alpha.8+), `Token` enum from lexer
- **Interfaces**:
  - `pub fn parse_document(tokens: &[(Token, Span)]) -> Result<Document, Vec<ParseError>>`
  - Internal combinator functions for each AST node type

#### Component 3: AST Nodes
- **Responsibility**: Strongly-typed representation of parsed document structure
- **Location**: `src/ast/`
  - `src/ast/document.rs` - Root Document node
  - `src/ast/block.rs` - Block-level nodes
  - `src/ast/inline.rs` - Inline-level nodes
  - `src/ast/attributes.rs` - Attribute handling
- **Dependencies**: `serde` (for serialization), `miette` (for span tracking)
- **Interfaces**: Public structs/enums for all AST node types

#### Component 4: Validator
- **Responsibility**: Post-parse semantic validation and constraint checking
- **Location**: `src/parser/validator.rs`
- **Dependencies**: AST types
- **Interfaces**: `pub fn validate(document: &Document) -> Result<(), Vec<ValidationError>>`

#### Component 5: Parser State
- **Responsibility**: Manage context-sensitive state (attributes, conditionals, block stack)
- **Location**: `src/parser/state.rs`
- **Dependencies**: None (pure state management)
- **Interfaces**: Internal to parser, not public API

#### Component 6: Error Reporting
- **Responsibility**: Beautiful diagnostic output with source context
- **Location**: `src/parser/errors.rs`, `src/parser/diagnostics.rs`
- **Dependencies**: `miette` (v7+), `thiserror`
- **Interfaces**:
  - `ParseError` type with `miette::Diagnostic` implementation
  - `pub fn render_errors(errors: &[ParseError], source: &str) -> String`

### Data Model

```rust
// ===== Token Definition (Logos) =====

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t]+")] // Skip inline whitespace
pub enum Token {
    // Document Structure
    #[token("= ")]
    Heading1,
    #[token("== ")]
    Heading2,
    #[token("=== ")]
    Heading3,
    #[token("==== ")]
    Heading4,
    #[token("===== ")]
    Heading5,
    #[token("====== ")]
    Heading6,

    // Block Delimiters
    #[regex(r"-{4,}")]
    ListingDelimiter,
    #[regex(r"={4,}")]
    ExampleDelimiter,
    #[regex(r"\*{4,}")]
    SidebarDelimiter,
    #[regex(r"_{4,}")]
    QuoteDelimiter,
    #[regex(r"\+{4,}")]
    PassthroughDelimiter,
    #[regex(r"\.{4,}")]
    LiteralDelimiter,

    // Table Elements
    #[token("|===")]
    TableDelimiter,
    #[token("!===")]
    NestedTableDelimiter,
    #[token("|")]
    TableCellSeparator,
    #[token("!")]
    NestedTableCellSeparator,

    // Lists
    #[regex(r"\*{1,5} ")]
    UnorderedListMarker,
    #[regex(r"\.{1,5} ")]
    OrderedListMarker,
    #[regex(r".+::")]
    DescriptionListTerm,
    #[token("::")]
    DescriptionListDelimiter,

    // Inline Formatting
    #[token("**")]
    BoldDelimiter,
    #[token("*")]
    Italic,
    #[token("``")]
    MonospaceDelimiter,
    #[token("`")]
    Monospace,
    #[token("__")]
    ItalicDelimiter,
    #[token("_")]
    ItalicSingle,

    // Links and Images
    #[regex(r"https?://[^\s\[\]]+")]
    Url,
    #[token("link:")]
    LinkMacro,
    #[token("image:")]
    ImageMacro,
    #[token("image::")]
    ImageBlockMacro,

    // Attributes
    #[regex(r":[a-zA-Z0-9_-]+:")]
    AttributeRef,
    #[regex(r":[a-zA-Z0-9_-]+: .+")]
    AttributeEntry,
    #[token("{")]
    AttrRefOpen,
    #[token("}")]
    AttrRefClose,

    // Conditional Directives
    #[token("ifdef::")]
    IfDef,
    #[token("ifndef::")]
    IfNDef,
    #[token("ifeval::")]
    IfEval,
    #[token("endif::")]
    EndIf,

    // Include Directive
    #[token("include::")]
    Include,

    // Special
    #[token("\n")]
    Newline,
    #[regex(r"\n\n+")]
    BlankLine,
    #[regex(r"//.*")]
    Comment,

    // Content
    #[regex(r"[^\s\*_`\[\]{}|=\-\.\+:]+")]
    Word,
    #[regex(r"[^\n]+")]
    Line,

    // Brackets
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,

    // Error token
    #[error]
    Error,
}

// ===== AST Types =====

/// Root document node
pub struct Document {
    /// Document-level attributes
    pub attributes: HashMap<String, AttributeValue>,
    /// Optional document header
    pub header: Option<Header>,
    /// Document body (block-level nodes)
    pub body: Vec<Block>,
    /// Source span for error reporting
    pub span: Span,
}

/// Document header
pub struct Header {
    pub title: InlineContent,
    pub author: Option<String>,
    pub revision: Option<String>,
    pub attributes: HashMap<String, AttributeValue>,
    pub span: Span,
}

/// Attribute value types
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    String(String),
    Bool(bool),
    Number(i64),
    Unset,
}

/// Block-level AST nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    /// Section with level, title, and nested content
    Section {
        level: u8,          // 1-6
        title: InlineContent,
        id: Option<String>,
        attributes: BlockAttributes,
        blocks: Vec<Block>,
        span: Span,
    },

    /// Paragraph
    Paragraph {
        content: InlineContent,
        attributes: BlockAttributes,
        span: Span,
    },

    /// Delimited block
    Delimited {
        kind: DelimiterKind,
        content: BlockContent,
        attributes: BlockAttributes,
        span: Span,
    },

    /// List (unordered, ordered, description)
    List {
        kind: ListKind,
        items: Vec<ListItem>,
        attributes: BlockAttributes,
        span: Span,
    },

    /// Table
    Table {
        header: Option<TableRow>,
        rows: Vec<TableRow>,
        attributes: BlockAttributes,
        span: Span,
    },

    /// Block macro (image, video, etc.)
    BlockMacro {
        name: String,
        target: String,
        attributes: BlockAttributes,
        span: Span,
    },
}

/// Delimiter kinds for delimited blocks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelimiterKind {
    Listing,    // ----
    Example,    // ====
    Sidebar,    // ****
    Quote,      // ____
    Passthrough, // ++++
    Literal,    // ....
    Open,       // --
}

/// Content inside delimited blocks
#[derive(Debug, Clone, PartialEq)]
pub enum BlockContent {
    /// Raw text (for literal/listing blocks)
    Raw(String),
    /// Nested blocks (for example/sidebar blocks)
    Blocks(Vec<Block>),
    /// AsciiDoc content (for open blocks)
    AsciiDoc(Vec<Block>),
}

/// List kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListKind {
    Unordered { level: u8 },
    Ordered { level: u8, style: OrderedStyle },
    Description,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderedStyle {
    Arabic,      // 1, 2, 3
    LowerAlpha,  // a, b, c
    UpperAlpha,  // A, B, C
    LowerRoman,  // i, ii, iii
    UpperRoman,  // I, II, III
}

/// List item
#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub marker: String,
    pub content: Vec<Block>,
    pub span: Span,
}

/// Table row
#[derive(Debug, Clone, PartialEq)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub span: Span,
}

/// Table cell
#[derive(Debug, Clone, PartialEq)]
pub struct TableCell {
    pub content: CellContent,
    pub colspan: u8,
    pub rowspan: u8,
    pub style: Option<CellStyle>,
    pub span: Span,
}

/// Cell content types
#[derive(Debug, Clone, PartialEq)]
pub enum CellContent {
    /// Simple text content
    Inline(InlineContent),
    /// AsciiDoc content (cell style 'a')
    AsciiDoc(Vec<Block>),
}

/// Cell styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellStyle {
    AsciiDoc,
    Emphasis,
    Header,
    Literal,
    Monospace,
    Strong,
}

/// Block-level attributes
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlockAttributes {
    pub id: Option<String>,
    pub roles: Vec<String>,
    pub options: Vec<String>,
    pub custom: HashMap<String, String>,
}

/// Inline content (sequence of inline nodes)
pub type InlineContent = Vec<InlineNode>;

/// Inline-level AST nodes
#[derive(Debug, Clone, PartialEq)]
pub enum InlineNode {
    /// Plain text
    Text(String),

    /// Formatted text
    Formatted {
        style: FormatStyle,
        content: InlineContent,
        span: Span,
    },

    /// Inline macro
    Macro {
        name: String,
        target: String,
        attributes: Vec<String>,
        span: Span,
    },

    /// Link
    Link {
        url: String,
        text: Option<InlineContent>,
        span: Span,
    },

    /// Attribute reference
    AttributeRef {
        name: String,
        span: Span,
    },

    /// Line break
    LineBreak { span: Span },
}

/// Inline formatting styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatStyle {
    Bold,
    Italic,
    Monospace,
    Subscript,
    Superscript,
    Mark,
}

/// Span for source location tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn union(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}
```

### API Design

```rust
// ===== Public Parser API =====

use std::path::Path;
use crate::ast::Document;
use crate::parser::errors::ParseError;

/// Main parser entry point
pub struct Parser {
    config: ParserConfig,
}

impl Parser {
    /// Create parser with default configuration
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
        }
    }

    /// Create parser with custom configuration
    pub fn with_config(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Parse AsciiDoc text into AST
    pub fn parse(&self, input: &str) -> ParseResult {
        // 1. Lex input into tokens
        let tokens: Vec<_> = lex(input).collect();

        // 2. Parse tokens into AST (with error recovery)
        let (document, errors) = parse_document(&tokens, &self.config);

        // 3. Validate AST
        let validation_errors = validate(&document);

        // 4. Combine all errors
        let all_errors: Vec<_> = errors.into_iter()
            .chain(validation_errors)
            .collect();

        ParseResult {
            document: Some(document),
            errors: all_errors,
        }
    }

    /// Parse AsciiDoc file
    pub fn parse_file(&self, path: &Path) -> Result<ParseResult, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        Ok(self.parse(&content))
    }
}

/// Parser configuration
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Maximum include depth
    pub max_include_depth: usize,
    /// Maximum section nesting
    pub max_section_depth: u8,
    /// Base directory for resolving includes
    pub base_dir: Option<PathBuf>,
    /// Enable safe mode (restrict includes)
    pub safe_mode: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            max_include_depth: 10,
            max_section_depth: 6,
            base_dir: None,
            safe_mode: false,
        }
    }
}

/// Parse result with optional AST and errors
#[derive(Debug)]
pub struct ParseResult {
    /// Parsed document (may be partial if errors occurred)
    pub document: Option<Document>,
    /// Parse and validation errors
    pub errors: Vec<ParseError>,
}

impl ParseResult {
    /// Check if parsing succeeded without errors
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get document, failing if there were errors
    pub fn into_result(self) -> Result<Document, Vec<ParseError>> {
        if self.errors.is_empty() {
            Ok(self.document.expect("document should exist if no errors"))
        } else {
            Err(self.errors)
        }
    }
}

/// Lexer function
pub fn lex(input: &str) -> impl Iterator<Item = (Token, Span)> {
    // Logos lexer implementation
}

// ===== Error Types =====

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ParseError {
    #[error("unexpected token {found:?}, expected {expected}")]
    #[diagnostic(code(parser::unexpected_token))]
    UnexpectedToken {
        #[label("unexpected token here")]
        span: Span,
        found: Token,
        expected: String,
    },

    #[error("unclosed delimiter {delimiter:?}")]
    #[diagnostic(code(parser::unclosed_delimiter))]
    UnclosedDelimiter {
        #[label("delimiter opened here")]
        open_span: Span,
        #[label("expected closing delimiter here")]
        close_span: Option<Span>,
        delimiter: DelimiterKind,
    },

    #[error("invalid nesting: {message}")]
    #[diagnostic(code(parser::invalid_nesting))]
    InvalidNesting {
        #[label("invalid nesting here")]
        span: Span,
        message: String,
    },

    #[error("include depth exceeded (max: {max_depth})")]
    #[diagnostic(code(parser::include_depth_exceeded))]
    IncludeDepthExceeded {
        #[label("include directive here")]
        span: Span,
        max_depth: usize,
    },

    #[error("include file not found: {path}")]
    #[diagnostic(code(parser::include_not_found))]
    IncludeNotFound {
        #[label("include directive here")]
        span: Span,
        path: String,
    },

    #[error("undefined attribute: {name}")]
    #[diagnostic(code(parser::undefined_attribute))]
    UndefinedAttribute {
        #[label("attribute reference here")]
        span: Span,
        name: String,
    },
}
```

### Processing Flow

**High-Level Flow**:
1. **Input**: Raw AsciiDoc text string
2. **Lexing**: Logos tokenizes text → stream of `(Token, Span)`
3. **Parsing**: Chumsky combinators build AST from tokens (with error recovery)
4. **Validation**: Post-parse semantic checks
5. **Output**: `ParseResult` with AST and any errors

**Detailed Parsing Flow**:

```
Raw AsciiDoc Text
    ↓
┌─────────────────────────────────────┐
│  Lexer (Logos)                      │
│  - Pattern matching via DFA         │
│  - Token classification             │
│  - Span tracking                    │
└─────────────────┬───────────────────┘
                  ↓
    Token Stream: Vec<(Token, Span)>
                  ↓
┌─────────────────────────────────────┐
│  Parser (Chumsky Combinators)      │
│  ┌───────────────────────────────┐ │
│  │ Document Parser               │ │
│  │  ├─→ Header Parser            │ │
│  │  ├─→ Attribute Parser         │ │
│  │  └─→ Block Parser (recursive) │ │
│  │      ├─→ Section Parser       │ │
│  │      ├─→ Paragraph Parser     │ │
│  │      ├─→ List Parser          │ │
│  │      ├─→ Table Parser         │ │
│  │      ├─→ Delimited Block      │ │
│  │      └─→ Inline Parser        │ │
│  │           ├─→ Text            │ │
│  │           ├─→ Formatting      │ │
│  │           ├─→ Links/Macros    │ │
│  │           └─→ Attr Refs       │ │
│  └───────────────────────────────┘ │
│                                     │
│  Error Recovery:                    │
│  - Synchronization at block bounds  │
│  - Partial AST generation           │
│  - Multiple error collection        │
└─────────────────┬───────────────────┘
                  ↓
      AST + Parse Errors
                  ↓
┌─────────────────────────────────────┐
│  Validator                          │
│  - Attribute reference validation   │
│  - Section level consistency        │
│  - Nesting depth checks             │
│  - Include path validation          │
└─────────────────┬───────────────────┘
                  ↓
    ParseResult { document, errors }
```

**State Management During Parsing**:

```rust
// Parser state (maintained during parsing)
struct ParserState {
    // Document-level attributes
    attributes: HashMap<String, AttributeValue>,

    // Conditional directive stack
    conditionals: Vec<ConditionalState>,

    // Delimited block stack (for nesting validation)
    block_stack: Vec<BlockContext>,

    // Current section level
    section_level: u8,

    // Include depth counter
    include_depth: usize,
}
```

### Error Handling

**Error Recovery Strategy**:
- **Synchronization Points**: Blank lines, section headings, block delimiters
- **Panic Mode**: Skip tokens until reaching synchronization point
- **Error Production**: Insert placeholder AST nodes to enable continued parsing
- **Multiple Errors**: Collect all errors, don't stop at first failure

**Error Types and Handling**:

1. **Lexer Errors** (Token::Error)
   - **Detection**: Logos encounters unrecognized character sequence
   - **Handling**: Skip character, emit error token, continue lexing
   - **Recovery**: Sync at next whitespace or newline

2. **Unexpected Token**
   - **Detection**: Parser expected token A, found token B
   - **Handling**: Emit error, attempt to skip to recovery point
   - **Recovery**: Sync at blank line or block delimiter

3. **Unclosed Delimiter**
   - **Detection**: End of input with open delimiter on block stack
   - **Handling**: Emit error pointing to opening delimiter, close implicitly
   - **Recovery**: Generate partial AST node for unclosed block

4. **Invalid Nesting**
   - **Detection**: Delimited block nesting violates AsciiDoc rules
   - **Handling**: Emit error, insert implicit close for outer block
   - **Recovery**: Continue parsing with corrected nesting

5. **Include Errors**
   - **Detection**: File not found, path traversal, depth exceeded
   - **Handling**: Emit error, replace include with placeholder
   - **Recovery**: Continue parsing parent document

6. **Attribute Errors**
   - **Detection**: Undefined attribute reference, circular dependency
   - **Handling**: Emit warning, substitute with empty string or attribute name
   - **Recovery**: Continue parsing with substitution

**Error Reporting with Miette**:

```rust
// Example error output
Error: unexpected token Word("content"), expected TableCellSeparator
  ┌─ document.adoc:15:3
  │
15 │   content without delimiter
  │   ^^^^^^^ unexpected token here
  │
  = note: table cells must be separated by '|' or '!'
  = help: did you forget the cell separator?

Error: unclosed delimiter Listing
  ┌─ document.adoc:8:1
  │
 8 │ ----
  │ ^^^^ delimiter opened here
  │
20 │ == Next Section
  │ -- expected closing delimiter here
  │
  = note: listing blocks must be closed with '----'
```

## Dependencies

### Depends On
- **logos** (v0.15+): Lexer generator for tokenization
- **chumsky** (v1.0.0-alpha.8+): Parser combinator library with error recovery
- **miette** (v7+): Diagnostic reporting with source context
- **thiserror** (v2+): Error type derivation

### Blocks
- **HTML Processor**: Cannot convert to HTML without AST
- **PDF Processor**: Cannot convert to PDF without AST
- **All Processors**: All output format processors depend on parsed AST
- **CLI**: Command-line interface needs parser to process files
- **Language Server**: LSP implementation needs parser for diagnostics

### Related Features
- **Preprocessor** (future): Text transformations before parsing
- **Tree-sitter Grammar** (future): Incremental parsing for editors
- **AST Serialization** (future): JSON/MessagePack export for tooling
- **Parser Extensions** (future): Custom syntax via parser plugins

## Implementation Plan

### Phase 1: Proof of Concept (1 week)
- [ ] Task 1.1: Set up Rust project with logos and chumsky dependencies
- [ ] Task 1.2: Define minimal Token enum (headings, paragraphs, bold, italic)
- [ ] Task 1.3: Implement basic Logos lexer for minimal tokens
- [ ] Task 1.4: Create simple Chumsky parser for subset (document → sections → paragraphs)
- [ ] Task 1.5: Parse example AsciiDoc files, validate approach
- [ ] Task 1.6: Benchmark POC: compare Logos+Chumsky vs Logos+Winnow
- [ ] Task 1.7: Decision point: confirm Chumsky or switch to Winnow

### Phase 2: Lexer Implementation (1 week)
- [ ] Task 2.1: Define complete Token enum for all AsciiDoc syntax
- [ ] Task 2.2: Implement Logos patterns for all tokens
- [ ] Task 2.3: Add span tracking to lexer output
- [ ] Task 2.4: Write lexer unit tests (>95% coverage)
- [ ] Task 2.5: Benchmark lexer performance (target: 500+ MB/s)
- [ ] Task 2.6: Add error token handling

### Phase 3: Core AST Types (3 days)
- [ ] Task 3.1: Define Document, Header, Block, InlineNode types
- [ ] Task 3.2: Add Span to all AST nodes
- [ ] Task 3.3: Implement Default, Debug, Clone for AST types
- [ ] Task 3.4: Add serde Serialize/Deserialize for AST
- [ ] Task 3.5: Write AST construction tests

### Phase 4: Block Parser (2 weeks)
- [ ] Task 4.1: Document parser (header + body)
- [ ] Task 4.2: Section parser (with nesting)
- [ ] Task 4.3: Paragraph parser
- [ ] Task 4.4: Delimited block parser (all types)
- [ ] Task 4.5: List parser (unordered, ordered, description)
- [ ] Task 4.6: Table parser (basic tables)
- [ ] Task 4.7: Table parser (nested tables, cell styles)
- [ ] Task 4.8: Block macro parser
- [ ] Task 4.9: Block parser integration tests

### Phase 5: Inline Parser (1 week)
- [ ] Task 5.1: Text and whitespace handling
- [ ] Task 5.2: Bold, italic, monospace formatting
- [ ] Task 5.3: Links (inline and macro)
- [ ] Task 5.4: Inline macros (image, etc.)
- [ ] Task 5.5: Attribute references
- [ ] Task 5.6: Subscript, superscript, mark
- [ ] Task 5.7: Inline parser tests

### Phase 6: Context-Sensitive Features (1 week)
- [ ] Task 6.1: Attribute definition and substitution
- [ ] Task 6.2: Conditional directives (ifdef/ifndef/ifeval)
- [ ] Task 6.3: Include directive processing
- [ ] Task 6.4: Parser state management
- [ ] Task 6.5: Context-sensitive tests

### Phase 7: Error Recovery (1 week)
- [ ] Task 7.1: Define ParseError types with miette integration
- [ ] Task 7.2: Implement synchronization strategy
- [ ] Task 7.3: Add error recovery to block parser
- [ ] Task 7.4: Add error recovery to inline parser
- [ ] Task 7.5: Multiple error collection
- [ ] Task 7.6: Error message quality tests

### Phase 8: Validation (3 days)
- [ ] Task 8.1: Implement post-parse validator
- [ ] Task 8.2: Validate attribute references
- [ ] Task 8.3: Validate section nesting
- [ ] Task 8.4: Validate include paths
- [ ] Task 8.5: Validator tests

### Phase 9: Public API (2 days)
- [ ] Task 9.1: Design Parser public API
- [ ] Task 9.2: Implement ParserConfig
- [ ] Task 9.3: Implement ParseResult
- [ ] Task 9.4: Add convenience methods (parse_file, etc.)
- [ ] Task 9.5: API documentation with examples

### Phase 10: Performance Optimization (1 week)
- [ ] Task 10.1: Benchmark full parser on real AsciiDoc documents
- [ ] Task 10.2: Profile hot paths, optimize allocations
- [ ] Task 10.3: Add zero-copy optimizations where possible
- [ ] Task 10.4: Streaming parser (if needed)
- [ ] Task 10.5: Final performance tests (must meet requirements)

### Phase 11: Testing & Quality (1 week)
- [ ] Task 11.1: Achieve >95% test coverage
- [ ] Task 11.2: Add property-based tests (proptest)
- [ ] Task 11.3: Set up fuzzing with cargo-fuzz
- [ ] Task 11.4: Compatibility tests against asciidoctor
- [ ] Task 11.5: Edge case tests from research
- [ ] Task 11.6: Documentation with rustdoc

### Testing Strategy

#### Unit Tests
- Test each lexer token pattern with positive/negative cases
- Test each parser combinator in isolation
- Test AST node construction and validation
- Test error recovery for each error type
- Test state management (attributes, conditionals, includes)

#### Integration Tests
- Parse complete AsciiDoc documents and validate AST structure
- Test error reporting with multiple errors in document
- Test include directive resolution with nested includes
- Test attribute substitution with complex dependencies
- Test conditional directive evaluation

#### Edge Cases to Test
- [ ] Empty document
- [ ] Document with only whitespace
- [ ] Maximum nesting depth (sections, lists, includes)
- [ ] Delimited blocks with same delimiter at different lengths
- [ ] Unclosed delimiters at end of file
- [ ] Attribute references to undefined attributes
- [ ] Circular include dependencies
- [ ] Include path traversal attempts
- [ ] Very large documents (100MB+)
- [ ] Unicode content (emoji, RTL text, etc.)
- [ ] All AsciiDoc test cases from asciidoctor test suite

## Edge Cases and Considerations

### Edge Case 1: Delimiter Length Ambiguity
**Scenario**: Nested delimited blocks with same delimiter type require different lengths:
```asciidoc
****
outer sidebar
*****
inner sidebar
*****
****
```
**Handling**: Track delimiter length on block stack, require inner delimiter to have different length (longer or shorter). Emit error if lengths match.

### Edge Case 2: Attribute Circular Dependencies
**Scenario**: Attributes reference each other in a cycle:
```asciidoc
:foo: {bar}
:bar: {baz}
:baz: {foo}
```
**Handling**: Detect cycles during attribute substitution using visited set. Emit error and break cycle by substituting with attribute name. Maximum substitution depth limit (e.g., 10).

### Edge Case 3: Include Directive in Delimited Block
**Scenario**: Include directive inside listing block should be literal text:
```asciidoc
----
include::example.adoc[]
----
```
**Handling**: Context-sensitive parsing. Inside literal/listing blocks, treat include as plain text. Only process includes at document/block level.

### Edge Case 4: Table Cell with Delimited Block
**Scenario**: Table cell with AsciiDoc style (`a`) contains delimited block:
```asciidoc
|===
a|
****
sidebar in cell
****
|===
```
**Handling**: Parse cell content recursively with full block parser. Track table nesting level to handle nested tables with different delimiters (`!===`).

### Edge Case 5: Unclosed Delimiter at EOF
**Scenario**: Document ends while delimited block is open:
```asciidoc
----
some code
[EOF]
```
**Handling**: Error recovery emits "unclosed delimiter" error pointing to opening delimiter. Generate partial AST with implicit close. Allow parser to continue for subsequent blocks.

### Edge Case 6: Malformed Attribute Reference
**Scenario**: Unclosed attribute reference:
```asciidoc
{attribute
```
**Handling**: Lex as `AttrRefOpen` + `Word("attribute")`. Parser emits error "expected `}` to close attribute reference". Treat as literal text in recovery.

## Alternatives Considered

### Alternative 1: Pest PEG Parser Generator
**Description**: Use Pest to define AsciiDoc grammar in declarative `.pest` file.

**Pros**:
- Declarative grammar is readable and maintainable
- Good error messages from Pest
- Strong ecosystem and documentation

**Cons**:
- **14x slower** than Chumsky (1,971 µs vs 210 µs for JSON parsing)
- PEG cannot handle context-sensitive parsing well
- No left recursion support
- AsciiDoc PEG experiment abandoned with 30+ unmapped features
- Too slow for large documents (100MB at 57 MB/s = 1.75 seconds)

**Why Not Chosen**: Performance unacceptable for production use. Context-sensitivity is fundamental blocker.

### Alternative 2: Logos + Winnow
**Description**: Use Winnow parser combinators instead of Chumsky.

**Pros**:
- **18% faster parsing** (627 MB/s vs 533 MB/s)
- More mature than Chumsky (winnow evolved from nom)
- Larger ecosystem and community
- Stateful parsing support

**Cons**:
- Less sophisticated error recovery than Chumsky
- Error messages not as good (though better than nom)
- More manual work for multiple error reporting

**Why Not Chosen**: Error recovery and user experience prioritized over raw performance. Chumsky's 533 MB/s is still excellent (meets our 20 MB/s requirement with 26x headroom). Can swap to Winnow if benchmarks show bottleneck.

**Status**: Remains viable alternative if performance becomes issue.

### Alternative 3: Hand-Written Recursive Descent Parser
**Description**: Write parser from scratch without library, using Logos for lexing.

**Pros**:
- Maximum control over parsing logic and performance
- Proven approach (all existing Rust AsciiDoc parsers use this)
- Can achieve performance equal to Logos+Winnow
- No dependency on parser library maintenance

**Cons**:
- Much more code to write and maintain (~3-5x more LOC)
- Error recovery must be implemented manually
- Longer development time (2-3 weeks → 6-8 weeks estimate)
- More potential for bugs without library guarantees

**Why Not Chosen**: Development effort too high for MVP. Chumsky provides error recovery out of the box. Can migrate to hand-written later if needed (Logos lexer is reusable).

**Status**: Future option if library approach proves insufficient.

### Alternative 4: tree-sitter
**Description**: Use tree-sitter incremental parser generator.

**Pros**:
- Incremental parsing (perfect for editor integration)
- Robust error recovery
- Multi-language support (code blocks)
- Excellent for LSP

**Cons**:
- Complex setup
- Optimized for editors, not batch conversion
- Requires JavaScript grammar definition
- Overkill for doctora's batch processing use case

**Why Not Chosen**: Not suited for primary parser. Should be used for LSP/editor integration after core parser is working.

**Status**: Future consideration for separate tree-sitter-asciidoc project.

### Alternative 5: Nom Parser Combinators
**Description**: Use nom (mature parser combinator library).

**Pros**:
- Extremely mature (10k+ stars)
- Massive ecosystem (20k+ dependent crates)
- Proven in production (TLS, DNS parsers)
- Extensive documentation and tutorials

**Cons**:
- **Winnow is nom's successor** with better errors
- Slower than Winnow (213 MB/s vs 627 MB/s)
- Poorer error messages than both Winnow and Chumsky
- Less modern API design

**Why Not Chosen**: Winnow supersedes nom for new projects. If choosing this approach, use Winnow instead.

## Research References

### AsciiDoc Specification
- Eclipse AsciiDoc Language Project: https://gitlab.eclipse.org/eclipse/asciidoc-lang/asciidoc-lang
- AsciiDoc Syntax Reference: https://docs.asciidoctor.org/asciidoc/latest/syntax-quick-reference/
- Asciidoctor Documentation: https://docs.asciidoctor.org/asciidoctor/latest/
- AsciiDoc Parsing Lab (PEG Experiment): https://github.com/opendevise/asciidoc-parsing-lab

### Existing Implementations
- Asciidoctor Parser (Ruby): https://github.com/asciidoctor/asciidoctor/blob/main/lib/asciidoctor/parser.rb
- asciidoc-parser (Rust): https://github.com/scouten/asciidoc-parser
- asciidocr (Rust): https://github.com/asciidocr/asciidocr
- asciidork (Rust): https://github.com/jaredh159/asciidork

### Rust Parsing Libraries
- Logos Documentation: https://logos.maciej.codes/
- Logos GitHub: https://github.com/maciejhirsz/logos
- Chumsky Guide: https://github.com/zesterer/chumsky/blob/main/guide.md
- Chumsky Examples: https://github.com/zesterer/chumsky/tree/main/examples
- Winnow Documentation: https://docs.rs/winnow/latest/winnow/
- Nom Parser Combinators: https://github.com/rust-bakery/nom

### Error Reporting
- miette Documentation: https://docs.rs/miette/latest/miette/
- ariadne (alternative): https://github.com/zesterer/ariadne
- Rust Error Handling Book: https://doc.rust-lang.org/book/ch09-00-error-handling.html

### Parser Theory & Techniques
- PEG Left Recursion: https://arxiv.org/abs/1207.0443
- Parser Combinators and Error Recovery: https://blog.jsbarretto.com/blog/parser-combinators-and-error-recovery/
- Building Parsers in Rust: https://blog.logrocket.com/building-rust-parser-pest-peg/

### Performance Benchmarks
- Parser Combinator Benchmarks: https://github.com/rosetta-rs/parse-rosetta-rs
- JSON Parser Comparison: (included in research data)

## Open Questions
- [x] ✅ Which parser library to use? → **Decided: Logos + Chumsky** (see ADR-004)
- [ ] Should we support Markdown compatibility mode from day one, or add it later?
- [ ] What is the optimal error recovery strategy for table parsing?
- [ ] Should attribute substitution happen during parsing or as post-parse transformation?
- [ ] How much of the AsciiDoc spec should be in v0.1.0 MVP vs v0.2.0?
- [ ] Should we prioritize compatibility with asciidoctor or define our own behavior for ambiguous cases?

## Change Log

### 2025-11-14
- Created initial feature specification
- Documented Logos + Chumsky decision from ADR-004
- Defined complete token set and AST types
- Outlined 11-phase implementation plan
- Added comprehensive error handling strategy
- Documented edge cases and alternatives

---

## Notes

**Context-Sensitivity Challenges**: AsciiDoc's context-sensitive nature (attributes, conditionals, stateful blocks) makes it significantly more complex than context-free languages. The parser state must track:
- Active attributes and their values
- Conditional directive stack (ifdef/ifndef nesting)
- Delimited block stack (for nesting validation)
- Current section level
- Include depth

**Error Recovery Philosophy**: Prioritize reporting ALL errors in a document, not just the first. This dramatically improves author experience. Use synchronization at block boundaries (blank lines, section headings) as natural recovery points.

**Performance Headroom**: Target is 20 MB/s minimum, Chumsky achieves 533 MB/s (26x headroom). This allows for:
- Less-optimized first implementation
- Feature additions without performance regression
- Complex error recovery without speed concerns

**Migration Strategy**: Logos tokens are library-agnostic. If Chumsky proves inadequate:
1. Swap Chumsky → Winnow (easy, similar APIs)
2. Swap to hand-written parser (moderate effort, keep Logos)
3. Both options maintain lexer investment

**Compatibility**: Aim for asciidoctor compatibility but don't be dogmatic. Where AsciiDoc behavior is ambiguous or problematic, document our decision and rationale. Build comprehensive compatibility test suite against asciidoctor output.
