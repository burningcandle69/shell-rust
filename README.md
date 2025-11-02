# Rust Shell - A POSIX-Compliant Shell Implementation

[![progress-banner](https://backend.codecrafters.io/progress/shell/f75a9106-d0a7-47be-8dce-c9db20ebee23)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

A fully-featured, POSIX-compliant shell written in Rust, built as part of the ["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

## 🌟 Features

### Core Shell Capabilities
- **Command Execution**: Execute external programs with full argument support
- **Builtin Commands**: Native implementations of `echo`, `exit`, `type`, `pwd`, `cd`, and `history`
- **Pipeline Support**: Chain commands using `|` operator
- **I/O Redirection**: Full support for `>`, `>>`, `2>`, `2>>`, and `1>` operators
- **Command History**: Persistent command history with read/write/append operations
- **Tab Completion**: Intelligent autocomplete for commands in PATH

### Advanced Features
- **Quote Handling**: Proper parsing of single quotes (`'`), double quotes (`"`), and escape sequences
- **Trie-based Autocomplete**: Fast command suggestions using prefix tree data structure
- **Flexible I/O**: Support for stdin/stdout/stderr redirection and piping
- **PATH Resolution**: Automatic executable discovery across PATH directories

## 🏗️ Architecture

The codebase is organized into focused modules:

```
src/
├── main.rs           # Entry point and REPL loop
├── shell.rs          # Shell state and execution orchestration
├── command.rs        # Command parsing and structure
├── builtins.rs       # Builtin command implementations
├── shell_io.rs       # I/O abstraction layer (stdin/stdout/stderr/pipes/files)
├── trie.rs           # Trie data structure for autocomplete
└── autocomplete.rs   # Rustyline integration for tab completion
```

### Module Breakdown

#### `shell.rs`
Manages shell state including:
- Current working directory
- PATH environment variable
- Command history
- Exit status codes
- Pipeline execution coordination

#### `command.rs`
Handles command parsing with support for:
- Quote escaping (single, double, and backslash)
- Argument tokenization
- Output redirection operators
- Pipeline splitting

#### `builtins.rs`
Implements builtin commands:
- `exit <code>` - Exit shell with status code
- `echo <args>` - Print arguments to stdout
- `type <cmd>` - Display command type (builtin or path)
- `pwd` - Print working directory
- `cd <path>` - Change directory (supports `~` for home)
- `history [n]` - Show command history
  - `history -r <file>` - Read history from file
  - `history -w <file>` - Write history to file
  - `history -a <file>` - Append new history to file

#### `shell_io.rs`
Provides unified I/O abstraction for:
- Standard streams (stdin/stdout/stderr)
- File descriptors
- Pipes (for pipeline chaining)
- Conversion to `std::process::Stdio`

#### `trie.rs`
Efficient prefix tree implementation for:
- Fast command lookup
- Fuzzy matching
- Autocomplete suggestions

#### `autocomplete.rs`
Integrates with `rustyline` for:
- Tab completion
- Command suggestions
- Visual feedback (bell on no matches)

## 🚀 Getting Started

### Prerequisites
- Rust 1.80+ (specified in `Cargo.toml`)
- Cargo package manager

### Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd shell-rust

# Build the project
cargo build --release

# Run the shell
./your_program.sh
```

### Usage Examples

```bash
# Basic command execution
$ echo Hello, World!
Hello, World!

# Pipeline chaining
$ cat file.txt | grep pattern | wc -l

# Output redirection
$ echo "log entry" >> output.log
$ command 2> errors.log

# Tab completion
$ ec<TAB>
echo  # autocompletes

# Command history
$ history 5
    1  echo test
    2  pwd
    3  cd ~
    4  ls -la
    5  history 5

# Change directory
$ cd ~/projects
$ pwd
/Users/username/projects
```

## 🔧 Implementation Highlights

### Quote and Escape Handling
The command parser correctly handles:
- **Single quotes**: Preserves literal values (except `\'`)
- **Double quotes**: Allows variable expansion and escapes `\``, `$`, `"`, `` ` ``, and `\n`
- **Backslash escaping**: General escape character outside quotes

### Pipeline Architecture
Pipelines are executed by:
1. Splitting commands on `|` delimiter
2. Creating pipe pairs between consecutive commands
3. Redirecting stdout → pipe writer → stdin
4. Waiting for all processes in pipeline chain

### I/O Redirection
Supports multiple redirection operators:
- `>` / `1>` - Redirect stdout (truncate)
- `>>` / `1>>` - Redirect stdout (append)
- `2>` - Redirect stderr (truncate)
- `2>>` - Redirect stderr (append)

### Autocomplete System
Uses a Trie for O(k) prefix matching where k is the prefix length:
1. Scans PATH directories at startup
2. Builds trie of all executable names
3. Provides instant suggestions on tab press

## 📝 Command Reference

### Builtin Commands

| Command | Description | Example |
|---------|-------------|---------|
| `exit <code>` | Exit shell with status | `exit 0` |
| `echo <args>` | Print arguments | `echo "Hello World"` |
| `type <cmd>` | Show command type | `type ls` |
| `pwd` | Print working directory | `pwd` |
| `cd <path>` | Change directory | `cd /tmp` |
| `history [n]` | Show history | `history 10` |

### History Options
- `history -r <file>` - Read history from file
- `history -w <file>` - Write complete history to file
- `history -a <file>` - Append new entries to file

## 🧪 Testing

```bash
# Run tests
cargo test

# Run with debug output
debug: true  # in codecrafters.yml
```

## 📦 Dependencies

- **rustyline** (17.0.2) - Readline implementation with history and completion
- **regex** (1.12.2) - Command parsing and pattern matching
- **is_executable** (1.0.5) - Portable executable detection

## 🎯 Design Decisions

1. **Enum-based I/O**: `Input` and `Output` enums provide type-safe I/O handling
2. **Trie for Autocomplete**: O(k) prefix matching outperforms linear search
3. **Separate Parsing**: Command parsing is isolated from execution logic
4. **Child Process Management**: `ChildOrStatus` enum handles both async and sync command execution
5. **Persistent History**: History stored in `HISTFILE` environment variable location

## 🔮 Future Enhancements

- [ ] Environment variable expansion (`$VAR`)
- [ ] Command substitution (`` `cmd` `` or `$(cmd)`)
- [ ] Background jobs (`&`)
- [ ] Job control (`fg`, `bg`, `jobs`)
- [ ] Glob expansion (`*.txt`)
- [ ] Signal handling (Ctrl+C, Ctrl+Z)
- [ ] Alias support

## 📄 License

This project is part of the CodeCrafters challenge.

## 🙏 Acknowledgments

Built as part of the [CodeCrafters](https://codecrafters.io) Shell challenge.
