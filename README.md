🔆 glint
========

[![License](https://img.shields.io/hexpm/l/thumbp.svg)](https://github.com/ryochin/thumbp/blob/main/LICENSE)

A fast and flexible command-line tool to colorize text lines using regular expressions.

**glint** is a drop-in replacement for tools like [`grcat`](https://github.com/garabik/grc), written in Rust for performance.
It reads text from standard input and applies colors/styles based on patterns you define in a simple TOML config file.

## 🔧 Why glint?

- ⚡️ Fast — written in Rust, zero Python overhead
- 📜 PCRE-compatible regex (via Oniguruma)
- 🎨 Supports ANSI styles like `bold`, `underline`, `red`, etc.
- 🧩 Handles multiple matches per line
- 📁 Easy-to-read and write config file (`rules.toml`)

## 📦 Installation

```sh
brew tap ryochin/tap
brew install glint
```

## 🚀 Usage

```sh
glint rules.toml < input.txt
```

### MySQL

dot.my.cnf

```ini
[mysql]

pager = /usr/local/bin/glint /etc/glint/rules.toml
```

Dockerfile

```dockerfile
FROM mysql:8.0.32

RUN microdnf install -y wget

ENV GLINT_VERSION 0.1.0

RUN wget -q -O /usr/local/bin/glint \
    https://github.com/ryochin/glint/releases/download/v$GLINT_VERSION/glint-v$GLINT_VERSION-aarch64-unknown-linux-musl \
  && chmod 755 /usr/local/bin/glint \
  && mkdir -p /etc/glint \
  && wget -q -O /etc/glint/rules.toml https://raw.githubusercontent.com/ryochin/glint/refs/heads/main/rules.toml

COPY dot.my.cnf /root/.my.cnf
```

## 📝 Config format: `rules.toml`

Each rule has:

- one or more regular expressions
- a color or style

```toml
[[rules]]
regexp = ["ERROR", "CRITICAL"]
color = "red"

[[rules]]
regexp = "WARN"
color = "yellow"

[[rules]]
regexp = "backup completed"
color = "green"

[[rules]]
regexp = "username="
color = "bold"
```

You can use either:

```toml
regexp = "WARN"
```

or

```toml
regexp = ["WARN", "WARNING"]
```

## 🎨 Supported styles

You can use any of the following:

- Colors: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
- Text styles: `bold`, `underline`, `blink`, `reverse`, `concealed`
- `default` resets style

Currently, one style per rule. Multiple styles (like `["bold", "red"]`) may be supported in the future.

📄 License
-------

The MIT License
