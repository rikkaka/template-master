# Introduce
A Simple Command Line Tool for Streamlined Template Management

Still in development, but it can be used.

# Usage
## Install
Ensure your rust environment is ready, then run:
```bash
git clone https://github.com/rikkaka/template-master
cd template-master
cargo install --path .
```
## Add a template
```bash
tempmaster add <template_path>
```
Also can use --rename to add a renamed template:
```bash
tempmaster add example.tex --rename renamed.tex
```
## remove a template
```bash
tempmaster remove <template_name>
```
## list all templates
```bash
tempmaster list
```
## use a template
Copy the template to current directory:
```bash
tempmaster clone <template_name>
```