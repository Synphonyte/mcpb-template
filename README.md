# my-server

A minimal MCP server in Rust, built with [`rmcp`](https://crates.io/crates/rmcp), that releases
itself as [MCP Bundles](https://github.com/modelcontextprotocol/mcpb) (`.mcpb`): one file per
platform that Claude Desktop installs with a click.

## Start from it

Press **Use this template** on GitHub, or generate a copy locally:

```sh
cargo generate --git https://github.com/Synphonyte/mcpb-template --name my-server
```

Then give the server its name. It appears in four places:

- `name` in `Cargo.toml`, which is the binary's name
- `name`, `entry_point` (`server/<binary>`) and `mcp_config.command` in `manifest.json`
- `BINARY` in `.github/workflows/release.yml`

fill in `display_name`, `description` and `author` in `manifest.json`, and replace `LICENSE`
with your own.

## Develop

`src/main.rs` is the whole server: one `greet` tool, served over stdio. Add tools as methods with
`#[tool]`, and list them under `tools` in `manifest.json` so clients show them before installing.

```sh
cargo run    # speaks MCP on stdin and stdout
```

To try a bundle on your own machine, pack the binary you built with the
[`mcpb`](https://crates.io/crates/mcpb) command and open the bundle in Claude Desktop:

```sh
cargo binstall mcpb   # or: cargo install --locked mcpb
cargo build --release
mcpb pack manifest.json --platform darwin --arch arm64 --executable target/release/my-server
```

## Release

Set the same version in `Cargo.toml` and `manifest.json`, commit, then tag and push:

```sh
git tag v0.1.0
git push origin v0.1.0
```

`.github/workflows/release.yml` builds the server for macOS (arm64, x64), Linux (arm64, x64) and
Windows (arm64, x64), packs each build with the [`Synphonyte/mcpb-pack`](https://github.com/Synphonyte/mcpb-pack)
action, and attaches the six bundles to a GitHub release:

```text
my-server-0.1.0-darwin-arm64.mcpb
my-server-0.1.0-darwin-x64.mcpb
my-server-0.1.0-linux-arm64.mcpb
my-server-0.1.0-linux-x64.mcpb
my-server-0.1.0-win32-arm64.mcpb
my-server-0.1.0-win32-x64.mcpb
```

Running the workflow by hand (Actions → Release → Run workflow) packs the bundles as artifacts
without releasing them.
