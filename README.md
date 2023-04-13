# Introduction
A simple command-line tool for easy template management. Still under development but is usable.

# Installation
Make sure your Rust environment is set up. Then, run these commands:
```bash
git clone https://github.com/rikkaka/template-master
cd template-master
cargo install --path .
```

# Usage
- **Add Template**: 
  ```bash
  tempmaster add <template_path>
  ```
  `<template_path>` can be a file or folder. Use `--rename` or `-r` to add a template with a new name:
  ```bash
  tempmaster add example.tex --rename renamed.tex
  ```

- **Remove Template**: 
  ```bash
  tempmaster remove <template_name>
  ```

- **List Templates**: 
  ```bash
  tempmaster list
  ```

- **Use Template** (copy to the current directory): 
  ```bash
  tempmaster clone <template_name>
  ```
