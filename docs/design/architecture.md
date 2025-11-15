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
- **Technology**: **Logos** (compile-time lexer generator, see ADR-004)

#### Parser
- **Purpose**: Build AST from token stream
- **Input**: Token stream from Logos
- **Output**: Raw AST
- **Technology**: **Winnow** (parser combinator with zero-copy design, see ADR-004)
- **Approach**: Parser combinators with context-sensitive state management

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

### Parsing Libraries
**Chosen Approach**: Logos + Winnow (see ADR-004)

**Primary Libraries**:
- **logos** (v0.15): Compile-time lexer generator for fast tokenization (1,200+ MB/s)
- **winnow** (v0.7): Parser combinator with zero-copy design (112 MiB/s on typical documents)

**Rationale**:
- Best performance validated by POC benchmarks (16-45% faster than alternatives)
- Zero-copy design for scalability with large documents
- Handles AsciiDoc's context-sensitive parsing requirements
- Migration path to winnow or hand-written parser if needed

### Other Dependencies
- **Error Handling**: `thiserror`, `anyhow`
- **Diagnostics**: `miette` (for beautiful error reporting with code context)
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

### ADR-004: Parsing Library - Logos + Winnow
**Status**: Accepted
**Date**: 2024-11-14
**Updated**: 2025-11-15 (Revised after POC benchmarks)

**Context**:
AsciiDoc is a context-sensitive markup language with complex parsing requirements:
- Multi-stage attribute substitution with dependency chains
- Conditional directives evaluated during parsing (ifdef/ifndef)
- Stateful delimited block nesting with dynamic delimiters
- Complex table parsing with nested structures
- Include directives requiring file system access during parsing

Need to choose parsing approach that can handle these requirements while providing excellent error messages and performance.

**Research Conducted**:
Analyzed existing Rust AsciiDoc implementations:
- **asciidoc-parser**: Hand-written recursive descent, 99% test coverage
- **asciidocr**: Regex-based with custom AST
- **asciidork**: Hand-written custom parser
- **Finding**: ALL existing implementations use hand-written parsers, none use parser combinators or PEG generators

Evaluated parsing libraries with benchmarks (JSON parsing, AMD Ryzen 7 3700x):
- **Chumsky**: 210 µs (533 MB/s) - Best error recovery
- **Winnow**: 179 µs (627 MB/s) - Best performance
- **Nom**: 527 µs (213 MB/s) - Mature ecosystem
- **Pest**: 1,971 µs (57 MB/s) - 14x slower, PEG limitations
- **Logos** (lexer): 1,200+ MB/s - Fastest tokenization

**Decision**: Use **Logos (lexer) + Winnow (parser)**.

**POC Results** (2025-11-15): Phase 1 proof-of-concept implemented both Logos+Chumsky and Logos+Winnow parsers with identical functionality. Comprehensive benchmarks showed Winnow's clear performance advantage:
- Small documents (100B): Winnow 33% faster (87.86 MiB/s vs 65.99 MiB/s)
- Medium documents (1KB): Winnow 45% faster (112.44 MiB/s vs 77.56 MiB/s)
- Large documents (10KB): Winnow 16% faster (97.55 MiB/s vs 84.39 MiB/s)

See `docs/BENCHMARK_RESULTS.md` for detailed analysis.

**Rationale**:
1. **Performance**: Winnow demonstrated 16-45% faster parsing than Chumsky in real-world benchmarks, with best gains on typical document sizes (1KB: 112.44 MiB/s)
2. **Consistency**: Winnow maintains performance advantage across all input sizes tested (small, medium, large documents)
3. **Zero-Copy Design**: Winnow's architecture minimizes allocations and memory overhead, scales better for large documents
4. **Production Ready**: Winnow 0.7 is stable release (vs Chumsky 1.0 alpha), actively maintained as modern fork of nom
5. **Ecosystem**: Larger community, more examples, proven in production Rust projects
6. **Context-Sensitivity**: Parser combinators support AsciiDoc's stateful parsing requirements (attributes, conditionals, includes)
7. **Error Handling**: Flexible error recovery can be implemented on top of Winnow's `ErrMode` system
8. **Future-Proof**: Performance headroom allows for additional features without bottleneck concerns
9. **Migration Path**: Logos lexer remains unchanged; only parser layer uses Winnow

**Consequences**:

*Pros*:
- Fastest overall parsing performance (16-45% faster than Chumsky)
- Stable release (Winnow 0.7) with predictable API
- Zero-copy design minimizes allocations and memory overhead
- Proven in production - larger ecosystem than Chumsky
- Automatic span tracking for precise error locations
- Modern fork of nom with improved ergonomics
- Idiomatic Rust with strong type safety
- no_std support for embedded use cases

*Cons*:
- More verbose syntax (explicit type annotations required)
- Manual error recovery implementation (vs Chumsky's built-in)
- Requires understanding parser combinator concepts
- Learning curve for contributors unfamiliar with Winnow

*Mitigations*:
- Document Winnow patterns and provide examples for contributors
- Implement custom error recovery layer on top of Winnow
- Extensive testing validates correctness despite manual approach
- POC demonstrated comparable implementation effort to Chumsky
- Tokens are library-agnostic (can migrate parser if needed)

**Alternatives Considered**:

1. **Pest** (PEG Generator)
   - *Rejected*: 14x slower (57 MB/s vs 533 MB/s)
   - PEG cannot handle context-sensitive parsing well
   - No left recursion support
   - AsciiDoc PEG experiment abandoned with 30+ unmapped features
   - Too slow for large documents

2. **Chumsky** (Parser Combinator)
   - *Not Chosen*: Excellent error recovery but 16-45% slower in POC benchmarks
   - Alpha status (1.0.0-alpha.8) vs Winnow's stable release
   - More compact syntax and better type inference
   - Built-in error recovery is sophisticated but comes with performance cost
   - Valid alternative if error recovery > performance in future versions

3. **Nom** (Parser Combinator)
   - *Not Chosen*: Winnow is nom's successor with better errors
   - Slower than Winnow (213 MB/s vs 627 MB/s)
   - Larger ecosystem but older API design
   - Use Winnow instead if choosing this approach

4. **Hand-Written Parser**
   - *Future Option*: Maximum control, proven by all existing projects
   - More development effort and maintenance burden
   - Can migrate to this if library approach proves insufficient
   - Logos lexer can be retained with hand-written parser

5. **Logos + Chumsky**
   - *Previously Considered*: Initial choice before POC benchmarks
   - Better built-in error recovery than Winnow
   - More elegant API with better type inference
   - POC showed 16-45% slower performance than Winnow
   - Performance trade-off not justified for error recovery benefits

**Implementation Plan**:
1. **Phase 1**: Proof-of-concept comparing Logos+Chumsky vs Logos+Winnow ✅ **COMPLETE** (2025-11-15)
   - Result: Winnow selected based on 16-45% performance advantage
   - 52 tests passing, working end-to-end pipeline
   - Benchmarks documented in docs/BENCHMARK_RESULTS.md
2. **Phase 2**: Implement Logos lexer with complete AsciiDoc token set
3. **Phase 3**: Build Winnow parser for core AsciiDoc features
4. **Phase 4**: Integrate miette for diagnostic reporting
5. **Phase 5**: Implement custom error recovery layer on top of Winnow
6. **Phase 6**: Benchmark with real AsciiDoc documents, optimize if needed

**Success Criteria**:
- Parse 1MB AsciiDoc document in < 50ms (20 MB/s minimum)
- Report all syntax errors in document (not just first error)
- Error messages include line/column, context, and suggestions
- Test coverage > 95% for parser core

**Review Date**: ✅ Completed 2025-11-15 (POC benchmarks confirmed Winnow decision). Next review: After v0.1.0 release.

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

### 2025-11-15
- **ADR-004 Updated**: Revised parsing library decision from Chumsky to Winnow
- Phase 1 POC completed with comprehensive benchmarks
- Winnow selected based on 16-45% performance advantage over Chumsky
- Updated rationale, consequences, and alternatives sections
- Added POC results and benchmark references (docs/BENCHMARK_RESULTS.md)
- Updated Core Components section to reflect Winnow technology choice

### 2024-11-14
- Initial architecture document created
- Defined core components and data flow
- Established architectural decisions (ADRs 001-004)
- Outlined design principles and technology stack
