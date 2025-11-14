# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Doctor A (doctora)** is a modular AsciiDoc parser and processor written in Rust, designed as an alternative to asciidoctor. The project separates parsing (AsciiDoc → AST) from processing (AST → output formats), allowing pluggable processors for different output formats.

**Current Status**: Early development - no Rust code yet, currently in architectural design phase.

## Architecture

### Three-Layer Architecture

1. **Core Parser Layer**: Lexer → Parser → AST Builder → Validator
   - Input: AsciiDoc text
   - Output: Validated Abstract Syntax Tree (AST)

2. **AST Layer**: Document structure representation
   - Document metadata
   - Block nodes (sections, paragraphs, lists, tables, code blocks)
   - Inline nodes (text, emphasis, links, images)

3. **Processor Layer**: Plugin-based transformation system
   - Processors implement a common `Processor` trait
   - Processors can invoke other processors (e.g., PDF processor calling image processor)
   - Registry manages available processors

### Key Design Principles

- **Separation of Concerns**: Parser only parses, processors only transform
- **Modularity**: Independent components with well-defined interfaces
- **Extensibility**: Plugin system for custom processors
- **Rust Idioms**: Use traits for polymorphism, leverage type system for correctness

## Documentation Structure

**All design documentation lives in `docs/design/`:**

```
docs/design/
├── architecture.md              # System architecture, ADRs, design decisions
├── TODO.md                       # Task tracking with checkboxes (what's done/not done)
├── templates/
│   └── feature-template.md      # Standard template for feature specs
└── features/
    └── [feature-name].md         # Detailed feature specifications
```

### Important Files

- **`docs/design/architecture.md`**: Complete system architecture with Architectural Decision Records (ADRs)
- **`docs/design/TODO.md`**: Master task list organized by milestones and features
- **`docs/design/templates/feature-template.md`**: Template for creating new feature specifications

## Claude Code Agents

This project uses specialized Claude Code agents:

### doctora-architect Agent

**When to invoke**: Feature design, architecture planning, progress tracking

**Capabilities**:
- Research AsciiDoc specs and Rust parsing libraries online
- Create detailed feature specifications using the template
- Maintain architecture documentation and ADRs
- Update TODO.md to track progress
- Self-learning: Updates experience log with valuable insights

**Skills**:
- `feature-design`: Creates/updates feature spec files in `docs/design/features/`
- `project-management`: Updates TODO.md to mark tasks complete, add new tasks

**Usage examples**:
- "Design a feature for parsing AsciiDoc tables"
- "Research Rust parsing libraries for our parser"
- "Update the TODO to mark completed tasks"
- "What's the current architecture?"

## Workflows

### Adding a New Feature

1. **Design Phase** (use doctora-architect agent):
   - Research the feature requirements
   - Create `docs/design/features/[feature-name].md` from template
   - Design architecture, API, data models
   - Document all sections (requirements, technical design, alternatives)

2. **Planning Phase** (use doctora-architect agent with project-management skill):
   - Break feature into implementation tasks
   - Add feature and tasks to `docs/design/TODO.md`
   - Set priority and effort estimates

3. **Implementation Phase** (future - when Rust code exists):
   - Implement according to feature spec
   - Mark tasks complete in TODO.md as you finish them
   - Update feature spec status as work progresses

### Updating Progress

**Always update TODO.md when completing work:**
- Mark checkboxes: `- [ ]` → `- [x] ✅`
- Update feature status: `📋 Not Started` → `🔄 In Progress` → `✅ Completed`
- Update summary statistics (completion percentages)
- Move completed features to the "Completed Features" section

### Creating Feature Specifications

**All feature specs must follow the template:**
1. Read `docs/design/templates/feature-template.md`
2. Create new file: `docs/design/features/[feature-name].md` (use kebab-case)
3. Fill in ALL sections (no placeholders remaining):
   - Overview, Status, Business Value
   - Requirements (functional and non-functional)
   - Technical Design (architecture, components, data model, API)
   - Implementation Plan (tasks with checkboxes)
   - Testing Strategy, Edge Cases
   - Alternatives Considered
   - Research References

## Development (Future)

**Note**: No Rust code exists yet. When development begins:

### Project Setup
```bash
# Initialize Rust project (future)
cargo init --name doctora

# Build (future)
cargo build

# Run tests (future)
cargo test

# Run (future)
cargo run -- input.adoc --format html
```

### Parsing Library Decision (TBD)

Current candidates under consideration (see ADR-004 in architecture.md):
- **pest**: PEG parser generator, declarative grammar
- **nom**: Parser combinator library, flexible
- **chumsky**: Error-recovery combinators
- **Hand-written**: Recursive descent parser

Decision pending research and prototyping.

## Self-Learning System

The doctora-architect agent learns from experience:

**Experience Logs**:
- `.claude/agents/doctora-architect-experience.md`: Chronological log of design decisions and research findings
- `.claude/skills/feature-design/SKILL.md`: Lessons Learned section
- `.claude/skills/project-management/SKILL.md`: Lessons Learned section

**When valuable insights are discovered:**
1. Always add to experience log first
2. Selectively promote to skill files if broadly applicable
3. Update with: research findings, design decisions, useful resources, lessons from completed work

## Key Resources

### Internal Documentation
- `docs/design/architecture.md`: Full architecture, ADRs, component design
- `docs/design/TODO.md`: Current project status and task list
- `README.md`: Project overview and high-level architecture

### External References (to be researched)
- AsciiDoc Language Specification
- Asciidoctor source code (Ruby reference implementation)
- Rust API Guidelines
- Rust parser library documentation (pest, nom, chumsky)

## Project Conventions

### File Naming
- Feature specs: `kebab-case.md` (e.g., `core-parser.md`, `html-processor.md`)
- Rust modules (future): `snake_case.rs`

### Status Indicators in TODO.md
- 📋 Not Started
- 🔄 In Progress (include task count: "2/5 tasks completed")
- ✅ Completed (include date)
- ⏸️ Blocked
- ❌ Cancelled

### Priority Levels
- Critical: Must have for MVP
- High: Important for usability
- Medium: Valuable addition
- Low: Nice to have

### Effort Estimates
- Small: < 1 day
- Medium: 1-3 days
- Large: 3-7 days
- X-Large: > 1 week

## Important Notes

- **No implementation code exists yet** - project is in design phase
- **Feature specs come before code** - always create detailed specifications first
- **Keep TODO.md current** - update immediately when tasks are completed
- **Architecture decisions go in ADRs** - document in `docs/design/architecture.md`
- **Use the template** - all feature specs must follow `feature-template.md` structure
- **Research first** - use WebSearch/WebFetch to inform design decisions
