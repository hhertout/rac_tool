# Rust CLI tool

## Goal config file
```yaml
# configuration file

on: .

copy:
  files:
    - hello.txt.example:hello.txt

replace:
  global:
    - hello world:hello mom
  target:
    setting.php:
      - hello world:hello mom

ignored_dir:
  - /.git/
  - /tests/
  - /node_modules/
  - /vendor/
  - /target/

```