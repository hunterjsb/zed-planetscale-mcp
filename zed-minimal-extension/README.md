# Minimal Zed Extension Example

This is a minimal working example of a Zed extension with a context server implementation.

## Structure

- `extension.toml` - Metadata for the extension
- `Cargo.toml` - Rust package configuration
- `src/lib.rs` - The extension implementation
- `.cargo/config.toml` - Rust configuration for WASM target

## Building the Extension

To build the extension, you need the Rust toolchain with the wasm32-wasi target:

```bash
rustup target add wasm32-wasi
cargo build --target wasm32-wasi
```

## Installing the Extension

For development, you can install this extension by adding it to your Zed extensions directory:

```bash
mkdir -p ~/.zed/extensions/
cp -r . ~/.zed/extensions/zed-minimal-extension
```

## Context Server Implementation

This example includes a minimal context server setup that uses the Model Context Protocol (MCP). The key part is the implementation of the `context_server_configuration` method in the `Extension` trait.

## Further Resources

- [Zed Extensions Documentation](https://zed.dev/docs/extensions/developing-extensions)
- [Context Server Extensions](https://zed.dev/docs/extensions/context-servers)
- [Model Context Protocol](https://zed.dev/docs/assistant/model-context-protocol)