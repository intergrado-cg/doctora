---
name: debugging
description: Expert skill for debugging Rust compiler errors, borrow checker issues, runtime errors, and finding solutions through online research. Specializes in understanding error messages, identifying root causes, and applying fixes.
---

# debugging Skill

## Primary Responsibility

**Debug and resolve Rust compilation errors, runtime issues, and logic bugs efficiently.**

Expertise in:
- Reading and understanding Rust compiler error messages
- Resolving borrow checker errors
- Debugging type system issues
- Finding solutions through online research
- Identifying root causes systematically
- Applying fixes that address the underlying problem

---

## When to Use This Skill

Use the debugging skill when:
- [ ] Code fails to compile with compiler errors
- [ ] Encountering borrow checker errors
- [ ] Facing type system errors (trait bounds, lifetimes, etc.)
- [ ] Tests fail with runtime errors
- [ ] Logic bugs in implementation
- [ ] Performance issues need investigation
- [ ] Cryptic error messages need clarification
- [ ] Need to research solutions online

---

## Core Workflow

### Debugging a Compiler Error

1. **Read the Error Message Carefully**
   - Note the error code (E0xxx)
   - Read the main error message
   - Examine the context (file, line, column)
   - Look at the suggestion (if provided)

2. **Understand the Context**
   - What is the code trying to do?
   - What types are involved?
   - What is the ownership flow?

3. **Get More Information**
   - Use `rustc --explain E0xxx` for detailed explanation
   - Check if there are multiple related errors
   - Read the full error chain

4. **Research if Needed**
   - Search error message on docs.rs
   - Look for similar issues on Stack Overflow
   - Check Rust forums or r/rust
   - Read relevant documentation sections

5. **Apply Fix**
   - Implement the solution
   - Verify with `cargo check`
   - Run tests to ensure correctness

6. **Document Learning**
   - Add entry to experience log
   - Note error pattern and solution
   - Link to helpful resources

---

## Core Competencies

### 1. Understanding Compiler Errors

**Common Error Categories**:

**E0382 - Use of moved value**:
```rust
// Error example
let s = String::from("hello");
let s2 = s;  // s moved here
println!("{}", s);  // Error: s already moved

// Fix: Clone or borrow
let s = String::from("hello");
let s2 = s.clone();  // Clone instead of move
println!("{}", s);  // OK

// Or use borrowing
let s = String::from("hello");
let s2 = &s;  // Borrow instead of move
println!("{}", s);  // OK
```

**E0502 - Cannot borrow as mutable/immutable**:
```rust
// Error example
let mut v = vec![1, 2, 3];
let first = &v[0];  // Immutable borrow
v.push(4);  // Error: cannot borrow as mutable

// Fix: Limit borrow scope
let mut v = vec![1, 2, 3];
{
    let first = &v[0];
    println!("{}", first);
}  // Immutable borrow ends here
v.push(4);  // OK
```

**E0106 - Missing lifetime specifier**:
```rust
// Error example
fn first_word(s: &str) -> &str {  // Error: missing lifetime
    // ...
}

// Fix: Add lifetime parameter
fn first_word<'a>(s: &'a str) -> &'a str {
    // ...
}

// Or let compiler infer (lifetime elision)
fn first_word(s: &str) -> &str {  // OK with elision rules
    // ...
}
```

**E0277 - Trait not implemented**:
```rust
// Error example
fn print_it<T>(item: T) {
    println!("{}", item);  // Error: T doesn't implement Display
}

// Fix: Add trait bound
fn print_it<T: std::fmt::Display>(item: T) {
    println!("{}", item);  // OK
}
```

### 2. Borrow Checker Issues

**Understanding Borrowing Rules**:
1. Any number of immutable borrows OR one mutable borrow
2. References must always be valid
3. No dangling references

**Common Patterns**:

**Multiple Mutable Borrows**:
```rust
// Problem
let mut v = vec![1, 2, 3];
let a = &mut v[0];
let b = &mut v[1];  // Error: multiple mutable borrows

// Solution: Use split_at_mut or indexing
let mut v = vec![1, 2, 3];
let (left, right) = v.split_at_mut(1);
let a = &mut left[0];
let b = &mut right[0];  // OK
```

**Returning Borrowed Data**:
```rust
// Problem
fn get_default() -> &str {
    let s = String::from("default");
    &s  // Error: returns reference to local variable
}

// Solution 1: Return owned data
fn get_default() -> String {
    String::from("default")  // OK
}

// Solution 2: Use static
fn get_default() -> &'static str {
    "default"  // OK: string literal has static lifetime
}
```

**Iterator Invalidation**:
```rust
// Problem
let mut v = vec![1, 2, 3];
for item in &v {
    v.push(*item);  // Error: cannot modify while iterating
}

// Solution: Collect indices first
let mut v = vec![1, 2, 3];
let indices: Vec<_> = (0..v.len()).collect();
for i in indices {
    v.push(v[i]);  // OK
}
```

### 3. Type System Issues

**Lifetime Errors**:
- Understand lifetime parameters
- Know when to use explicit lifetimes
- Understand lifetime elision rules
- Use named lifetimes for clarity

**Trait Bound Errors**:
- Add required trait bounds to generics
- Understand trait object requirements (Sized, object-safe)
- Use where clauses for complex bounds
- Check trait implementations

**Type Inference Failures**:
- Add type annotations where needed
- Use turbofish syntax (`::<T>`)
- Help compiler with intermediate types
- Check generic parameter inference

### 4. Runtime Debugging

**Debugging Tools**:
- **println! debugging**: Quick inspection
- **dbg! macro**: Print value and return it
- **rust-gdb/rust-lldb**: Full debuggers
- **Logging**: tracing, log crates
- **Backtrace**: RUST_BACKTRACE=1

**Common Runtime Issues**:

**Panics**:
```rust
// Panic source
let v = vec![1, 2, 3];
let x = v[10];  // Panic: index out of bounds

// Fix: Use safe access
let v = vec![1, 2, 3];
let x = v.get(10).unwrap_or(&0);  // OK
```

**Unwrap on None/Err**:
```rust
// Panic source
let result: Option<i32> = None;
let value = result.unwrap();  // Panic

// Fix: Handle properly
let result: Option<i32> = None;
let value = result.unwrap_or(0);  // OK
// Or
match result {
    Some(v) => v,
    None => 0,
}
```

### 5. Online Research Skills

**Where to Search**:
1. **docs.rs**: Crate documentation, examples
2. **Stack Overflow**: Tag [rust], copy exact error message
3. **r/rust**: Community advice, "How do I..." questions
4. **Rust Users Forum**: More detailed discussions
5. **GitHub Issues**: Crate-specific problems
6. **Rust Error Index**: https://doc.rust-lang.org/error-index.html

**Search Strategies**:
- Copy exact error message in quotes
- Include "rust" in search query
- Look for recent answers (Rust evolves quickly)
- Check multiple sources
- Verify solution applies to your Rust version

**Evaluating Solutions**:
- Does it address the root cause?
- Is it idiomatic Rust?
- Does it introduce other issues?
- Is it well-explained?
- Does it have good community feedback?

---

## Best Practices

### Debugging Process

1. **Read Error Messages Fully**
   - Don't skim, read completely
   - Understand the suggestion
   - Check related errors

2. **Understand Before Fixing**
   - Know why the error occurs
   - Don't blindly apply fixes
   - Learn the underlying concept

3. **Fix Root Cause**
   - Don't just make compiler happy
   - Address the actual problem
   - Ensure fix is correct

4. **Test the Fix**
   - Verify error is gone
   - Run tests
   - Check for new errors
   - Ensure logic is correct

5. **Document Learning**
   - Record error and solution
   - Link to resources
   - Note patterns for future

### Common Debugging Patterns

**Working Backward from Error**:
- Start at error location
- Trace data flow backward
- Find where assumption breaks
- Fix at the source

**Minimal Reproduction**:
- Isolate the problem
- Create minimal example
- Easier to understand
- Easier to search for help

**Incremental Verification**:
- Add assertions
- Use dbg! macro
- Print intermediate values
- Verify assumptions

**Rubber Duck Debugging**:
- Explain code to someone (or something)
- Forces clear thinking
- Often reveals the issue

---

## Common Error Patterns

### Ownership Errors

**Pattern**: Moving value when borrowing intended
**Symptom**: "value moved here", "use of moved value"
**Solution**: Use `&` to borrow instead of move, or `.clone()` if ownership needed
**Resource**: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

**Pattern**: Trying to modify borrowed data
**Symptom**: "cannot assign to `*x` which is behind a `&` reference"
**Solution**: Use `&mut` for mutable borrow
**Resource**: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

### Lifetime Errors

**Pattern**: Returning reference to local variable
**Symptom**: "returns a reference to data owned by the current function"
**Solution**: Return owned data or use static lifetime
**Resource**: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

**Pattern**: Conflicting lifetime requirements
**Symptom**: "lifetime may not live long enough"
**Solution**: Adjust lifetime parameters or restructure code
**Resource**: https://doc.rust-lang.org/nomicon/lifetimes.html

### Type Errors

**Pattern**: Missing trait bound
**Symptom**: "the trait `X` is not implemented for `Y`"
**Solution**: Add trait bound or implement trait
**Resource**: https://doc.rust-lang.org/book/ch10-02-traits.html

**Pattern**: Type mismatch
**Symptom**: "expected type `X`, found type `Y`"
**Solution**: Convert between types or fix logic
**Resource**: Use From/Into traits for conversions

### Borrow Checker Errors

**Pattern**: Simultaneous mutable and immutable borrows
**Symptom**: "cannot borrow as mutable because it is also borrowed as immutable"
**Solution**: Limit borrow scopes or restructure code
**Resource**: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

**Pattern**: Borrowing moved value
**Symptom**: "borrow of moved value"
**Solution**: Clone before move or restructure to avoid move
**Resource**: Rust Book ownership chapter

---

## Debugging Techniques by Error Type

### Compiler Errors

1. **Read `rustc --explain E0xxx`**
2. **Check compiler suggestions**
3. **Examine error context**
4. **Search online if unclear**
5. **Apply fix**
6. **Verify with `cargo check`**

### Test Failures

1. **Read assertion message**
2. **Check expected vs. actual**
3. **Add debug prints**
4. **Run single test: `cargo test test_name`**
5. **Use `dbg!()` in test**
6. **Fix and verify**

### Logic Bugs

1. **Add debug prints**
2. **Check assumptions**
3. **Trace execution flow**
4. **Use debugger if needed**
5. **Write failing test**
6. **Fix and verify test passes**

### Performance Issues

1. **Measure with benchmarks**
2. **Profile with tools**
3. **Identify bottlenecks**
4. **Optimize hot paths**
5. **Verify improvement**

---

## Research Resources

### Official Resources
- **Error Index**: https://doc.rust-lang.org/error-index.html
- **Rust Book**: https://doc.rust-lang.org/book/
- **Rustc Error Explanations**: Use `rustc --explain E0xxx`
- **Nomicon** (advanced): https://doc.rust-lang.org/nomicon/

### Community Resources
- **Stack Overflow**: Tag [rust]
- **r/rust**: Reddit community
- **Rust Users Forum**: users.rust-lang.org
- **Rust Discord**: Community chat

### Debugging Tools
- **rust-gdb**: GDB with Rust support
- **rust-lldb**: LLDB with Rust support
- **cargo-watch**: Auto-rebuild on changes
- **cargo-expand**: See macro expansions

---

## Examples

### Example 1: Fixing Borrow Checker Error

**Error**:
```rust
fn process_data(data: &mut Vec<i32>) {
    let first = &data[0];
    data.push(10);  // Error: cannot borrow as mutable
    println!("{}", first);
}
```

**Debugging Process**:
1. Error: "cannot borrow `*data` as mutable because it is also borrowed as immutable"
2. Identify: `first` holds immutable borrow, `push` needs mutable borrow
3. Solution: Limit scope of immutable borrow

**Fix**:
```rust
fn process_data(data: &mut Vec<i32>) {
    let first = data[0];  // Copy value instead of borrowing
    data.push(10);  // OK: no active borrows
    println!("{}", first);
}
```

### Example 2: Lifetime Error Resolution

**Error**:
```rust
fn longest(x: &str, y: &str) -> &str {  // Error: missing lifetime
    if x.len() > y.len() { x } else { y }
}
```

**Debugging Process**:
1. Error: "missing lifetime specifier"
2. Explanation: Compiler doesn't know which lifetime to use for return
3. Solution: Add lifetime parameter

**Fix**:
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## Lessons Learned

### Effective Patterns
*To be filled with patterns that successfully resolve errors.*

### Ineffective Approaches
*To be filled with debugging approaches that didn't work well.*

### Crate-Specific Issues
*To be filled with common issues encountered with specific crates.*

### Common Pitfalls
*To be filled with frequently encountered mistakes and how to avoid them.*

---

## Continuous Improvement

After each debugging session:
1. Document the error and solution in experience log
2. Note if it's a recurring pattern
3. Update "Common Error Patterns" if new pattern found
4. Link to useful resources discovered
5. Extract general lessons for future debugging

This skill becomes more powerful as it accumulates real debugging experience and builds a library of solutions.
