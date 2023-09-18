# Rust CLI tool

## Goal config file

```yaml
# configuration file

# Base file path to process - Default is root directory
on: .

copy:
  # List all files to copy 
  # - `base:dest`
  files:
    - hello.txt.example:hello.txt

replace:
  # Replace a string in all files, in all directory
  global:
    - hello mom:hello mom

  # Target files by name. By default, target all directory.
  # If you want to target a specifiq file, you must specify the correct path.
  # content: 
  #     - `past:future`
  target:
    - file_name: hello.txt
      content:
        - hello mom:hello mom
        - string to replace:string replaced
    - file_name: dir/example/hello.txt
      content:
        - hello mom:hello mom

# ignore directory during all the process
ignored_dir:
  - /.git/
  - /tests/
  - /node_modules/
  - /vendor/
  - /target/
```

## Presentation

## Installation

## Getting started
