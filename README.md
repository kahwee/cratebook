# Cratebook

Cratebook is a component workshop for Rust. Build an interactive control with GPUI, try it in a native macOS window, and share the same Rust view in a browser through WebAssembly.

On phones, use the bottom bar to browse **Stories**, try the selected example in **Canvas**, or inspect **Rust source**. **Rust source** shows the actual story module and **Copy code** copies it. Switch **Theme: Base** to **Mint** to restyle the live example without resetting it. Light and dark appearance work with either theme.

Start with the document tabs story. Open a long filename, mark it unsaved, close the active tab, then switch appearances to see how those states read in light and dark.

[Open the live gallery](https://kahwee.github.io/cratebook/) · [Preview the document tab story](https://github.com/kahwee/cratebook/blob/main/preview.png)

Explore document tabs, buttons, radio groups, and disclosures. Both targets use the controls in `src/ui`, originally developed for Commonleaf.

On macOS, run `cargo install --locked --path .` from the checkout, then launch `cratebook`. Use `cargo run --locked --release` during development, or `scripts/install.sh` for a Finder app in `~/Applications`. To produce a Rust source archive, run `cargo package --locked`.

For the browser, run `pnpm install --frozen-lockfile`, `pnpm build`, then `pnpm start` (Rust/WASM prerequisites below). Choose a different port with `pnpm start --port 4174`. To distribute a ready-to-run browser gallery locally, run `pnpm pack`. In another project run `pnpm add /path/to/cratebook-0.1.0.tgz` and `pnpm exec cratebook`. The tarball includes the compiled gallery and requires only Node.js at runtime.

The shared Button, Collapsible, and Radio Group primitives live in [GPUI Jig](https://github.com/kahwee/gpui-jig), the Rust-first UI component library.

## Headless tables

Open **Stories → Data table** and select **Builds** to cycle through numeric ascending order, descending order, and the original order. The [table model](src/table.rs) works with borrowed rows. Define each column once with `ColumnDef::accessor`, then use `format_cell` when the displayed text should differ from the underlying sort value. The [GPUI example](src/stories/table.rs) owns the header controls, accessible roles, spacing, and colors.

## Add a story

A story owns its title, instructions, and a live GPUI preview. The [button story](src/stories/button.rs) is a complete example. Its counter belongs to the preview and keeps its value when you visit another story.

1. Create a module in `src/stories/`. Have its `story` constructor return a `Story` with an entity for the preview, `path: file!()`, and `source: include_str!("your_module.rs")`.
2. Declare the module and register its constructor in the [catalog](src/stories/mod.rs). The sidebar and preview header read the same metadata.
3. Use the controls in [src/ui](src/ui) and check the interaction in both appearances. The [gallery shell](src/gallery.rs) owns navigation, layout and appearance.

To build your own look, start with `ui::button`, `ui::collapsible_trigger`, or `ui::RadioGroup`. These provide interaction and focus behavior; the caller adds layout, colors, focus indicators, and content using GPUI style methods. The button story shows this directly.

Contributions are welcome as GitHub issues or pull requests. For a new story, include the interaction it demonstrates and how to reproduce it.

## Browser gallery

The browser runs the same Rust stories in a GPUI canvas. The renderer tries WebGPU and can fall back to WebGL2. The native app uses the system font. The web gallery embeds Noto Sans under the license in `assets/OFL.txt`.

Both builds use stable Rust 1.99.1, pinned in `rust-toolchain.toml`. From the checkout, install the browser target with `rustup target add wasm32-unknown-unknown`. Install the bindings generator with `cargo install --locked wasm-bindgen-cli --version 0.2.130`. Then run:

```sh
pnpm build
pnpm start
```

Open http://127.0.0.1:4173. Rebuild and reload after changing a Rust story. For hosting, serve the contents of `web/` over HTTPS, including the generated `pkg/` directory.

The local gpui-pre-web 0.3.10 patch adapts the canvas readback dimensions to the integer signature in web-sys 0.3.107.

## License

Cratebook’s original code is available under the [MIT License](LICENSE). Bundled fonts and third-party code retain their own licenses. See [third-party notices](THIRD_PARTY.md).

## Verify a release candidate

The native and web jobs run in parallel. The web job builds the gallery, packs the npm CLI, installs the tarball into an empty project and runs its asset check. Pages waits for both jobs. For a registry release, download the **cratebook-npm** artifact from the successful run for the intended commit, extract the tarball and run `npm publish ./cratebook-VERSION.tgz --access public` with the publisher account. The GPUI Jig Rust library is packaged in its own repository.
