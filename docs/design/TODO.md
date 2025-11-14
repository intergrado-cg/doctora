# Doctor A (doctora) - Project TODO

**Last Updated**: 2024-11-14
**Project**: Doctor A - AsciiDoc Parser and Processor
**Language**: Rust

---

## Summary

- **Total Milestones**: 3
- **Completed Milestones**: 0
- **Total Features**: 5
- **Completed Features**: 0 (0%)
- **Total Tasks**: 0 (will grow as features are designed)
- **Completed Tasks**: 0 (0%)

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
**Status**: 📋 Not Started
**Goal**: Parse basic AsciiDoc and validate structure

### Feature: Parsing Library Selection
**Status**: 📋 Not Started
**Priority**: Critical
**Effort**: Medium

*Note: Tasks will be added when feature specification is created*

- [ ] Research parsing libraries (pest, nom, chumsky, hand-written)
- [ ] Create prototype with top candidates
- [ ] Evaluate error messages
- [ ] Benchmark performance
- [ ] Document decision (ADR-004)
- [ ] Create feature specification

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

### Feature: Basic Parser Implementation
**Status**: 📋 Not Started
**Priority**: Critical
**Effort**: X-Large
**Dependencies**: Parsing Library Selection, Core AST Design

*Note: Requires feature specification*

- [ ] Implement lexer/tokenizer
- [ ] Implement basic parser
- [ ] Implement AST builder
- [ ] Implement basic validator
- [ ] Add error handling
- [ ] Create feature specification

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

*No features completed yet*

---

## Cancelled/Deferred ❌

*No features cancelled or deferred yet*

---

## Design Work Needed

Features that need specifications before implementation:

1. ⚠️ **Parsing Library Selection** - Research and prototyping needed
2. ⚠️ **Core AST Design** - Detailed design needed
3. ⚠️ **Basic Parser Implementation** - Architecture design needed
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
- Which parsing library to use? (pest vs nom vs chumsky vs hand-written)
- What subset of AsciiDoc to support in v0.1.0?
- Should we follow asciidoctor behavior exactly or innovate?
- How to handle backwards compatibility in future versions?

### Recent Changes
- 2024-11-14: Initial TODO list created
- 2024-11-14: Project foundation completed
- 2024-11-14: Milestones 1-3 outlined

---

## Next Actions

1. **Immediate**: Create feature specification for "Parsing Library Selection"
2. **Immediate**: Research parsing libraries (pest, nom, chumsky)
3. **Short-term**: Design core AST structure
4. **Short-term**: Create detailed specifications for Milestone 1 features
5. **Medium-term**: Begin implementation of basic parser
