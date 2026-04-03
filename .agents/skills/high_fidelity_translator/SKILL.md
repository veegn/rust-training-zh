---
name: high_fidelity_translator
description: Translate long Markdown documents with high fidelity. Use when translating books, chapters, or technical Markdown while preserving structure, code blocks, tables, links, and Mermaid content without summarizing or omitting material.
---

# High Fidelity Translator

Translate Markdown content completely and faithfully. The goal is full translation of natural-language prose while preserving document structure and technical artifacts.

## Use This Skill When

- The input is a long Markdown file or a section of a book.
- The output must preserve headings, lists, tables, links, code fences, and Mermaid blocks.
- The user wants translation, not summarization, rewriting, or editing for style.

## Core Rules

- Translate all natural-language prose. Do not skip, compress, summarize, or paraphrase away content.
- Preserve Markdown structure exactly as written unless the user explicitly requests structural edits.
- Preserve code blocks, inline code, file paths, commands, identifiers, config keys, and string literals verbatim unless the user explicitly asks to translate comments inside code.
- Do not translate log messages, printed output text, exception messages, or other user-facing strings inside code. Only translate code comments when code-comment translation is explicitly required.
- Do not translate computer-related technical terms such as API names, protocol names, language features, library names, framework names, tooling names, type-system terms, and concurrency terms unless the user explicitly asks for localized terminology.
- Preserve Mermaid logic verbatim. Do not rewrite node relationships, syntax, or diagram structure.
- Preserve links, image targets, anchors, HTML blocks, callouts, and table layout.
- Do not add explanatory notes such as "content omitted" or "remaining sections are similar".

## Translation Workflow

### 1. Inspect The Source

Before translating, identify:

- File path and target output path.
- Whether the file is short enough to translate in one pass.
- Whether the file contains large code blocks, large tables, or Mermaid diagrams that require extra care.

## Output Path Rules

- If the user explicitly provides an output path, use that path.
- Otherwise, prefer a sibling localized path instead of overwriting the source file.
- In this repository, when translating English source content, prefer writing into the corresponding `src/zh/` location that mirrors the source structure.
- If the matching localized directory does not exist yet, create the mirrored path only when the task requires file output.
- Do not overwrite the source file unless the user explicitly asks for in-place replacement.
- Before writing any translated content, clear the target file so no stale content remains from a previous run.

### 2. Chunk Conservatively

If the file is long, translate in chunks.

- Prefer splitting at Markdown heading boundaries.
- Avoid splitting in the middle of a paragraph, list, table, code fence, blockquote, or Mermaid block.
- Use chunks small enough to keep continuity and avoid omissions. As a default, keep each chunk around 80 to 150 lines.
- If one section is larger than the default range, split again at the next safe sub-heading or other clear structural boundary.
- If a section contains a long table, long code fence, or Mermaid block, keep that structure intact even if the chunk becomes larger than the default range.
- Prefer fewer clean chunks over smaller chunks that damage continuity.

### 3. Translate Faithfully

For each chunk:

- Translate prose completely.
- Keep terminology consistent across chunks.
- Keep heading levels unchanged.
- Keep bullet and numbered list structure unchanged.
- Keep table column count and row order unchanged.
- Keep fenced code languages unchanged, such as `rust`, `toml`, `bash`, or `mermaid`.

### 4. Write Output Safely

- Clear the target file before writing the first translated chunk.
- Append translated chunks in source order.
- Do not reorder sections.
- Do not merge adjacent sections unless they were already merged in the source.
- If the output path is not specified, prefer writing to a sibling localized path rather than overwriting the source.

## Default Decisions

- Translate body prose, headings, table text, blockquotes, and image alt text.
- Do not translate code, command lines, URLs, file paths, schema keys, log text, or string literals inside code.
- Do not translate computer-related technical terms; keep the original term and translate surrounding prose naturally.
- If a heading mixes a proper noun and prose, keep the proper noun and translate the prose.

## Failure Handling

- If a chunk boundary would break Markdown structure, choose a smaller or later boundary.
- If terminology becomes inconsistent, normalize it before continuing.
- If the source is ambiguous, prefer literal fidelity over stylistic rewriting.

## Output Standard

A valid result must satisfy all of the following:

- The translated file covers the full source content.
- No section is omitted.
- Markdown structure remains intact.
- Code fences, Mermaid blocks, links, and tables remain usable.
- The result reads as a translation of the source, not a summary or adaptation.
