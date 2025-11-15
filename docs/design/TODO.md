# Doctor A (doctora) - Project TODO

**Last Updated**: 2025-11-14
**Project**: Doctor A - AsciiDoc Parser and Processor
**Language**: Rust

---

## Summary

- **Total Milestones**: 3
- **Completed Milestones**: 0
- **Total Features**: 6
- **Completed Features**: 1 (17%)
- **Total Tasks**: 79 (Milestone 1 tasks counted)
- **Completed Tasks**: 6/79 (8%)

---

## Legend

- ✅ Completed
- 🔄 In Progress
- ⏸️ Blocked
- ❌ Cancelled
- 📋 Not Started

---

## Milestone 0: Project Foundation

**Target**: 2024-11-30
**Status**: 🔄 In Progress

### Feature: Project Setup ✅
**Status**: ✅ Completed (2024-11-14)
**Priority**: Critical
**Effort**: Small

- [x] Create GitHub repository ✅
- [x] Initialize git repository ✅
- [x] Create README.md ✅
- [x] Setup documentation structure (docs/design/) ✅
- [x] Create Claude Code agents and skills ✅

---

## Milestone 1: Core Parser (v0.1.0 - MVP)

**Target**: TBD
**Status**: 🔄 In Progress (1/5 features completed, 6/79 tasks)
**Goal**: Parse basic AsciiDoc and validate structure

### Feature: Parsing Library Selection ✅
**Status**: ✅ Completed (2025-11-14)
**Priority**: Critical
**Effort**: Medium
**Spec**: `docs/design/features/core-parser.md`

- [x] Research parsing libraries (pest, nom, chumsky, winnow, hand-written) ✅
- [x] Research existing Rust AsciiDoc implementations (asciidoc-parser, asciidocr, asciidork) ✅
- [x] Evaluate error recovery capabilities ✅
- [x] Benchmark performance (JSON parsing comparisons) ✅
- [x] Document decision (ADR-004: Logos + Chumsky) ✅
- [x] Create feature specification (core-parser.md) ✅
- [ ] Create proof-of-concept (Logos + Chumsky vs Logos + Winnow)

**Decision**: Use **Logos (lexer) + Chumsky (parser)** for best error recovery and excellent performance.

### Feature: Core AST Design
**Status**: 📋 Not Started
**Priority**: Critical
**Effort**: Large

*Note: Requires feature specification*

- [ ] Design Document structure
- [ ] Design Block nodes (section, paragraph, list, code block)
- [ ] Design Inline nodes (text, emphasis, link, image)
- [ ] Design metadata handling
- [ ] Create feature specification

### Feature: Core Parser Implementation
**Status**: 🔄 In Progress (0/67 tasks completed)
**Priority**: Critical
**Effort**: X-Large
**Dependencies**: Parsing Library Selection ✅, Core AST Design
**Spec**: `docs/design/features/core-parser.md`

#### Phase 1: Proof of Concept (1 week)
- [ ] Set up Rust project with logos and chumsky dependencies
- [ ] Define minimal Token enum (headings, paragraphs, bold, italic)
- [ ] Implement basic Logos lexer for minimal tokens
- [ ] Create simple Chumsky parser for subset (document → sections → paragraphs)
- [ ] Parse example AsciiDoc files, validate approach
- [ ] Benchmark POC: compare Logos+Chumsky vs Logos+Winnow
- [ ] Decision point: confirm Chumsky or switch to Winnow

#### Phase 2: Lexer Implementation (1 week)
- [ ] Define complete Token enum for all AsciiDoc syntax
- [ ] Implement Logos patterns for all tokens
- [ ] Add span tracking to lexer output
- [ ] Write lexer unit tests (>95% coverage)
- [ ] Benchmark lexer performance (target: 500+ MB/s)
- [ ] Add error token handling

#### Phase 3: Core AST Types (3 days)
- [ ] Define Document, Header, Block, InlineNode types
- [ ] Add Span to all AST nodes
- [ ] Implement Default, Debug, Clone for AST types
- [ ] Add serde Serialize/Deserialize for AST
- [ ] Write AST construction tests

#### Phase 4: Block Parser (2 weeks)
- [ ] Document parser (header + body)
- [ ] Section parser (with nesting)
- [ ] Paragraph parser
- [ ] Delimited block parser (all types)
- [ ] List parser (unordered, ordered, description)
- [ ] Table parser (basic tables)
- [ ] Table parser (nested tables, cell styles)
- [ ] Block macro parser
- [ ] Block parser integration tests

#### Phase 5: Inline Parser (1 week)
- [ ] Text and whitespace handling
- [ ] Bold, italic, monospace formatting
- [ ] Links (inline and macro)
- [ ] Inline macros (image, etc.)
- [ ] Attribute references
- [ ] Subscript, superscript, mark
- [ ] Inline parser tests

#### Phase 6: Context-Sensitive Features (1 week)
- [ ] Attribute definition and substitution
- [ ] Conditional directives (ifdef/ifndef/ifeval)
- [ ] Include directive processing
- [ ] Parser state management
- [ ] Context-sensitive tests

#### Phase 7: Error Recovery (1 week)
- [ ] Define ParseError types with miette integration
- [ ] Implement synchronization strategy
- [ ] Add error recovery to block parser
- [ ] Add error recovery to inline parser
- [ ] Multiple error collection
- [ ] Error message quality tests

#### Phase 8: Validation (3 days)
- [ ] Implement post-parse validator
- [ ] Validate attribute references
- [ ] Validate section nesting
- [ ] Validate include paths
- [ ] Validator tests

#### Phase 9: Public API (2 days)
- [ ] Design Parser public API
- [ ] Implement ParserConfig
- [ ] Implement ParseResult
- [ ] Add convenience methods (parse_file, etc.)
- [ ] API documentation with examples

#### Phase 10: Performance Optimization (1 week)
- [ ] Benchmark full parser on real AsciiDoc documents
- [ ] Profile hot paths, optimize allocations
- [ ] Add zero-copy optimizations where possible
- [ ] Streaming parser (if needed)
- [ ] Final performance tests (must meet requirements)

#### Phase 11: Testing & Quality (1 week)
- [ ] Achieve >95% test coverage
- [ ] Add property-based tests (proptest)
- [ ] Set up fuzzing with cargo-fuzz
- [ ] Compatibility tests against asciidoctor
- [ ] Edge case tests from research
- [ ] Documentation with rustdoc

### Feature: CLI Interface
**Status**: 📋 Not Started
**Priority**: High
**Effort**: Medium

*Note: Requires feature specification*

- [ ] Design CLI interface
- [ ] Implement argument parsing (using clap)
- [ ] Implement file I/O
- [ ] Add basic options (input, output, format)
- [ ] Create feature specification

### Feature: HTML Processor
**Status**: 📋 Not Started
**Priority**: High
**Effort**: Large
**Dependencies**: Basic Parser Implementation

*Note: Requires feature specification*

- [ ] Design HTML processor architecture
- [ ] Implement Processor trait
- [ ] Implement HTML generation for basic nodes
- [ ] Handle nested structures
- [ ] Add styling/formatting options
- [ ] Create feature specification

---

## Milestone 2: Extended AsciiDoc Support (v0.2.0)

**Target**: TBD
**Status**: 📋 Not Started
**Goal**: Expand AsciiDoc syntax coverage

### Feature: Advanced Block Elements
**Status**: 📋 Not Started
**Priority**: High
**Effort**: Large

*Note: Specification needed*

Topics to cover:
- Tables
- Admonitions
- Sidebars
- Quote blocks
- Example blocks

### Feature: Advanced Inline Elements
**Status**: 📋 Not Started
**Priority**: High
**Effort**: Medium

*Note: Specification needed*

Topics to cover:
- Subscript/superscript
- Attributes/macros
- Cross-references
- Footnotes

### Feature: Document Attributes
**Status**: 📋 Not Started
**Priority**: Medium
**Effort**: Medium

*Note: Specification needed*

Topics to cover:
- Built-in attributes
- Custom attributes
- Conditional inclusion
- Attribute substitution

### Feature: Additional Processors
**Status**: 📋 Not Started
**Priority**: Medium
**Effort**: Large

*Note: Specification needed*

Processors to add:
- Markdown processor
- Plain text processor
- JSON/AST export processor

---

## Milestone 3: Extensibility & Polish (v0.3.0)

**Target**: TBD
**Status**: 📋 Not Started
**Goal**: Make system production-ready and extensible

### Feature: Processor Plugin System
**Status**: 📋 Not Started
**Priority**: High
**Effort**: X-Large

*Note: Specification needed*

### Feature: Advanced Error Handling
**Status**: 📋 Not Started
**Priority**: High
**Effort**: Medium

*Note: Specification needed*

### Feature: Performance Optimization
**Status**: 📋 Not Started
**Priority**: Medium
**Effort**: Large

*Note: Specification needed*

### Feature: Comprehensive Testing Suite
**Status**: 📋 Not Started
**Priority**: High
**Effort**: Large

*Note: Specification needed*

---

## Backlog

Features not yet assigned to a milestone.

### Feature: PDF Processor
**Priority**: Medium
**Effort**: X-Large
**Notes**: Requires external library integration

### Feature: Include Directive Support
**Priority**: Medium
**Effort**: Medium
**Notes**: Load external files into document

### Feature: Diagram Support
**Priority**: Low
**Effort**: Large
**Notes**: PlantUML, GraphViz, etc.

### Feature: Syntax Highlighting
**Priority**: Low
**Effort**: Medium
**Notes**: Code block syntax highlighting

### Feature: Custom Macros
**Priority**: Low
**Effort**: Large
**Notes**: User-defined macros

---

## Completed Features ✅

### Parsing Library Selection (2025-11-14)
**Milestone**: 1 - Core Parser
**Outcome**: Selected **Logos + Chumsky** approach
- Researched 5 parsing libraries and 4 existing Rust AsciiDoc implementations
- Documented comprehensive ADR-004 in architecture.md
- Created detailed feature specification (core-parser.md)
- Decision: Logos for lexing (1,200+ MB/s), Chumsky for parsing (533 MB/s) with error recovery
- Next: Proof-of-concept implementation

---

## Cancelled/Deferred ❌

*No features cancelled or deferred yet*

---

## Design Work Needed

Features that need specifications before implementation:

1. ✅ ~~**Parsing Library Selection**~~ - Complete (see core-parser.md)
2. ⚠️ **Core AST Design** - Detailed in core-parser.md, ready for implementation
3. 🔄 **Core Parser Implementation** - Specification complete (core-parser.md), implementation in progress
4. ⚠️ **CLI Interface** - API design needed
5. ⚠️ **HTML Processor** - Architecture design needed

---

## Notes

### Project Status
- Project foundation is complete
- Documentation structure in place
- Ready to begin feature design phase
- Next step: Research parsing libraries and create feature specifications

### Priorities for v0.1.0 (MVP)
1. Choose parsing library
2. Design and implement core AST
3. Implement basic parser (subset of AsciiDoc)
4. Create simple HTML processor
5. Build basic CLI

### Open Questions
- ~~Which parsing library to use?~~ **RESOLVED**: Logos + Chumsky (see ADR-004)
- What subset of AsciiDoc to support in v0.1.0? → See core-parser.md Phase 1-5
- Should Chumsky alpha status concern us? → Mitigated by pinning version, can swap to Winnow
- Should we follow asciidoctor behavior exactly or innovate? → Aim for compatibility, document differences
- How to handle backwards compatibility in future versions? → TBD after v0.1.0

### Recent Changes
- 2025-11-14: **Parsing Library Selection** completed - Decision: Logos + Chumsky
- 2025-11-14: Created comprehensive feature spec (core-parser.md) with 67 implementation tasks
- 2025-11-14: Updated ADR-004 with detailed research findings and benchmarks
- 2025-11-14: Added 11-phase implementation plan to TODO.md
- 2024-11-14: Initial TODO list created
- 2024-11-14: Project foundation completed
- 2024-11-14: Milestones 1-3 outlined

---

## Next Actions

1. **Immediate**: Set up Rust project (cargo init, add logos/chumsky dependencies)
2. **Immediate**: Create proof-of-concept parser (Phase 1 tasks)
3. **Immediate**: Benchmark Logos+Chumsky vs Logos+Winnow
4. **Short-term**: Implement complete Logos lexer (Phase 2 tasks)
5. **Short-term**: Define all AST types (Phase 3 tasks)
6. **Medium-term**: Build block parser (Phase 4 tasks)
7. **Medium-term**: Create feature specifications for CLI and HTML Processor
