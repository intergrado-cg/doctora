# Doctor A (doctora) - System Architecture

**Last Updated**: 2024-11-14
**Version**: 0.1.0-alpha
**Status**: Initial Design

---

## Table of Contents
1. [System Overview](#system-overview)
2. [Architecture Style](#architecture-style)
3. [Core Components](#core-components)
4. [Data Flow](#data-flow)
5. [Extension Points](#extension-points)
6. [Technology Stack](#technology-stack)
7. [Design Principles](#design-principles)
8. [Architectural Decisions](#architectural-decisions)

---

## System Overview

Doctor A (doctora) is a modular AsciiDoc parser and processor written in Rust, designed as an alternative to asciidoctor with a focus on modularity, extensibility, and performance.

### Key Characteristics
- **Language**: Rust (for performance, safety, and modern tooling)
- **Input**: AsciiDoc text format
- **Output**: Various formats via pluggable processors
- **Philosophy**: Separation of parsing and processing

### High-Level Architecture
```
┌─────────────────────────────────────────────────────────┐
│                    AsciiDoc Input                       │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│                   Core Parser                           │
│  • Lexer/Tokenizer                                      │
│  • Parser (syntax analysis)                             │
│  • AST Builder                                          │
│  • Validator                                            │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│              Abstract Syntax Tree (AST)                 │
│  • Document structure                                   │
│  • Metadata                                             │
│  • Content nodes                                        │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│               Processor System                          │
│  • Processor Registry                                   │
│  • Processor Chain/Tree                                 │
│  • Format-specific Processors                           │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│                Output (PDF, HTML, etc.)                 │
└─────────────────────────────────────────────────────────┘
```

---

## Architecture Style

**Primary Style**: **Layered Architecture with Plugin System**

### Layers
1. **Input Layer**: File I/O, text preprocessing
2. **Parser Layer**: Lexical analysis, syntactic analysis, AST construction
3. **Validation Layer**: Semantic validation, constraint checking
4. **Processing Layer**: Format conversion, transformation
5. **Output Layer**: File writing, format-specific output

### Plugin Pattern
The processor system uses a plugin architecture:
- Processors are loaded dynamically or registered statically
- Each processor implements a common trait/interface
- Processors can invoke other processors (tree structure)
- Registry manages available processors

---

## Core Components

### 1. Core Parser

**Responsibility**: Parse AsciiDoc text and produce validated AST

**Sub-components**:

#### Lexer/Tokenizer
- **Purpose**: Break input text into tokens
- **Input**: Raw AsciiDoc text
- **Output**: Stream of tokens
- **Technology**: TBD (pest PEG grammar, nom combinators, or hand-written)

#### Parser
- **Purpose**: Build AST from token stream
- **Input**: Token stream
- **Output**: Raw AST
- **Approach**: TBD (recursive descent, parser combinators)

#### AST Builder
- **Purpose**: Construct typed AST nodes
- **Input**: Parse tree
- **Output**: Typed AST
- **Responsibilities**: Type safety, node construction

#### Validator
- **Purpose**: Validate AST for correctness
- **Input**: AST
- **Output**: Validated AST or errors
- **Checks**: Syntax correctness, semantic rules, constraints

**Public API**:
```rust
pub struct Parser {
    // Configuration, options
}

impl Parser {
    pub fn new() -> Self;
    pub fn parse(&self, input: &str) -> Result<Document, ParseError>;
    pub fn parse_file(&self, path: &Path) -> Result<Document, ParseError>;
}
```

### 2. Abstract Syntax Tree (AST)

**Responsibility**: Represent parsed document structure

**Node Types**:
- **Document**: Root node, contains metadata and body
- **Block Nodes**: Sections, paragraphs, lists, tables, code blocks
- **Inline Nodes**: Text, emphasis, links, images, inline code
- **Metadata**: Attributes, properties, configuration

**Design**:
```rust
pub struct Document {
    pub metadata: HashMap<String, String>,
    pub nodes: Vec<Node>,
}

pub enum Node {
    Block(BlockNode),
    Inline(InlineNode),
}

pub enum BlockNode {
    Section { level: u8, title: String, nodes: Vec<Node> },
    Paragraph { nodes: Vec<Node> },
    List { type: ListType, items: Vec<ListItem> },
    CodeBlock { language: Option<String>, code: String },
    Table { /* ... */ },
    // ... more block types
}

pub enum InlineNode {
    Text(String),
    Emphasis { style: EmphasisStyle, content: Vec<InlineNode> },
    Link { url: String, text: Option<String> },
    Image { path: String, alt: Option<String> },
    // ... more inline types
}
```

### 3. Processor System

**Responsibility**: Transform validated AST to output formats

**Components**:

#### Processor Trait
```rust
pub trait Processor {
    /// Processor name/identifier
    fn name(&self) -> &str;

    /// Check if this processor can handle the output format
    fn can_handle(&self, format: &str) -> bool;

    /// Process the document
    fn process(&self, document: &Document, context: &ProcessorContext)
        -> Result<Output, ProcessorError>;
}
```

#### Processor Registry
- **Purpose**: Register and discover processors
- **Responsibilities**:
  - Register processors
  - Find processor for format
  - Manage processor lifecycle

```rust
pub struct ProcessorRegistry {
    processors: HashMap<String, Box<dyn Processor>>,
}

impl ProcessorRegistry {
    pub fn register(&mut self, processor: Box<dyn Processor>);
    pub fn get(&self, format: &str) -> Option<&dyn Processor>;
}
```

#### Processor Context
- **Purpose**: Provide context and services to processors
- **Capabilities**:
  - Access to other processors (for sub-processing)
  - Configuration
  - Resource management

```rust
pub struct ProcessorContext {
    registry: Arc<ProcessorRegistry>,
    config: Config,
    // ... other shared resources
}
```

### 4. Built-in Processors

Initial processors to implement:

#### Validation Processor
- **Format**: None (internal validation)
- **Purpose**: Validate AST correctness
- **Output**: Validation report

#### HTML Processor
- **Format**: "html"
- **Purpose**: Convert to HTML
- **Output**: HTML string

#### Markdown Processor
- **Format**: "markdown"
- **Purpose**: Convert to Markdown
- **Output**: Markdown string

Future processors:
- PDF Processor (using external library)
- Image Processor (for diagrams)
- Custom format processors

---

## Data Flow

### Parse Flow
```
AsciiDoc Text
    ↓
Lexer → Tokens
    ↓
Parser → Parse Tree
    ↓
AST Builder → Raw AST
    ↓
Validator → Validated AST
```

### Process Flow
```
Validated AST
    ↓
Processor Registry → Find Processor
    ↓
Processor → Transform
    ↓
    ├─→ [Invoke Sub-Processor?]
    │       ↓
    │   Sub-Processor → Partial Output
    │       ↓
    └─→ Combine Outputs
    ↓
Final Output
```

### Example: PDF with Images
```
AST (with image nodes)
    ↓
PDF Processor
    ├─→ For each image node:
    │   ├─→ Invoke Image Processor
    │   │       ↓
    │   │   Process image, return image data
    │   └─→ Embed image in PDF
    └─→ Continue PDF generation
    ↓
PDF Output
```

---

## Extension Points

### 1. Custom Processors
Users can create custom processors:
- Implement `Processor` trait
- Register with registry
- Access full AST and context

### 2. Custom AST Nodes
Future: Allow custom node types via attributes

### 3. Configuration
Processors can accept configuration:
- Global configuration
- Per-processor configuration
- Document-level attributes

---

## Technology Stack

### Language & Core
- **Language**: Rust (stable)
- **Edition**: 2021
- **Minimum Version**: TBD (likely 1.70+)

### Parsing Libraries (To Be Decided)
Options under consideration:
- **pest**: PEG parser generator, declarative grammar
- **nom**: Parser combinator library, flexible
- **chumsky**: Error-recovery parser combinators
- **lalrpop**: LR parser generator
- **Hand-written**: Recursive descent parser

### Other Dependencies
- **Error Handling**: `thiserror`, `anyhow`
- **CLI**: `clap` (for command-line interface)
- **Serialization**: `serde` (for config, intermediate formats)
- **Testing**: Built-in Rust testing, `proptest` for property testing

---

## Design Principles

### 1. Separation of Concerns
- Parser only parses, doesn't process
- Processors only transform, don't parse
- Clear module boundaries

### 2. Modularity
- Independent components
- Well-defined interfaces
- Minimal coupling

### 3. Extensibility
- Plugin system for processors
- Easy to add new output formats
- Configuration-driven behavior

### 4. Performance
- Zero-copy where possible
- Efficient AST representation
- Lazy evaluation where appropriate

### 5. Rust Idioms
- Use traits for polymorphism
- Leverage type system for correctness
- Follow Rust API guidelines
- Prefer composition over inheritance

### 6. Error Handling
- Comprehensive error types
- Helpful error messages
- Recovery where possible
- No panics in library code

### 7. Testability
- Unit tests for components
- Integration tests for workflows
- Property-based testing for parser
- Fuzzing for robustness

---

## Architectural Decisions

### ADR-001: Rust as Implementation Language
**Status**: Accepted
**Date**: 2024-11-14

**Context**: Need to choose implementation language for doctora.

**Decision**: Use Rust.

**Rationale**:
- Performance: Comparable to C/C++, faster than Ruby/Python
- Safety: Memory safety without GC overhead
- Modern tooling: cargo, rustfmt, clippy
- Type system: Helps catch errors at compile time
- Ecosystem: Growing set of parsing libraries
- Concurrency: Safe concurrency for potential parallel processing

**Consequences**:
- Pros: Fast, safe, modern
- Cons: Learning curve, compile times, smaller ecosystem than some languages

**Alternatives Considered**:
- Ruby (like asciidoctor): Easier, slower
- Python: Easier, slower
- Go: Simpler, less powerful type system
- C/C++: Fast but unsafe

---

### ADR-002: Separate Parser and Processor
**Status**: Accepted
**Date**: 2024-11-14

**Context**: How to structure the transformation from AsciiDoc to output formats.

**Decision**: Separate parsing (→ AST) from processing (AST → output).

**Rationale**:
- Separation of concerns
- Reusable AST for multiple output formats
- Easier testing (test parser separately from processors)
- Enables processor composition

**Consequences**:
- Pros: Clean architecture, testable, extensible
- Cons: Additional layer, potential performance overhead (mitigated by Rust performance)

**Alternatives Considered**:
- Direct transformation: Simpler but less flexible
- Stream processing: Good for large docs, harder to implement

---

### ADR-003: Plugin-based Processor System
**Status**: Accepted
**Date**: 2024-11-14

**Context**: How to support multiple output formats.

**Decision**: Use plugin-based system with Processor trait.

**Rationale**:
- Extensibility: Users can add custom processors
- Composition: Processors can invoke other processors
- Flexibility: Easy to add new formats
- Rust traits: Natural fit for this pattern

**Consequences**:
- Pros: Highly extensible, composable
- Cons: More complex than hard-coded formats

**Alternatives Considered**:
- Hard-coded formats: Simpler, less extensible
- Full dynamic plugin loading: More complex, potential safety issues

---

### ADR-004: Parsing Library (TBD)
**Status**: Under Consideration
**Date**: 2024-11-14

**Context**: Need to choose parsing approach.

**Options**:
1. **pest**: PEG grammar, declarative, good errors
2. **nom**: Combinators, flexible, steep learning curve
3. **chumsky**: Error recovery, good for interactive use
4. **Hand-written**: Full control, more work

**Decision**: TBD - requires research and prototyping.

**Next Steps**:
- Prototype simple AsciiDoc grammar with each option
- Evaluate error messages
- Assess performance
- Consider maintainability

---

## Next Steps

### Immediate (v0.1.0 - MVP)
1. Research and choose parsing library
2. Design basic AST for common elements
3. Implement core parser for subset of AsciiDoc
4. Create simple HTML processor
5. Basic CLI

### Short Term (v0.2.0)
1. Expand AsciiDoc coverage
2. Add more processors (Markdown, validation)
3. Improve error messages
4. Add testing suite

### Medium Term (v0.3.0+)
1. Plugin loading system
2. Advanced processors (PDF, etc.)
3. Performance optimization
4. Comprehensive documentation

---

## Document Change Log

### 2024-11-14
- Initial architecture document created
- Defined core components and data flow
- Established architectural decisions (ADRs 001-004)
- Outlined design principles and technology stack
