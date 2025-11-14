---
name: project-management
description: PRIMARY RESPONSIBILITY - Edit and update the docs/design/TODO.md file to track what's done and not done. Mark tasks as complete with checkboxes, update feature status, track progress, and manage milestones. Also provides expert knowledge of task breakdown, effort estimation, and project planning. Use when updating TODO.md, tracking progress, or managing project tasks.
---

# Project Management Skill

**PRIMARY RESPONSIBILITY**: Edit and update `docs/design/TODO.md` to track what's done and what's not done.

Expert skill for managing project tasks, tracking completion status, and maintaining accurate progress records.

---

## PRIMARY WORKFLOW: Updating the TODO.md File

### When to Use This Skill

**ALWAYS use this skill when you need to**:
1. **Mark tasks as complete** (check off checkboxes)
2. **Add new tasks** to features
3. **Update feature status** (Not Started → In Progress → Completed)
4. **Update progress metrics** (completion percentages)
5. **Add new features** to milestones
6. **Track what's done and what's not done**

### TODO.md Update Workflow

#### Marking Tasks as Complete

1. **Read the Current TODO.md**
   ```
   Read: docs/design/TODO.md
   ```

2. **Identify Completed Work**
   - Which tasks have been finished?
   - Which features are now complete?
   - What progress has been made?

3. **Update Checkboxes Using Edit Tool**
   ```markdown
   # Change from:
   - [ ] Task 1: Implement core parser

   # To:
   - [x] Task 1: Implement core parser ✅
   ```

4. **Update Feature Status**
   ```markdown
   # Change from:
   **Status**: 📋 Not Started

   # To:
   **Status**: 🔄 In Progress (2/5 tasks completed)

   # Or if all tasks complete:
   **Status**: ✅ Completed (2024-11-14)
   ```

5. **Update Summary Statistics**
   ```markdown
   ## Summary
   - **Total Features**: X
   - **Completed Features**: Y (Z%)
   - **Total Tasks**: A
   - **Completed Tasks**: B (C%)
   ```

6. **Update Last Updated Date**
   ```markdown
   **Last Updated**: YYYY-MM-DD
   ```

#### Adding New Tasks to Features

1. **Read Current TODO.md**
   ```
   Read: docs/design/TODO.md
   ```

2. **Locate the Feature Section**
   - Find the feature under the appropriate milestone
   - Identify where to add tasks

3. **Add Tasks with Checkboxes**
   ```markdown
   ### Feature: Core Parser
   **Status**: 🔄 In Progress

   - [x] Design AST structure ✅
   - [x] Implement lexer ✅
   - [ ] Implement parser        ← NEW TASK
   - [ ] Add error handling      ← NEW TASK
   - [ ] Write unit tests        ← NEW TASK
   ```

4. **Update Feature Status if Needed**
   - Update task count in status: `(2/5 tasks completed)`
   - Update effort estimate if significantly changed

#### Adding New Features

1. **Read Current TODO.md**
   ```
   Read: docs/design/TODO.md
   ```

2. **Determine Milestone Placement**
   - Which milestone does this feature belong to?
   - What's the priority?

3. **Add Feature Section**
   ```markdown
   ### Feature: [Feature Name]
   **Status**: 📋 Not Started
   **Priority**: [Critical/High/Medium/Low]
   **Effort**: [Small/Medium/Large/X-Large]

   - [ ] Task 1: Description
   - [ ] Task 2: Description
   - [ ] Task 3: Description
   ```

4. **Update Milestone Status**
   - Update total feature count for milestone

#### Moving Features to Completed Section

1. **When All Tasks Are Done**
   ```markdown
   ### Feature: Project Setup
   **Status**: ✅ Completed (2024-11-14)

   - [x] Create repository ✅
   - [x] Setup documentation ✅
   - [x] Create agents ✅
   ```

2. **Cut from Current Location and Paste**
   - Remove from active milestone section
   - Add to "## Completed Features ✅" section at bottom

3. **Update Summary Counts**
   - Increment completed features count
   - Update completion percentage

### File Location

- **TODO File**: `docs/design/TODO.md` (EDIT/UPDATE this file)

### Update Checklist

When updating TODO.md, ensure you:
- ✅ Mark completed tasks with `[x]` and `✅` emoji
- ✅ Update feature status (📋/🔄/✅/⏸️/❌)
- ✅ Update task completion counts `(X/Y tasks completed)`
- ✅ Update summary statistics
- ✅ Update "Last Updated" date
- ✅ Move fully completed features to completed section
- ✅ Keep structure and formatting consistent

### Status Emoji Guide

Use these consistently:
- `📋` Not Started
- `🔄` In Progress
- `✅` Completed
- `⏸️` Blocked
- `❌` Cancelled/Deferred

### Example Update

**Before**:
```markdown
### Feature: Core Parser
**Status**: 📋 Not Started
**Priority**: Critical

- [ ] Design AST
- [ ] Implement lexer
- [ ] Write tests
```

**After** (assuming AST design is done):
```markdown
### Feature: Core Parser
**Status**: 🔄 In Progress (1/3 tasks completed)
**Priority**: Critical

- [x] Design AST ✅
- [ ] Implement lexer
- [ ] Write tests
```

---

## Core Competencies

### Task Breakdown
- Decompose features into manageable tasks
- Identify dependencies between tasks
- Estimate effort and complexity
- Create actionable work items
- Define clear completion criteria

### Progress Tracking
- Maintain task status and progress
- Track completed vs pending work
- Identify blockers and risks
- Monitor milestone progress
- Generate status reports

### TODO List Management
- Organize tasks hierarchically
- Use markdown checkboxes for tracking
- Group tasks by feature or component
- Prioritize tasks effectively
- Keep lists up to date

### Milestone Planning
- Define project milestones
- Set realistic deadlines
- Track milestone completion
- Adjust plans based on progress
- Communicate milestone status

### Workflow Organization
- Define development workflows
- Establish task assignment processes
- Create review and approval gates
- Manage release cycles
- Coordinate team activities

## TODO.md File Structure

### Standard TODO Format

```markdown
# Project TODO List

**Last Updated**: YYYY-MM-DD
**Project**: [Project Name]

## Summary
- **Total Features**: X
- **Completed Features**: Y (Z%)
- **Total Tasks**: A
- **Completed Tasks**: B (C%)

## Legend
- ✅ Completed
- 🔄 In Progress
- ⏸️ Blocked
- ❌ Cancelled
- 📋 Not Started

---

## Milestone 1: [Milestone Name]
**Target**: [Date/Version]
**Status**: [Not Started/In Progress/Completed]

### Feature: [Feature Name]
**Status**: 📋 Not Started
**Priority**: High
**Effort**: Medium

- [ ] Task 1: Description
- [ ] Task 2: Description
  - [ ] Subtask 2.1: Description
  - [ ] Subtask 2.2: Description
- [ ] Task 3: Description

### Feature: [Feature Name]
**Status**: 🔄 In Progress (2/5 tasks completed)
**Priority**: High
**Effort**: Large

- [x] Task 1: Description ✅
- [x] Task 2: Description ✅
- [ ] Task 3: Description
- [ ] Task 4: Description
- [ ] Task 5: Description

---

## Milestone 2: [Milestone Name]
**Target**: [Date/Version]
**Status**: [Not Started/In Progress/Completed]

### Feature: [Feature Name]
**Status**: 📋 Not Started
**Priority**: Medium
**Effort**: Small

- [ ] Task 1: Description
- [ ] Task 2: Description

---

## Backlog
Features and tasks not yet assigned to a milestone.

### Feature: [Feature Name]
**Priority**: Low
**Effort**: Medium

- [ ] Task 1: Description
- [ ] Task 2: Description

---

## Completed Features ✅

### Feature: [Feature Name]
**Completed**: YYYY-MM-DD

- [x] Task 1: Description ✅
- [x] Task 2: Description ✅
- [x] Task 3: Description ✅

---

## Cancelled/Deferred ❌

### Feature: [Feature Name]
**Reason**: [Why cancelled or deferred]

- [x] Task 1: Description ❌

---

## Notes
- Note about project status
- Important considerations
- Recent changes
```

## Best Practices

### Task Writing
- **Be Specific**: "Implement parser for section blocks" not "Work on parser"
- **Be Actionable**: Use verbs (Implement, Create, Design, Test)
- **Be Measurable**: Define clear completion criteria
- **Be Scoped**: Keep tasks small enough to complete in 1-2 days
- **Be Independent**: Minimize dependencies where possible

### Progress Tracking
- **Update Regularly**: Keep TODO list current
- **Mark Completions Immediately**: Check off tasks as soon as done
- **Track In-Progress**: Show what's actively being worked on
- **Note Blockers**: Document what's blocking progress
- **Celebrate Wins**: Acknowledge completed milestones

### Organization
- **Group Logically**: By feature, milestone, or component
- **Prioritize Clearly**: Use priority labels (Critical, High, Medium, Low)
- **Estimate Effort**: Use t-shirt sizes (S, M, L, XL) or story points
- **Track Dependencies**: Note which tasks depend on others
- **Archive Completed**: Move done items to completed section

### Milestone Management
- **Set Realistic Targets**: Base on actual velocity
- **Define Clear Scope**: Know what's in and out of milestone
- **Review Regularly**: Adjust scope if needed
- **Communicate Progress**: Keep stakeholders informed
- **Learn from History**: Use past milestones to improve estimates

## Task Templates

### Feature Implementation Task Template
```markdown
### Feature: [Feature Name]
**Status**: 📋 Not Started
**Priority**: [Critical/High/Medium/Low]
**Effort**: [Small/Medium/Large/X-Large]
**Owner**: [Person/Team]

#### Analysis & Design
- [ ] Research existing solutions and best practices
- [ ] Write feature specification (docs/design/features/[name].md)
- [ ] Design technical architecture
- [ ] Review and approve design

#### Implementation
- [ ] Set up project structure
- [ ] Implement core functionality
- [ ] Add error handling
- [ ] Write unit tests
- [ ] Write integration tests

#### Documentation
- [ ] Update API documentation
- [ ] Add usage examples
- [ ] Update README if needed
- [ ] Write migration guide if needed

#### Review & Release
- [ ] Code review
- [ ] Performance testing
- [ ] Security review
- [ ] Update changelog
- [ ] Tag release
```

### Bug Fix Task Template
```markdown
### Bug: [Bug Description]
**Status**: 📋 Not Started
**Severity**: [Critical/High/Medium/Low]
**Affects**: [Component/Version]

- [ ] Reproduce bug
- [ ] Identify root cause
- [ ] Design fix
- [ ] Implement fix
- [ ] Add regression test
- [ ] Verify fix
- [ ] Update documentation if needed
```

### Research Task Template
```markdown
### Research: [Topic]
**Status**: 📋 Not Started
**Purpose**: [Why researching]

- [ ] Define research questions
- [ ] Survey existing solutions
- [ ] Evaluate options
- [ ] Document findings (docs/research/[topic].md)
- [ ] Make recommendations
- [ ] Present findings for review
```

## Progress Calculation

### Feature Completion Percentage
```
Completed Tasks / Total Tasks × 100
```

### Milestone Progress
```
Completed Features / Total Features in Milestone × 100
```

### Overall Project Progress
```
(Completed Tasks across all milestones) / (Total Tasks) × 100
```

## Common Patterns

### Iterative Development
1. **Milestone 1**: Core functionality (MVP)
2. **Milestone 2**: Extended features
3. **Milestone 3**: Polish and optimization
4. **Milestone 4**: Advanced features

### Feature Flags
- Use checkboxes with feature flag indicators
- `- [ ] [EXPERIMENTAL] Feature name`
- `- [ ] [DEPRECATED] Old feature (remove in v2.0)`

### Task Dependencies
```markdown
- [ ] Task A (dependency: none)
- [ ] Task B (dependency: Task A)
- [ ] Task C (dependency: Task A, Task B)
```

## Examples

### Example 1: Simple Feature Breakdown
```markdown
### Feature: Add support for AsciiDoc tables

**Status**: 📋 Not Started
**Priority**: High
**Effort**: Large

- [ ] Research AsciiDoc table syntax
- [ ] Design AST representation for tables
- [ ] Implement table parser
  - [ ] Basic table parsing
  - [ ] Cell spanning support
  - [ ] Header rows
  - [ ] Footer rows
- [ ] Add table validation
- [ ] Write unit tests
- [ ] Update documentation
```

### Example 2: Milestone with Multiple Features
```markdown
## Milestone 1: Core Parser (v0.1.0)
**Target**: 2024-12-15
**Status**: 🔄 In Progress (3/6 features completed)

### Feature: Document Structure Parsing ✅
**Status**: ✅ Completed (2024-11-10)
- [x] Parse document headers ✅
- [x] Parse sections ✅
- [x] Parse paragraphs ✅

### Feature: Block Elements
**Status**: 🔄 In Progress (2/4 tasks)
- [x] Parse literal blocks ✅
- [x] Parse code blocks ✅
- [ ] Parse quote blocks
- [ ] Parse sidebar blocks

### Feature: Inline Elements
**Status**: 📋 Not Started
- [ ] Parse bold/italic text
- [ ] Parse inline code
- [ ] Parse links
```

## When to Use This Skill

**PRIMARY USE CASES** (TODO.md file updates):
- ✅ Marking tasks as complete (checking off checkboxes in TODO.md)
- ✅ Adding new tasks to features in TODO.md
- ✅ Updating feature status in TODO.md (📋 → 🔄 → ✅)
- ✅ Updating progress metrics and summary statistics in TODO.md
- ✅ Adding new features to milestones in TODO.md
- ✅ Moving completed features to the completed section in TODO.md

**SUPPORTING USE CASES** (general project management knowledge):
- Breaking down features into implementation tasks
- Estimating effort and complexity (Small/Medium/Large/X-Large)
- Planning milestones and releases
- Identifying task dependencies
- Managing project backlog and priorities

## Lessons Learned

This section contains real-world insights from project management experience. The doctora-architect agent updates this section when discovering better ways to organize and track work.

---

*This section will grow as projects are managed and lessons are learned.*

## Common Challenges

### Challenge: Tasks too large
**Symptom**: Tasks take weeks to complete
**Solution**: Break into subtasks completable in 1-2 days
**Rule of Thumb**: If you can't describe task in one sentence, it's too big

### Challenge: Unclear completion criteria
**Symptom**: Tasks marked done but not really finished
**Solution**: Define specific acceptance criteria for each task
**Example**: "Parser works" → "Parser handles all AsciiDoc block types with test coverage >90%"

### Challenge: TODO list becomes stale
**Symptom**: List doesn't reflect actual work being done
**Solution**:
- Update list immediately when completing tasks
- Review and update weekly
- Archive completed items monthly

### Challenge: Poor task estimates
**Symptom**: Features take much longer than expected
**Solution**:
- Track actual vs estimated time
- Learn from past velocity
- Add buffer for unknowns (multiply estimate by 1.5-2x)

---

*Add new challenges and solutions here as they are encountered.*
