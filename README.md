# thino-bolt ⚡

A lightning-fast CLI to append notes to Obsidian [Thino](https://github.com/Quorafind/Obsidian-Thino) via [advanced-uri](https://github.com/Vinzent03/obsidian-advanced-uri).

## Installation

```shell
cargo build --release
cp target/release/thino-bolt ~/.local/bin/

```

## Usage

``` shell
thino-bolt <vault-name> "your message"

# With tag
thino-bolt <vault-name> --tag todo "buy milk"
```
## License

MIT
