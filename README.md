# Doctor A (doctora)

A modular asciidoc parser and processor written in Rust.

## Overview

Doctor A is an asciidoc parser and processor similar to asciidoctor, designed with modularity and extensibility in mind. The core parser validates asciidoc input and provides a foundation for pluggable processors that transform asciidoc to various output formats.

## Architecture

### Core Parser
The base program focuses on:
- Parsing asciidoc input
- Validating the asciidoc syntax
- Providing a clean AST for processors

### Processor Chain
Processors can:
- Transform validated asciidoc to different formats (PDF, HTML, etc.)
- Pass snippets or sections to other processors
- Incorporate outputs from child processors into their own output
- Example: A PDF processor can invoke an image processor to handle embedded images

## Usage

**Note**: The project is currently in the design phase. The following shows the planned CLI interface.

### Basic Parsing and Validation

To parse and validate an AsciiDoc file:

```bash
# Parse and validate (checks syntax, reports errors)
doctora input.adoc

# Parse a file and output the AST for inspection
doctora input.adoc --ast
```

### Converting to Output Formats

To convert AsciiDoc to various output formats:

```bash
# Convert to HTML
doctora input.adoc --format html -o output.html

# Convert to Markdown
doctora input.adoc --format markdown -o output.md

# Convert to PDF (future)
doctora input.adoc --format pdf -o output.pdf
```

### Options

```bash
# Show detailed error messages with context
doctora input.adoc --verbose

# Validate only (don't produce output)
doctora input.adoc --validate

# Specify output file
doctora input.adoc --format html --output output.html

# Read from stdin
cat document.adoc | doctora --format html > output.html
```

### Exit Codes

- `0`: Success - document parsed/validated successfully
- `1`: Parse error - invalid AsciiDoc syntax
- `2`: Processing error - error during format conversion
- `3`: I/O error - file not found or cannot be read

## Status

This project is in early development. See `docs/design/` for current architecture and planning documentation.

## License

TBD
