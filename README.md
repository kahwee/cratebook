# Cratebook

Cratebook is a component workshop for Rust. Build an interactive control with GPUI, try it in a native macOS window, and share the same Rust view in a browser through WebAssembly.

On phones, use the bottom bar to browse **Stories**, try the selected example in **Canvas**, or inspect **Rust source**. **Rust source** shows the actual story module and **Copy code** copies it. Choose **Editor**, **Base**, **Mint**, or **Tahoe** directly in the theme selector (or use its arrow keys) without resetting the live story. Editor is the default, with neutral graphite surfaces, quiet dividers and a compact source pane inspired by Zed. Tahoe extends a cool blue palette across the window, with rounded controls and raised previews inspired by macOS Tahoe. The surfaces are drawn in GPUI on both native and web. The light/dark control works independently in all four themes.

Start with **Card & context menu**. Right-click the project card, open **Move to**, and choose **Archive**. The card updates immediately. Choose **Rust source** to see how the menu positions itself and how the command changes the view.

[Open the live gallery](https://kahwee.github.io/cratebook/) · [Explore the Rust API](https://kahwee.github.io/gpui-jig/api/gpui_jig/)

The gallery also includes settings, document tabs, sortable tables, buttons, radio groups, collapsibles, checkboxes, accordions, batch progress and a scrollable contributor list. Each story has a Rust source view.

On macOS, install from [crates.io](https://crates.io/crates/cratebook) with `cargo install --locked cratebook`, then launch `cratebook`. Use `cargo run --locked --release` during development, or `scripts/install.sh` for a Finder app in `~/Applications`.

For the browser, run `pnpm install --frozen-lockfile`, `pnpm build`, then `pnpm start` (Rust/WASM prerequisites below). Choose a different port with `pnpm start --port 4174`. To distribute a ready-to-run browser gallery locally, run `pnpm pack`. In another project run `pnpm add /path/to/cratebook-0.1.0.tgz` and `pnpm exec cratebook`. The tarball includes the compiled gallery and requires only Node.js at runtime.

The shared unstyled primitives live in [GPUI Jig](https://github.com/kahwee/gpui-jig), the Rust-first UI component library.

The desktop component sidebar docks on the right. Its heading and visibility control share the integrated title bar. Choose **Split title bar** to switch between split and unified chrome. Launch the native Tahoe theme with `cratebook --theme=tahoe`, or add `--dark` for its dark appearance.

## Add a theme

Add a variant to `ui::Theme`, list it in `Theme::ALL`, and implement its name and palette. The selector generates its choices and keyboard focus handles from that list. Light/dark appearance is an independent setting.

## Headless tables

Open **Stories → Data table** and select **Builds** to cycle through numeric ascending order, descending order, and the original order. The [table model](src/table.rs) works with borrowed rows. Define each column once with `ColumnDef::accessor`, then use `format_cell` when the displayed text should differ from the underlying sort value. The [GPUI example](src/stories/table.rs) owns the header controls, accessible roles, spacing, and colors.

## Desktop patterns and component coverage

The gallery covers all 37 Base UI families across 24 stories. Open **Rust source** for the currently displayed implementation.

| Desktop task | Component families |
| --- | --- |
| Edit workspace preferences | Field, Fieldset, Select, Switch, Toggle, Toggle Group |
| Choose notifications and reading modes | Checkbox, Checkbox Group, Radio Group |
| Run commands and navigate sections | Button, Context Menu, Menu, Menubar, Navigation Menu, Toolbar, Separator |
| Manage documents and inspect notes | Tabs, Accordion, Collapsible |
| Create a project and find a workspace | Form, Input, OTP Field, Combobox, Autocomplete |
| Adjust zoom and inspect usage | Number Field, Slider, Meter, Progress |
| Inspect project details or confirm a command | Dialog, Alert Dialog, Drawer, Popover, Preview Card, Tooltip |
| Browse contributors and confirm a save | Avatar, Scroll Area, Toast |

Overlays are bounded canvas demonstrations. The modal examples demonstrate triggers, content and confirmation; production applications supply focus trapping and window-level modality. The form and search examples accept simple keyboard entry and backspace. Production text editors supply selection, clipboard and IME through GPUI’s input handler. The zoom example demonstrates stepper and keyboard slider input.

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

Native tests, the packaged macOS executable build, and the web build run in parallel. The web job builds the gallery, packs the npm CLI, installs the tarball into an empty project and runs its asset check. Pages waits for all three jobs. For a registry release, download the **cratebook-npm** artifact from the successful run for the intended commit, extract the tarball and run `npm publish ./cratebook-VERSION.tgz --access public` with the publisher account. The GPUI Jig Rust library is packaged in its own repository.

Calendar and Project board share the same [month picker](src/ui/month_picker.rs).
In Calendar, a selected date opens its agenda. In Project board, select a task
and choose its due date. Tab into the grid once, then use arrow keys to move by
day or week, Home and End for week boundaries, and Page Up or Page Down for
months. The Previous and Next controls also work with pointer and touch input.

Use **Corners** beside the theme selector to cycle through Standard, Round and
Square. The setting applies across the current session. Standard uses logical
pixel radii of 6 for controls, 10 for cards, 14 for dialogs and 10 for toasts.
Switch thumbs, event dots and other circular indicators keep their shape.
For custom GPUI themes, override individual fields of
`gpui_jig::corners::Corners::STANDARD`. Aligned nested controls can use
`Corners::inset(parent_radius, padding)` to keep their corners concentric.
