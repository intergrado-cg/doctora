---
name: code-review
description: Expert skill for reviewing Rust code quality, ensuring idiomatic patterns, checking safety and performance, and suggesting improvements. Focuses on code maintainability, documentation, and adherence to Rust best practices.
---

# code-review Skill

## Primary Responsibility

**Review Rust code for quality, correctness, idiomacy, safety, and performance.**

Ensure code:
- Follows Rust API Guidelines and naming conventions
- Uses idiomatic Rust patterns
- Is safe and handles errors properly
- Is well-documented and tested
- Is maintainable and performant
- Passes clippy and rustfmt checks

---

## When to Use This Skill

Use the code-review skill when:
- [ ] Reviewing newly written code before commit
- [ ] Checking code quality after implementation
- [ ] Evaluating pull request code
- [ ] Refactoring existing code
- [ ] Ensuring code meets project standards
- [ ] Verifying safety and correctness
- [ ] Optimizing performance
- [ ] Improving code maintainability

---

## Core Workflow

### Conducting a Code Review

1. **Understand the Purpose**
   - What is this code supposed to do?
   - Read the feature spec or issue
   - Understand the requirements

2. **Check Correctness**
   - Does it implement requirements correctly?
   - Are edge cases handled?
   - Is error handling appropriate?
   - Do tests pass?

3. **Review Code Quality**
   - Is it idiomatic Rust?
   - Follows naming conventions?
   - Appropriate use of types and traits?
   - Clean and readable?

4. **Check Safety & Performance**
   - No unnecessary unsafe code?
   - Proper error handling?
   - Efficient use of ownership?
   - No performance anti-patterns?

5. **Verify Documentation & Tests**
   - Public APIs documented?
   - Doc comments clear and helpful?
   - Adequate test coverage?
   - Examples provided?

6. **Run Tools**
   - `cargo clippy --all-targets --all-features`
   - `cargo fmt --check`
   - `cargo test`
   - `cargo doc --no-deps --open`

7. **Provide Feedback**
   - List issues found
   - Suggest improvements
   - Explain reasoning
   - Prioritize (critical vs. nice-to-have)

---

## Core Competencies

### 1. Code Correctness

**Functional Requirements**:
- Implements specified behavior
- Handles all required cases
- Edge cases considered
- Error conditions handled

**Logic Verification**:
- Correct algorithms
- Proper control flow
- No off-by-one errors
- Correct boundary conditions

**Test Coverage**:
- Unit tests for functions
- Integration tests for workflows
- Edge cases tested
- Error cases tested

### 2. Idiomatic Rust

**Naming Conventions**:
```rust
// Good
fn calculate_total(items: &[Item]) -> i32 { }
struct UserAccount { }
const MAX_RETRIES: u32 = 3;
type Result<T> = std::result::Result<T, Error>;

// Bad
fn CalculateTotal(items: &[Item]) -> i32 { }  // Wrong case
struct user_account { }  // Wrong case
const maxRetries: u32 = 3;  // Wrong case
```

**Ownership Patterns**:
```rust
// Good: Borrow when possible
fn process(data: &[u8]) -> Result<()> { }

// Bad: Taking ownership unnecessarily
fn process(data: Vec<u8>) -> Result<()> { }

// Good: &str for string parameters
fn greet(name: &str) -> String { }

// Bad: String for read-only parameter
fn greet(name: String) -> String { }
```

**Iterator Usage**:
```rust
// Good: Use iterator methods
let sum: i32 = items.iter()
    .filter(|x| x.is_valid())
    .map(|x| x.value)
    .sum();

// Bad: Manual loops
let mut sum = 0;
for item in &items {
    if item.is_valid() {
        sum += item.value;
    }
}
```

**Pattern Matching**:
```rust
// Good: Exhaustive matching
match result {
    Ok(value) => handle_value(value),
    Err(e) => handle_error(e),
}

// Bad: Unwrapping without reason
let value = result.unwrap();

// Good: if let for single case
if let Some(value) = option {
    process(value);
}

// Bad: match for single case
match option {
    Some(value) => process(value),
    None => {}
}
```

### 3. Error Handling

**Library Code**:
```rust
// Good: Return Result
pub fn parse(input: &str) -> Result<Ast, ParseError> {
    // ...
}

// Bad: Panic in library
pub fn parse(input: &str) -> Ast {
    input.parse().expect("parse failed")  // Don't do this
}
```

**Error Types**:
```rust
// Good: Custom error type with context
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unexpected token at line {line}: {token}")]
    UnexpectedToken { line: usize, token: String },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

// Bad: String errors
pub type Error = String;
```

**Error Propagation**:
```rust
// Good: Use ? operator
fn process_file(path: &Path) -> Result<Data> {
    let content = std::fs::read_to_string(path)?;
    let data = parse(&content)?;
    Ok(data)
}

// Bad: Unwrapping
fn process_file(path: &Path) -> Result<Data> {
    let content = std::fs::read_to_string(path).unwrap();
    let data = parse(&content).unwrap();
    Ok(data)
}
```

### 4. Documentation Quality

**Module Documentation**:
```rust
//! Parser module for AsciiDoc.
//!
//! This module provides the core parsing functionality,
//! converting AsciiDoc text into an Abstract Syntax Tree.
//!
//! # Examples
//!
//! ```
//! use doctora::parser::Parser;
//!
//! let parser = Parser::new();
//! let ast = parser.parse("= Title").unwrap();
//! ```
```

**Function Documentation**:
```rust
/// Parses AsciiDoc text into an AST.
///
/// Takes raw AsciiDoc input and produces a structured
/// representation suitable for processing.
///
/// # Arguments
///
/// * `input` - The AsciiDoc source text
///
/// # Returns
///
/// Returns `Ok(Ast)` on success, or `Err(ParseError)`
/// if the input is malformed.
///
/// # Examples
///
/// ```
/// # use doctora::parser::parse;
/// let ast = parse("= Document Title\n\nContent").unwrap();
/// ```
///
/// # Errors
///
/// Returns `ParseError::UnexpectedToken` if the syntax
/// is invalid.
pub fn parse(input: &str) -> Result<Ast, ParseError> {
    // ...
}
```

**Struct Documentation**:
```rust
/// Represents a section in the document.
///
/// Sections are hierarchical blocks that structure
/// the document content.
#[derive(Debug, Clone)]
pub struct Section {
    /// The section level (1-6).
    pub level: u8,

    /// The section title.
    pub title: String,

    /// Child blocks contained in this section.
    pub blocks: Vec<Block>,
}
```

### 5. Safety & Correctness

**Avoiding Unsafe**:
```rust
// Good: Safe alternative
fn get_value(slice: &[i32], index: usize) -> Option<i32> {
    slice.get(index).copied()
}

// Bad: Unnecessary unsafe
fn get_value(slice: &[i32], index: usize) -> i32 {
    unsafe { *slice.get_unchecked(index) }
}
```

**Proper Bounds Checking**:
```rust
// Good: Safe indexing
if index < vec.len() {
    process(vec[index]);
}

// Or use get
if let Some(value) = vec.get(index) {
    process(*value);
}

// Bad: Assuming bounds
process(vec[index]);  // May panic
```

**Thread Safety**:
```rust
// Good: Proper Send/Sync bounds
fn spawn_processor<T: Send + 'static>(data: T) {
    std::thread::spawn(move || {
        process(data);
    });
}

// Check for data races with Arc and Mutex
use std::sync::{Arc, Mutex};

let shared = Arc::new(Mutex::new(data));
```

### 6. Performance Considerations

**Avoiding Unnecessary Allocations**:
```rust
// Good: Borrow and iterate
fn count_valid(items: &[Item]) -> usize {
    items.iter().filter(|x| x.is_valid()).count()
}

// Bad: Clone entire collection
fn count_valid(items: &[Item]) -> usize {
    items.to_vec().iter().filter(|x| x.is_valid()).count()
}
```

**Efficient String Handling**:
```rust
// Good: Use &str and avoid allocations
fn format_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Bad: Unnecessary String parameter
fn format_greeting(name: String) -> String {
    format!("Hello, {}!", name)
}
```

**Using Appropriate Data Structures**:
```rust
// Good: HashMap for O(1) lookup
use std::collections::HashMap;
let mut map = HashMap::new();

// Bad: Vec for frequent lookups (O(n))
let mut vec = Vec::new();  // Then searching with .iter().find()
```

**Iterator Efficiency**:
```rust
// Good: Iterator chains (lazy, no intermediate allocations)
let result: Vec<_> = data.iter()
    .filter(|x| x.is_valid())
    .map(|x| x.value * 2)
    .collect();

// Less efficient: Multiple passes with intermediate collections
let filtered: Vec<_> = data.iter()
    .filter(|x| x.is_valid())
    .collect();
let result: Vec<_> = filtered.iter()
    .map(|x| x.value * 2)
    .collect();
```

---

## Review Checklist

### Correctness
- [ ] Implements specified requirements
- [ ] Handles all edge cases
- [ ] Proper error handling
- [ ] Tests pass
- [ ] Logic is sound

### Code Quality
- [ ] Follows Rust naming conventions
- [ ] Uses idiomatic patterns
- [ ] Clean and readable
- [ ] Appropriate abstractions
- [ ] No code duplication

### Safety
- [ ] No unnecessary unsafe code
- [ ] Proper bounds checking
- [ ] No potential panics in library code
- [ ] Thread-safe if concurrent
- [ ] No memory leaks

### Error Handling
- [ ] Uses Result for recoverable errors
- [ ] Custom error types when appropriate
- [ ] Errors have context
- [ ] No unwrap() in library code
- [ ] Error propagation with ?

### Performance
- [ ] No unnecessary allocations
- [ ] Efficient data structures
- [ ] Proper use of borrowing
- [ ] Iterators used appropriately
- [ ] No obvious bottlenecks

### Documentation
- [ ] Public APIs documented
- [ ] Examples in doc comments
- [ ] Module-level documentation
- [ ] Complex logic explained
- [ ] Errors and panics documented

### Testing
- [ ] Unit tests for functions
- [ ] Integration tests for workflows
- [ ] Edge cases tested
- [ ] Error cases tested
- [ ] Doc tests for examples

### Tooling
- [ ] cargo clippy passes
- [ ] cargo fmt applied
- [ ] cargo test passes
- [ ] cargo doc builds cleanly

---

## Common Issues to Look For

### Anti-Patterns

**Excessive Cloning**:
```rust
// Bad
fn process_items(items: Vec<Item>) -> Vec<String> {
    items.iter()
        .map(|item| item.clone())  // Unnecessary clone
        .map(|item| item.to_string())
        .collect()
}

// Good
fn process_items(items: &[Item]) -> Vec<String> {
    items.iter()
        .map(|item| item.to_string())
        .collect()
}
```

**String Concatenation in Loops**:
```rust
// Bad: Inefficient
let mut result = String::new();
for item in &items {
    result = result + &item.to_string();  // Allocates each time
}

// Good: Use a buffer
let mut result = String::new();
for item in &items {
    result.push_str(&item.to_string());
}

// Better: Use iterators
let result = items.iter()
    .map(|item| item.to_string())
    .collect::<Vec<_>>()
    .join("");
```

**Not Using Entry API**:
```rust
// Bad
if !map.contains_key(&key) {
    map.insert(key, vec![]);
}
map.get_mut(&key).unwrap().push(value);

// Good
map.entry(key).or_insert_with(Vec::new).push(value);
```

**Unnecessary Option/Result Conversions**:
```rust
// Bad
if option.is_some() {
    let value = option.unwrap();
    process(value);
}

// Good
if let Some(value) = option {
    process(value);
}
```

### Code Smells

- Functions > 50 lines (consider breaking up)
- Deep nesting (> 3-4 levels)
- Many function parameters (> 4-5, use struct)
- Commented-out code
- Magic numbers (use constants)
- Unclear variable names
- Lack of tests
- Missing error handling
- Overly complex logic

---

## Review Feedback Guidelines

### Providing Constructive Feedback

**Be Specific**:
```
Bad: "This code is not idiomatic."
Good: "Consider using iterator methods instead of a manual loop at line 42.
       This is more idiomatic Rust and can be more efficient."
```

**Explain Why**:
```
Bad: "Use &str here instead of String."
Good: "Use &str instead of String for the parameter since we only read
       the value. This avoids an unnecessary allocation and follows Rust
       API guidelines."
```

**Prioritize Issues**:
- **Critical**: Correctness bugs, safety issues, panics in library code
- **High**: Performance problems, poor error handling, missing tests
- **Medium**: Non-idiomatic code, missing documentation
- **Low**: Style preferences, minor optimizations

**Suggest Solutions**:
```
Issue: "This function could panic if the index is out of bounds."
Solution: "Consider using .get(index) which returns Option<&T> instead
          of panicking. Then handle the None case appropriately."
```

**Be Positive**:
- Acknowledge good code
- Phrase suggestions constructively
- Focus on improvement, not criticism

---

## Tools for Code Review

### Clippy
```bash
# Run clippy with all checks
cargo clippy --all-targets --all-features

# Run clippy with pedantic checks
cargo clippy --all-targets --all-features -- -W clippy::pedantic

# Run clippy with specific denies
cargo clippy -- -D warnings
```

**Common Clippy Lints**:
- `clippy::needless_return`: Remove unnecessary returns
- `clippy::redundant_clone`: Remove unnecessary clones
- `clippy::use_self`: Use `Self` instead of type name
- `clippy::wildcard_imports`: Avoid `use module::*`
- `clippy::missing_errors_doc`: Document error cases

### Rustfmt
```bash
# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt
```

### Documentation
```bash
# Build and view documentation
cargo doc --no-deps --open

# Check for documentation warnings
cargo doc --no-deps 2>&1 | grep warning
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run doc tests only
cargo test --doc
```

---

## Examples

### Example Review 1: Error Handling

**Code**:
```rust
pub fn load_config(path: &Path) -> Config {
    let content = std::fs::read_to_string(path).unwrap();
    let config: Config = serde_json::from_str(&content).unwrap();
    config
}
```

**Review Feedback**:
- **Critical**: Function panics on error, should return `Result`
- **Issue**: No context in errors
- **Suggestion**:
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse config: {0}")]
    Parse(#[from] serde_json::Error),
}

pub fn load_config(path: &Path) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}
```

### Example Review 2: Performance

**Code**:
```rust
pub fn find_duplicates(items: Vec<String>) -> Vec<String> {
    let mut duplicates = Vec::new();
    for i in 0..items.len() {
        for j in (i+1)..items.len() {
            if items[i] == items[j] && !duplicates.contains(&items[i]) {
                duplicates.push(items[i].clone());
            }
        }
    }
    duplicates
}
```

**Review Feedback**:
- **High**: O(n²) algorithm, use HashSet for O(n)
- **Medium**: Takes ownership unnecessarily
- **Medium**: Unnecessary clones
- **Suggestion**:
```rust
use std::collections::HashSet;

pub fn find_duplicates(items: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();

    for item in items {
        if !seen.insert(item) {
            duplicates.insert(item.clone());
        }
    }

    duplicates.into_iter().cloned().collect()
}
```

---

## Lessons Learned

### Effective Review Practices
*To be filled with patterns for effective code review.*

### Common Issues Found
*To be filled with frequently found issues in reviews.*

### Quality Improvements
*To be filled with improvements that resulted from reviews.*

### Review Patterns
*To be filled with repeating patterns in code that need attention.*

---

## Continuous Improvement

After each code review:
1. Note recurring issues
2. Update "Common Issues" section
3. Document new patterns discovered
4. Share learning with team
5. Update review checklist if needed

This skill becomes more valuable as it accumulates experience reviewing code and builds a library of common issues and solutions.
