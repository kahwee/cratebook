# Cratebook

Cratebook is a component workshop for Rust. Build an interactive control with GPUI, try it in a native Windows or macOS window, and share the same Rust view in a browser through WebAssembly.

On phones, use the bottom bar to browse **Stories**, try the selected example in **Canvas**, or inspect **Rust source**. **Rust source** shows the actual story module with theme-aware lexical highlighting; **Copy code** copies the original text. Choose **Editor**, **Base**, **Mint**, or **Tahoe** directly in the theme selector (or use its arrow keys) without resetting the live story. Editor is the default, with neutral graphite surfaces, quiet dividers and a compact source pane inspired by Zed. Tahoe extends a cool blue palette across the window, with rounded controls and raised previews inspired by macOS Tahoe. The surfaces are drawn in GPUI on both native and web. The light/dark control works independently in all four themes.

Start with **Card & context menu**. Right-click the project card, open **Move to**, and choose **Archive**. The card updates immediately. Choose **Rust source** to see how the menu positions itself and how the command changes the view.

[Open the live gallery](https://kahwee.github.io/cratebook/) · [Explore the Rust API](https://kahwee.github.io/gpui-jig/api/gpui_jig/)

The gallery also includes settings, document tabs, sortable tables, buttons, radio groups, collapsibles, checkboxes, accordions, batch progress and a scrollable contributor list. Each story has a Rust source view.

On Windows or macOS, install from [crates.io](https://crates.io/crates/cratebook) with `cargo install --locked cratebook`, then launch `cratebook`. Use `cargo run --locked --release --bin cratebook` during development. On macOS, `scripts/install.sh` creates a Finder app in `~/Applications`.

## Windows desktop

Download the Windows x86-64 ZIP from [GitHub Releases](https://github.com/kahwee/cratebook/releases), extract it, and open `cratebook.exe`. The portable app does not require Rust or Node.js. From 0.1.3, the portable package statically links the C runtime so a separate Visual C++ runtime installation is not required. It uses the native Windows title bar, Segoe UI for controls and Consolas for Rust source. Closing the last window exits the app. The executable is unsigned.

To build from source, install Rust 1.99.1 for `x86_64-pc-windows-msvc` and Visual Studio Build Tools with Desktop development with C++ and the Windows SDK. In PowerShell, run:

```powershell
cargo test --locked --release
./scripts/package-windows.ps1
```

The script tests and builds the x86-64 MSVC target with a static C runtime, then creates `dist/cratebook-VERSION-windows-x86_64.zip` and a `.zip.sha256` checksum. Compare the checksum with `Get-FileHash -Algorithm SHA256` before distributing a downloaded archive. Windows CI runs native tests and uploads the ZIP on each build. Follow [the interactive checklist](docs/windows-testing.md) to validate rendering and input; automated tests do not cover every story.

For the browser, run `pnpm install --frozen-lockfile`, `pnpm build`, then `pnpm start` (Rust/WASM prerequisites below). Choose a different port with `pnpm start --port 4174`. To distribute a ready-to-run browser gallery locally, run `pnpm pack`. In another project run `pnpm add /path/to/cratebook-0.1.0.tgz` and `pnpm exec cratebook`. The tarball includes the compiled gallery and requires only Node.js at runtime.

Many bundled stories use the shared unstyled primitives in [GPUI Jig](https://github.com/kahwee/gpui-jig). A story can also render ordinary GPUI elements or use a different component library; the [button story](src/stories/button.rs) uses GPUI directly. Jig is a dependency of this gallery build, not a requirement for the components you bring into your own stories.

Cratebook and GPUI Jig use [`gpui-pre`](https://crates.io/crates/gpui-pre), a published snapshot of [Zed's GPUI framework](https://github.com/zed-industries/zed/tree/main/crates/gpui) maintained by the [Longbridge GPUI Kit project](https://github.com/longbridge/gpui-component). Zed also publishes an official [`gpui` crate](https://crates.io/crates/gpui), but its published releases can lag behind the GPUI code Zed uses directly from its repository. The snapshot lets Cratebook use a more recent GPUI version from crates.io; it is not published by Zed.

Cratebook also uses [`gpui-pre-platform`](https://crates.io/crates/gpui-pre-platform) to select the appropriate platform backends for native windows and the browser. It is Longbridge's published snapshot of Zed's [`gpui_platform`](https://github.com/zed-industries/zed/tree/main/crates/gpui_platform), from the same versioned snapshot family as `gpui-pre`. Both are currently at 0.3.10 in Cratebook.

The desktop component sidebar docks on the right. Its heading and visibility control share the integrated title bar. Choose **Split title bar** to switch between split and unified chrome. Launch the native Tahoe theme with `cratebook --theme=tahoe`, or add `--dark` for its dark appearance.

## Add a theme

Add a variant to `ui::Theme`, list it in `Theme::ALL`, and implement its name and palette. The selector generates its choices and keyboard focus handles from that list. Light/dark appearance is an independent setting.

## Headless tables

Open **Stories → Data table** and select **Builds** to cycle through numeric ascending order, descending order, and the original order. The [GPUI Jig table model](https://kahwee.github.io/gpui-jig/api/gpui_jig/table/) works with borrowed rows and is imported directly by the gallery. Define each column once with `ColumnDef::accessor`, then use `format_cell` when the displayed text should differ from the underlying sort value. The [GPUI example](src/stories/table.rs) owns the header controls, accessible roles, spacing, and colors.

## Desktop patterns and component coverage

The gallery demonstrates desktop component families across 30 stories. Open **Rust source** for the currently displayed implementation. The [Windows/macOS Markdown comparison](docs/visual-comparison/README.md) shows every story side by side at the same **960 × 640** viewport, with expanded component examples and a coverage index. The [capture standard](AGENTS.md#native-screenshot-comparison-standard) defines dimensions, framing and validation.

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

Overlays are bounded canvas demonstrations. The modal examples demonstrate triggers, content and confirmation; production applications supply focus trapping and window-level modality. The form and search examples demonstrate simple keyboard entry, backspace and clipboard paste. Production text editors supply selection, full editing and IME through GPUI’s input handler. The zoom example demonstrates stepper and keyboard slider input.

## Calendar, corners and workbench examples

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

The **Contacts** and **Task planner** kitchen sinks use the same responsive
[navigation and detail layout](src/stories/workbench.rs). In Contacts, search
by name or team, mark favorites, and copy a work email. In Task planner, add a
to-do to Inbox, schedule it for Today, check it off, then reopen it in Logbook.
Both stories keep their fixture data in memory for the gallery session.

## Add a story

A story owns its title, instructions, and a live GPUI preview. The [button story](src/stories/button.rs) is a complete example. Its counter belongs to the preview and keeps its value when you visit another story.

1. Create a module in `src/stories/`. Have its `story` constructor return a `Story` with an entity for the preview, `path: file!()`, and `source: include_str!("your_module.rs")`.
2. Declare the module and register its constructor in the [catalog](src/stories/mod.rs). The sidebar and preview header read the same metadata.
3. Use the controls in [src/ui](src/ui) and check the interaction in both appearances. The [gallery shell](src/gallery.rs) owns navigation, layout and appearance.

Build your own look with ordinary GPUI elements, or use `ui::button`, `ui::collapsible_trigger`, or `ui::RadioGroup` for shared Jig primitives. Add layout, colors, focus indicators, and content with GPUI style methods. The button story shows a plain GPUI control; the document tabs story shows Jig parts with gallery-owned styling and state.

Contributions are welcome as GitHub issues or pull requests. For a new story, include the interaction it demonstrates and how to reproduce it.

## Browser gallery

The browser runs the same Rust stories in a GPUI canvas. The renderer tries WebGPU and can fall back to WebGL2. The native app uses the system font. The web gallery embeds Noto Sans for the interface and JetBrains Mono for source code; their licenses are in `assets/`.

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

The independent **Quick feedback** workflow checks formatting, the locked Cargo manifest and the source highlighter's focused tests. It can finish before the full build. Platform-specific caches speed up later builds; the first build and builds after dependency or toolchain changes can be cold. Changes limited to documentation or screenshot tooling do not start a release build; packaged documentation is refreshed on the next build.

Native tests on macOS and Windows, packaged macOS and Windows builds, and the web build run in parallel. The web job builds the gallery, packs the npm CLI, installs the tarball into an empty project and runs its asset check. Pages waits for all of these jobs. For a registry release, download the **cratebook-npm** artifact from the successful run for the intended commit, extract the tarball and run `npm publish ./cratebook-VERSION.tgz --access public` with the publisher account. The npm package is not yet published. The GPUI Jig Rust library is packaged in its own repository.
