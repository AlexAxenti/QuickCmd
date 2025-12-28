# QuickCmd (qc)

QuickCmd (`qc`) is a small CLI tool that lets you save and run frequently used shell commands so you don’t have to memorize them or repeatedly retype them.

---

## Features

- Save shell commands by name
- Run them later with a short alias
- Supports pipes (`|`), chaining (`&&`), redirects, etc.
- Commands are stored locally (per user)
- Quick and easy to type

---

## Installation

### Option 1: Install with Cargo (Rust required)

From this repo:
```bash
cargo install --git https://github.com/AlexAxenti/QuickCmd --locked
```

Or after cloing locally:
```bash
cargo install --path . --force
```

### Option 2: Download a prebuilt binary (no Rust required)
1. Go to [Releases](https://github.com/AlexAxenti/QuickCmd/releases) on GitHub
2. Download the binary for your OS:
    -  qc-windows-x86_64.exe
    - qc-linux-x86_64
    - qc-macos-x86_64
3. Put it somewhere already on your `PATH`

#### Linux/macOS
```bash
chmod +x qc-<os>-x86_64
sudo mv qc-<os>-x86_64 /usr/local/bin/qc
```

#### Windows
1. Rename to `qc.exe`
2. Move it to a folder on PATH

Test installation with:
```bash
qc --version
```

## Usage:

### Save a command

```bash
qc s up -- docker compose up -d
```

### Run a command

```bash
qc r up
```

### List all command

```bash
qc ls
```


### Show a command

```bash
qc sh up
```


### Remove a command

```bash
qc rm up
```

## Shell operators (important)

If your command uses shell operators like:
- pipes (|)
- chaining (&&, &)
- redirects (>, >>)
- parentheses (())

wrap the entire command in quotes, otherwise your shell will interpret it before qc can save it: 
```bash
qc s example "echo one && echo two) | grep two"
```
