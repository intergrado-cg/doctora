---
name: feature-design
description: PRIMARY RESPONSIBILITY - Create and update feature specification files in docs/design/features/ using the template from docs/design/templates/feature-template.md. Also provides expert knowledge of requirements gathering, technical design, architecture patterns, API design, and research. Use when creating new feature specs, updating existing specs, or designing technical solutions.
---

# Feature Design Skill

**PRIMARY RESPONSIBILITY**: Create and update feature specification files in `docs/design/features/` using the standardized template.

Expert skill for designing software features, writing detailed technical specifications, and conducting research to inform design decisions.

---

## PRIMARY WORKFLOW: Creating and Updating Feature Specifications

### When to Use This Skill

**ALWAYS use this skill when you need to**:
1. **Create a new feature specification**
2. **Update an existing feature specification**
3. **Research requirements for a feature**
4. **Design technical architecture for a feature**

### Feature Spec Workflow

#### Creating a New Feature Spec

1. **Read the Template**
   ```
   Read: docs/design/templates/feature-template.md
   ```

2. **Research the Feature**
   - Use WebSearch/WebFetch to research requirements
   - Look up existing solutions and best practices
   - Review relevant documentation
   - Document findings in "Research References" section

3. **Create the Feature File**
   ```
   Write: docs/design/features/[feature-name].md
   ```
   - Use kebab-case for filename (e.g., `core-parser.md`, `html-processor.md`)
   - Copy template structure exactly
   - Fill in ALL sections of the template

4. **Complete All Sections**
   - **Overview**: 2-3 sentence summary
   - **Status**: Set to "Draft" initially
   - **Business Value**: Why this feature matters
   - **User Stories**: As a [user]... I want... so that...
   - **Requirements**: Functional and non-functional
   - **Technical Design**: Architecture, components, data model, API
   - **Dependencies**: What this needs and what needs this
   - **Implementation Plan**: Specific tasks with checkboxes
   - **Testing Strategy**: How to verify it works
   - **Edge Cases**: Unusual scenarios to handle
   - **Alternatives Considered**: Other approaches and why not chosen
   - **Research References**: Links to documentation, examples
   - **Open Questions**: Things still to decide

5. **Verify Completeness**
   - All template sections filled in
   - No [placeholder] text remaining
   - Technical design is detailed enough to implement
   - Tasks are specific and actionable

#### Updating an Existing Feature Spec

1. **Read the Current Spec**
   ```
   Read: docs/design/features/[feature-name].md
   ```

2. **Make Updates Using Edit Tool**
   - Update **Status** field (Draft → In Review → Approved → In Progress → Completed)
   - Add new **Requirements** if discovered
   - Refine **Technical Design** based on learnings
   - Update **Implementation Plan** tasks
   - Add to **Change Log** section with date
   - Resolve **Open Questions** as decisions are made

3. **Update Last Updated Date**
   ```
   - **Last Updated**: YYYY-MM-DD
   ```

4. **Document Changes in Change Log**
   ```markdown
   ## Change Log

   ### YYYY-MM-DD
   - Updated technical design based on prototype
   - Resolved open question about data model
   - Added new requirement for error handling
   ```

### File Locations

- **Template**: `docs/design/templates/feature-template.md` (DO NOT MODIFY - use as reference)
- **Feature Specs**: `docs/design/features/[feature-name].md` (CREATE/UPDATE these)

### Quality Standards

Every feature spec must have:
- ✅ Clear, concise overview
- ✅ Complete requirements (functional and non-functional)
- ✅ Detailed technical design with code examples
- ✅ Specific, actionable tasks
- ✅ Defined acceptance criteria
- ✅ Research references and justification
- ✅ Edge cases identified
- ✅ Alternatives considered with rationale

---

## Core Competencies

### Requirements Analysis
- Gather and analyze user requirements
- Identify functional and non-functional requirements
- Define acceptance criteria
- Prioritize features and requirements
- Identify edge cases and constraints

### Feature Specification
- Write clear, detailed feature specifications
- Define user stories and use cases
- Create technical design documents
- Document API contracts and interfaces
- Specify data models and schemas

### System Architecture
- Design system components and their interactions
- Choose appropriate architecture patterns (layered, microservices, plugin-based)
- Define module boundaries and responsibilities
- Plan for scalability and extensibility
- Design data flow and processing pipelines

### Technical Design Patterns
- Apply appropriate design patterns (Strategy, Factory, Observer, etc.)
- Design plugin/extension systems
- Create flexible, maintainable architectures
- Implement separation of concerns
- Design for testability

### API Design
- Design RESTful and GraphQL APIs
- Create clear API contracts
- Define request/response formats
- Plan versioning strategies
- Document API endpoints and parameters

### Research and Best Practices
- Research existing solutions and approaches
- Analyze competitor features
- Review industry best practices
- Stay current with technology trends
- Learn from open-source projects

## Feature Specification Template

### Standard Feature Specification Format

```markdown
# Feature: [Feature Name]

## Overview
Brief description of the feature and its purpose.

## Status
- **Status**: [Draft/In Review/Approved/In Progress/Completed]
- **Priority**: [Critical/High/Medium/Low]
- **Effort**: [Small/Medium/Large/X-Large]
- **Target Version**: [Version number]

## Business Value
Why this feature matters and what problems it solves.

## User Stories
- As a [user type], I want [goal] so that [benefit]
- As a [user type], I want [goal] so that [benefit]

## Requirements

### Functional Requirements
1. The system shall...
2. The system shall...

### Non-Functional Requirements
- **Performance**: [Requirements]
- **Security**: [Requirements]
- **Scalability**: [Requirements]
- **Usability**: [Requirements]

## Technical Design

### Architecture
Description of how this feature fits into the overall system architecture.

### Components
- **Component 1**: Description and responsibility
- **Component 2**: Description and responsibility

### Data Model
```
[Schema/Structure definitions]
```

### API Design
```
[API contracts, endpoints, interfaces]
```

### Processing Flow
1. Step 1
2. Step 2
3. Step 3

## Dependencies
- Depends on: [Other features/libraries/systems]
- Blocks: [Features that depend on this]

## Implementation Plan

### Tasks
- [ ] Task 1
- [ ] Task 2
- [ ] Task 3

### Testing Strategy
- Unit tests for [components]
- Integration tests for [workflows]
- Edge cases to test

## Edge Cases and Considerations
- Edge case 1: [Description and handling]
- Edge case 2: [Description and handling]

## Alternatives Considered
- **Alternative 1**: [Description, pros/cons, why not chosen]
- **Alternative 2**: [Description, pros/cons, why not chosen]

## References
- [Link to research]
- [Link to documentation]
- [Link to similar implementations]

## Open Questions
- Question 1?
- Question 2?
```

## Architecture Documentation Guidelines

### System Architecture Document Structure

```markdown
# System Architecture

## System Overview
High-level description of the system.

## Architecture Style
[Layered/Microservices/Event-driven/Plugin-based/etc.]

## Core Components
### Component 1
- **Responsibility**:
- **Dependencies**:
- **Interfaces**:

### Component 2
- **Responsibility**:
- **Dependencies**:
- **Interfaces**:

## Data Flow
Description of how data flows through the system.

## Extension Points
How the system can be extended or customized.

## Technology Stack
- Language:
- Frameworks:
- Libraries:
- Tools:

## Design Principles
- Principle 1
- Principle 2

## Architectural Decisions
### ADR 1: [Decision Title]
- **Status**: [Accepted/Rejected/Superseded]
- **Context**: Why this decision was needed
- **Decision**: What was decided
- **Consequences**: Impact of the decision
```

## Best Practices

### Writing Specifications
- **Be Specific**: Avoid ambiguity, use concrete examples
- **Be Complete**: Cover all scenarios including edge cases
- **Be Testable**: Include clear acceptance criteria
- **Be Maintainable**: Keep documentation up to date
- **Be Visual**: Use diagrams where helpful

### Research Process
1. **Understand the Problem**: Clearly define what needs to be solved
2. **Survey Existing Solutions**: Research how others solve similar problems
3. **Evaluate Options**: Compare approaches, pros/cons
4. **Adapt to Context**: Consider project-specific constraints
5. **Document Findings**: Record research and decisions

### Design Principles
- **SOLID Principles**: Single responsibility, Open/closed, Liskov substitution, Interface segregation, Dependency inversion
- **DRY**: Don't Repeat Yourself
- **KISS**: Keep It Simple, Stupid
- **YAGNI**: You Aren't Gonna Need It
- **Separation of Concerns**: Organize code by responsibility
- **Composition over Inheritance**: Prefer composing behaviors

## Common Patterns

### Parser Design Patterns
- **Recursive Descent Parser**: Top-down parsing approach
- **Parser Combinator**: Compose small parsers into larger ones
- **Visitor Pattern**: Traverse and operate on AST nodes
- **Builder Pattern**: Construct complex objects step by step

### Processor Architecture Patterns
- **Pipeline Pattern**: Chain of processing steps
- **Strategy Pattern**: Swappable processing algorithms
- **Chain of Responsibility**: Pass requests through handler chain
- **Template Method**: Define algorithm skeleton, customize steps

### Extension System Patterns
- **Plugin Architecture**: Load and execute external modules
- **Registry Pattern**: Central registry of available extensions
- **Service Locator**: Discover and access services dynamically
- **Dependency Injection**: Inject dependencies at runtime

## Research Resources

### General Software Design
- Martin Fowler's Blog (martinfowler.com)
- Refactoring Guru (refactoring.guru)
- Architecture Decision Records (adr.github.io)
- The C4 Model for software architecture

### Language-Specific (Rust)
- Rust API Guidelines
- Rust Design Patterns book
- Rust Performance book
- Awesome Rust repositories

### Parser/Compiler Design
- Crafting Interpreters (craftinginterpreters.com)
- Dragon Book (Compilers: Principles, Techniques, and Tools)
- ANTLR and parser generator documentation
- Pest parser documentation (for Rust)

### Documentation Formats
- AsciiDoc specification
- Markdown specifications
- ReStructuredText documentation
- LaTeX documentation

## Examples

### Example 1: Plugin System Design
```markdown
## Plugin System Architecture

### Overview
Modular plugin system allowing third-party processors.

### Design
- **Plugin Interface**: Define trait for all processors
- **Plugin Registry**: Central registration of available plugins
- **Plugin Loader**: Dynamic loading of plugin modules
- **Plugin Chain**: Execute plugins in sequence or tree

### API
```rust
pub trait Processor {
    fn name(&self) -> &str;
    fn process(&self, input: &Document) -> Result<Output>;
    fn can_handle(&self, format: &str) -> bool;
}
```
```

### Example 2: AST Design
```markdown
## Abstract Syntax Tree Design

### Node Types
- **Block**: Section, paragraph, list, table
- **Inline**: Text, emphasis, link, image
- **Metadata**: Title, author, attributes

### Structure
```rust
pub enum Node {
    Block(BlockNode),
    Inline(InlineNode),
    Text(String),
}

pub struct Document {
    metadata: HashMap<String, String>,
    nodes: Vec<Node>,
}
```
```

## When to Use This Skill

**PRIMARY USE CASES** (feature specification files):
- ✅ Creating new feature specification files in `docs/design/features/`
- ✅ Updating existing feature specification files
- ✅ Researching requirements for a feature spec
- ✅ Designing technical architecture for a feature spec

**SUPPORTING USE CASES** (general design knowledge):
- Evaluating design alternatives and documenting decisions
- Researching best practices and existing solutions
- Creating API designs and data models
- Planning system architecture patterns
- Defining acceptance criteria and testing strategies
- Breaking down features into implementation tasks

## Lessons Learned

This section contains real-world insights from feature design experience. The doctora-architect agent updates this section when discovering valuable design patterns or approaches.

---

*This section will grow as features are designed and lessons are learned.*

## Common Design Challenges

### Challenge: Over-engineering vs Under-engineering
**Balance**: Design for current needs plus one level of abstraction
**Approach**:
- Start with simplest solution that works
- Add abstraction when you see patterns emerge
- Refactor when you have 3+ similar use cases

### Challenge: Plugin vs Monolithic Design
**Decision Factors**:
- Need for third-party extensions?
- Complexity of plugin interface?
- Performance overhead acceptable?
- Maintenance burden?

### Challenge: Performance vs Readability
**Guideline**:
- Write readable code first
- Profile to find bottlenecks
- Optimize hot paths only
- Document performance-critical sections

---

*Add new challenges and solutions here as they are encountered.*
