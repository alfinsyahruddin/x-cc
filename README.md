# X-Code Coverage

CLI tool for Code Coverage Reporting for Xcode Project with excluded file path feature. 🕊️ 🦀

![x-cc](https://i.ibb.co.com/Zm3BRM7/Screenshot-2024-12-04-at-11-02-24-AM.png)

## Requirements

- Xcode
- Rust

## Installation

To install the CLI tool, run the following command:

```bash
cargo install x-cc
```

## Usage

To generate the code coverage report for the Xcode project, run the following command:

```bash
x-cc -- -path test.xcresult
```

(You can find the `test.xcresult` file in the `DerivedData` folder of the Xcode project.)

### Ignore Folder / File

To ignore the folders or files, you can list them in the `.xccignore` file (Regex Supported):

``` 
# UI
- .*ViewController.swift
- .*Cell.*
```
