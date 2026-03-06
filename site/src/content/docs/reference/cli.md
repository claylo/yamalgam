---
title: CLI Reference
description: Command-line interface reference for yamalgam
---

## Global options

| Option | Description |
|--------|-------------|
| `--verbose` | Enable verbose output |
| `--json` | Output in JSON format |
| `--version` | Print version information |
| `--help` | Print help |

## Commands

### `info`

Display version, build, and environment information.

```bash
yamalgam info
yamalgam info --json
```

### `completions`

Generate shell completions.

```bash
yamalgam completions bash
yamalgam completions zsh
yamalgam completions fish
yamalgam completions powershell
```


### `doctor`

Check configuration and environment health.

```bash
yamalgam doctor
```
