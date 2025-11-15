---
name: rust-coding
description: Expert skill for implementing features in Rust, writing idiomatic and production-quality code, following Rust API guidelines, and leveraging the Rust ecosystem effectively.
---

# rust-coding Skill

## Primary Responsibility

**Implement features in Rust following specifications from `docs/design/features/` directory.**

Write production-quality, idiomatic Rust code that:
- Follows the Rust API Guidelines
- Leverages Rust's ownership system effectively
- Uses appropriate error handling patterns
- Is well-documented and tested
- Integrates well with the Rust ecosystem

---

## When to Use This Skill

Use the rust-coding skill when:
- [ ] Implementing a feature from a specification
- [ ] Writing new modules or components
- [ ] Creating data structures (structs, enums)
- [ ] Implementing traits and methods
- [ ] Setting up error handling for a module
- [ ] Integrating external crates
- [ ] Writing API surfaces that will be used by other code
- [ ] Creating examples or documentation with code

---

## Core Workflow

### Implementing from a Feature Specification

1. **Read & Understand**
   - Read feature spec in `docs/design/features/[feature-name].md`
   - Understand requirements, API design, data model
   - Note implementation tasks and acceptance criteria

2. **Plan Module Structure**
   - Determine where code goes (which modules, files)
   - Plan public vs. private API surface
   - Identify dependencies (crates, other modules)

3. **Write Data Structures**
   - Define structs and enums
   - Derive appropriate traits (Debug, Clone, PartialEq, etc.)
   - Add documentation comments

4. **Implement Core Logic**
   - Write function implementations
   - Implement trait methods
   - Add error handling
   - Follow ownership patterns

5. **Document**
   - Add `///` doc comments for public items
   - Include examples in doc comments
   - Document panics, errors, safety

6. **Test**
   - Write unit tests
   - Write doc tests
   - Test edge cases

7. **Verify Quality**
   - Run `cargo clippy`
   - Run `cargo fmt`
   - Run `cargo test`
   - Run `cargo doc --open`

8. **Update Feature Spec**
   - Mark tasks complete in spec's Implementation Plan
   - Update Status field

---

## Core Competencies

### 1. Rust Language Fundamentals

**Ownership & Borrowing**:
- Understand ownership transfer, borrowing rules, lifetimes
- Use `&T` for shared borrows, `&mut T` for exclusive borrows
- Know when to clone vs. borrow
- Use lifetime parameters when needed

**Types & Traits**:
- Leverage type system for correctness
- Implement standard traits appropriately
- Use generics for flexibility
- Use trait objects when dynamic dispatch needed

**Error Handling**:
- Use `Result<T, E>` for recoverable errors
- Use `Option<T>` for optional values
- Use `?` operator for error propagation
- Create custom error types with thiserror

**Pattern Matching**:
- Use `match` for exhaustive handling
- Use `if let` and `while let` for simple cases
- Leverage destructuring in patterns

### 2. Idiomatic Rust Patterns

**API Design**:
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use builder patterns for complex construction
- Provide conversion traits (From, Into, TryFrom, TryInto)
- Return iterators from functions when appropriate

**Ownership Patterns**:
- Prefer `&str` over `String` for parameters
- Use `Cow<'_, T>` for conditional ownership
- Use `Arc<T>` for shared ownership across threads
- Use `Rc<T>` for shared ownership in single-threaded code

**Iterator Patterns**:
- Use iterator adapters (map, filter, fold) over manual loops
- Implement `Iterator` trait for custom types
- Use `collect()` to build collections
- Leverage `Iterator::chain()`, `zip()`, `enumerate()`

**Error Patterns**:
- Define module-level error types
- Use `Result<T>` as return type alias
- Implement `std::error::Error` for custom errors
- Use error context with libraries like anyhow (in applications)

### 3. Rust Ecosystem & Crates

**Common Crates**:
- `serde` - Serialization/deserialization
- `thiserror` - Derive for error types
- `itertools` - Extended iterator methods
- `rayon` - Data parallelism
- `once_cell` - Lazy initialization

**Parser Crates** (specific to doctora):
- `pest` - PEG parser generator
- `nom` - Parser combinators
- `logos` - Lexer generator

**Finding Crates**:
- Search on crates.io by keywords
- Check docs.rs for documentation
- Review GitHub stars and maintenance
- Read API docs and examples

### 4. Code Organization

**Module Structure**:
```rust
// lib.rs or main.rs
pub mod parser;
pub mod ast;
pub mod processor;

mod utils; // Private module
```

**Visibility**:
- Use `pub` for public API
- Use `pub(crate)` for internal sharing
- Use `pub(super)` for parent module only
- Default (no pub) for private

**File Organization**:
```
src/
├── lib.rs          # Library root, re-exports
├── parser/
│   ├── mod.rs      # Module definition
│   ├── lexer.rs    # Submodule
│   └── parser.rs   # Submodule
├── ast.rs          # Single-file module
└── processor/
    └── mod.rs
```

### 5. Documentation

**Doc Comments**:
```rust
/// Brief one-line description.
///
/// Longer description with details about behavior,
/// use cases, and important notes.
///
/// # Examples
///
/// ```
/// use doctora::parser::Parser;
///
/// let parser = Parser::new();
/// let result = parser.parse("= Title");
/// assert!(result.is_ok());
/// ```
///
/// # Errors
///
/// Returns `Err` if input is invalid.
///
/// # Panics
///
/// Panics if called before initialization.
pub fn parse(&self, input: &str) -> Result<Ast> {
    // ...
}
```

**Module-Level Docs**:
```rust
//! Parser module for AsciiDoc syntax.
//!
//! This module provides lexer and parser implementations
//! for converting AsciiDoc text into an AST.

pub mod lexer;
pub mod parser;
```

### 6. Testing

**Unit Tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let result = my_function(input);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "error message")]
    fn test_panic_case() {
        panicking_function();
    }
}
```

**Integration Tests**:
```
tests/
├── integration_test.rs
└── common/
    └── mod.rs  # Shared test utilities
```

**Doc Tests**:
- Examples in doc comments are automatically tested
- Use `no_run` for examples that shouldn't execute
- Use `ignore` for examples to skip

---

## Best Practices

### Code Quality

1. **Follow Rust API Guidelines**
   - Naming: `snake_case` for functions/variables, `CamelCase` for types
   - Use standard traits where applicable
   - Provide meaningful error messages

2. **Leverage the Type System**
   - Use newtypes for type safety
   - Make invalid states unrepresentable
   - Use enums for variants, structs for products

3. **Minimize Unsafe Code**
   - Avoid `unsafe` unless absolutely necessary
   - Document safety invariants
   - Encapsulate unsafe in safe abstractions

4. **Handle Errors Properly**
   - Don't use `unwrap()` in library code
   - Use `expect()` only with clear messages
   - Propagate errors with `?`
   - Create informative error types

5. **Document Public APIs**
   - Every public item needs doc comment
   - Include examples for non-trivial functions
   - Document error conditions
   - Document panics

### Performance

1. **Efficient Ownership**
   - Avoid unnecessary clones
   - Borrow when possible
   - Use `&str` for string parameters
   - Consider `Cow<'_, T>` for conditional ownership

2. **Choose Appropriate Data Structures**
   - `Vec<T>` for sequential data
   - `HashMap<K, V>` for key-value lookup
   - `HashSet<T>` for unique values
   - Consider `BTreeMap`/`BTreeSet` for ordered data

3. **Use Iterators**
   - Composable and often optimized
   - Lazy evaluation
   - Chain operations efficiently

4. **Profile Before Optimizing**
   - Use criterion for benchmarks
   - Measure actual performance
   - Optimize hot paths only

### Maintainability

1. **Keep Functions Small**
   - Single responsibility
   - Easy to test
   - Easy to understand

2. **Use Descriptive Names**
   - Clear intent
   - Avoid abbreviations
   - Follow conventions

3. **Write Tests First (TDD)**
   - Clarifies requirements
   - Ensures testability
   - Drives good design

4. **Refactor Regularly**
   - Reduce duplication
   - Improve clarity
   - Maintain clean code

---

## Common Patterns for doctora

### Parser Pattern (Recursive Descent)

```rust
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(input: &str) -> Result<Self> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;
        Ok(Self { lexer, current_token })
    }

    fn advance(&mut self) -> Result<()> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token> {
        if self.current_token.kind == expected {
            let token = self.current_token.clone();
            self.advance()?;
            Ok(token)
        } else {
            Err(Error::UnexpectedToken {
                expected,
                found: self.current_token.clone(),
            })
        }
    }

    pub fn parse(&mut self) -> Result<Ast> {
        self.parse_document()
    }

    fn parse_document(&mut self) -> Result<Document> {
        // Implementation
    }
}
```

### AST Node Pattern

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Section(Section),
    Paragraph(Paragraph),
    List(List),
    CodeBlock(CodeBlock),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    pub level: u8,
    pub title: Vec<Inline>,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    Text(String),
    Emphasis(Vec<Inline>),
    Strong(Vec<Inline>),
    Link { text: Vec<Inline>, url: String },
}
```

### Processor Trait Pattern

```rust
pub trait Processor {
    /// Process an AST and return the output.
    fn process(&self, ast: &Ast) -> Result<String>;

    /// Get the output format name.
    fn format_name(&self) -> &str;
}

pub struct HtmlProcessor {
    options: HtmlOptions,
}

impl Processor for HtmlProcessor {
    fn process(&self, ast: &Ast) -> Result<String> {
        // Implementation
    }

    fn format_name(&self) -> &str {
        "html"
    }
}
```

### Builder Pattern

```rust
#[derive(Debug, Default)]
pub struct ParserBuilder {
    strict: bool,
    max_depth: Option<usize>,
}

impl ParserBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    pub fn build(self) -> Parser {
        Parser {
            strict: self.strict,
            max_depth: self.max_depth.unwrap_or(100),
        }
    }
}
```

---

## Research Resources

### Official Rust Documentation
- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust Reference**: https://doc.rust-lang.org/reference/
- **Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustonomicon** (unsafe): https://doc.rust-lang.org/nomicon/

### Crate Documentation
- **docs.rs**: Documentation for all published crates
- **crates.io**: Crate registry and search
- **lib.rs**: Alternative crate browser

### Community
- **Stack Overflow**: Tag [rust]
- **r/rust**: Community discussions
- **Rust Users Forum**: users.rust-lang.org
- **This Week in Rust**: Weekly newsletter

### Parsing Specific
- **pest**: https://pest.rs
- **nom**: https://docs.rs/nom
- **Parser Combinators**: Various blog posts and tutorials

---

## Examples

### Simple Module Implementation

```rust
//! Simple lexer module example.

use std::str::Chars;

/// A token in the input stream.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// Token types.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Newline,
    Text(String),
    Eof,
}

/// Span tracking position in source.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// Lexer for tokenizing input.
pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from input string.
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars(),
            current_pos: 0,
        }
    }

    /// Get the next token from the input.
    pub fn next_token(&mut self) -> Token {
        // Implementation
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_creation() {
        let lexer = Lexer::new("test input");
        assert_eq!(lexer.current_pos, 0);
    }
}
```

---

## Lessons Learned

### From Implementation Work
*This section will be updated as implementation progresses with insights from actual coding work.*

### Patterns That Worked Well
*To be filled with successful patterns discovered during implementation.*

### Patterns to Avoid
*To be filled with anti-patterns or problematic approaches encountered.*

### Crate-Specific Insights
*To be filled with learnings about specific crates used in the project.*

---

## Continuous Improvement

After each significant implementation:
1. Review code for improvements
2. Extract reusable patterns
3. Update "Lessons Learned" section
4. Update "Common Patterns" if new patterns emerge
5. Note any crate discoveries or techniques

This skill grows more valuable as it accumulates real-world implementation experience.
