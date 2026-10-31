# Bundled sources and assets

- **Noto Sans** is distributed under the SIL Open Font License. The font and its copyright notice are in [assets](assets/OFL.txt).
- **gpui-pre-web 0.3.10**, derived from Zed Industries’ GPUI, is distributed under [Apache-2.0](vendor/gpui-pre-web/LICENSE-APACHE). Cratebook adjusts canvas image readback dimensions for its pinned web-sys version in `canvas_text.rs`.
- **block 0.1.6**, by Steven Sheldon, is distributed under the MIT license as declared in its [manifest](vendor/block/Cargo.toml). The bundled compatibility patch is used by the native build. [Upstream project](https://github.com/SSheldon/rust-block).

Other Rust dependencies and their versions are listed in Cargo.lock. Each dependency retains its own license.
