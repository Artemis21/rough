# Rough

Rough is a very simple and very opinionated tool for generating small gallery/
showcase/portfolio websites. Once you've built it or obtained a pre-built
executable, just do `rough <source dir> <build dir>` to generate your site.
For example, `rough src out`.

## Source format

A site generated with this tool consists of:

- A single HTML index file.
- Any number of project files, each their own HTML file.
- Any number of static files, such as images, CSS, and JavaScript.

Below, `src/` will be used for the source directory, and `out/` will be used
for the output directory. However, these are configurable as mentioned above.

### Project files

Project files will be read from `src/projects`. Each project file should be a
Markdown file with YAML front matter. They will be rendered to `out/projects`,
with whatever extension they have replaced with `.html`.

A file called `src/project.html` must also be present. This is a
[Tera](https://tera.netlify.app/docs/#introduction) template, which will be
used to render each project file. The following context variables are
available:

- `meta`: The YAML front matter.
- `content`: The Markdown content, rendered as HTML.

### The index file

A file called `src/index.html` must be present. It is another Tera template,
this time used just once to render the index file. Just one context variable is
available: `projects`. This is a list, each element of which is the YAML front
matter for one of the project files.

The index file is rendered to `out/index.html`.

### Static files

Any files in `src/static/` will be recursively copied to `out/static/`.

## Markdown flavour

Markdown parsing and rendering is done by
[`pulldown-cmark`](https://docs.rs/pulldown-cmark), which should be
[CommonMark](https://commonmark.org/) compliant. The following non-commonmark
extensions are added:

- [GitHub Flavoured Markdown (GFM) tables](https://github.github.com/gfm/#tables-extension-)
- [GFM Task Lists](https://github.github.com/gfm/#task-lists-extension-)
- [GFM Strikethrough](https://github.github.com/gfm/#strikethrough-extension-)
- [`pulldown-cmark`'s footnotes](https://github.com/raphlinus/pulldown-cmark/blob/master/specs/footnotes.txt)
- `pulldown-cmark`'s smart punctuation
- [`pulldown-cmark`'s heading attributes](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/struct.Options.html#associatedconstant.ENABLE_HEADING_ATTRIBUTES)
- A custom extension which removes wrapping paragraph tags when they surround
  a sole image.

YAML frontmatter should be at the start of the document, delineated by three
dashes on their own line, both above and below the frontmatter. For example:

```
---
title: Hello World!
slug: hello_world
description: My first ever file.
---

Hia!!
```
