# thino-bolt ⚡

A lightning-fast CLI to add notes to Obsidian [Thino](https://github.com/Quorafind/Obsidian-Thino) via the official [Obsidian CLI](https://help.obsidian.md/cli).

## Requirements

- Obsidian 1.12+ with the `obsidian` CLI available on your `PATH`
- The Thino plugin enabled in the target vault
- Obsidian running (the CLI talks to the running app)

## Installation

```shell
cargo build --release
cp target/release/thino-bolt ~/.local/bin/
```

## Usage

```shell
thino-bolt <vault-name> "your message"

# With tag
thino-bolt <vault-name> --tag todo "buy milk"
```

Notes are added to the daily note as Thino tasks (`- [ ] HH:MM ...`); the
timestamp is supplied by Thino itself.

## License

MIT
