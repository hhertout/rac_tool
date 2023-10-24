<p align="center">
  <h1 align="center">Rat - Auto-Copy Tool</h1>
    <p align="center">An app for auto-managing your repo file !</p>
</p>

<p align="center">
    <img src="https://img.shields.io/badge/version-1.0-blue" alt="version">
    <img src="https://img.shields.io/github/contributors/hhertout/rac_tool" />
    <a href="https://github.com/hhertout/rac_tool/actions">
      <img alt="Tests Passing" src="https://github.com/hhertout/rac_tool/actions/workflows/rust.yml/badge.svg" />
    </a>
</p>

## Presentation


Rat is a tool allowing you to automate file copying, automatic replacement of certain textual parts of a file.

All are the process is set from a yml configuration file.

You can configure it for processing globaly, or more specificly by targeting a specifiq file name, or file path.

## Getting started

### Initialization
Install the executable file in your working directory.

To create the configuration file, run :

```bash
./<exec_name> init
```

It create the config.yml in your current directory.

### Configuration

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
        - hello world:hello mom
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

### Running the tool

Simply run

```bash
./<exec_file> run
```

## Installation

-- 
