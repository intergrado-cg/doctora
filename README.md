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

## Status

This project is in early development.

## License

TBD
