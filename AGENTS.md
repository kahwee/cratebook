# Cratebook development

Pull the latest `main` before starting work and before pushing. Preserve other
contributors' changes. Push completed, validated changes to `main` when the user
has authorized that workflow; never force push. Record the exact source revision
used for screenshots rather than silently mixing builds from different commits.

## Native screenshot comparison standard

- Use **960 × 640 logical pixels of app client area**, captured at **1× scale**.
  Every published comparison PNG must be **exactly 960 × 640 physical pixels**
  on Windows and macOS. Reject a size mismatch. Never stretch, shrink, pad, or
  silently crop app content to make mismatched viewports appear equivalent.
- Launch `cargo run --locked --release --bin cratebook -- --capture --story=N`
  on macOS. The `cratebook-capture` binary selects the same profile for native
  Windows Computer Use, whose launch API does not take CLI arguments.
- Use Editor, light, Standard corners, Preview, split toolbar and visible sidebar.
  The capture profile uses deterministic expanded examples: show popup contents,
  suggestions, a tooltip, an expanded disclosure and a saved toast. Document the
  pictured state for each story; a trigger alone is not sufficient evidence of
  its hidden component. Keep ordinary launch defaults unchanged.
- Save untouched native screenshots under `native/` before extracting client
  images. On Windows, a verified 962 × 669 native window at 1× has a 960 × 640
  client rectangle at `(1, 28, 961, 668)`. Remove only this native border/title
  bar. Reject other native dimensions and investigate DPI/viewport settings.
  macOS transparent-titlebar captures must already be 960 × 640. Preserve its
  traffic lights and intentional platform toolbar inset. Keep native font
  differences; do not normalize fonts to conceal them.
- Capture all 30 stories from one source revision on both platforms. Inspect
  every saved image for correct story/state, complete framing, blank capture,
  occlusion and clipping. Record original dimensions, crop rectangle, commit,
  platform, profile and SHA-256 hashes in the manifest. Retain original files.
- Keep sidebar scroll at its top in canonical images on both platforms. If a
  story extends below the first viewport, provide a separately labeled lower
  viewport screenshot at the same 960 × 640 size on each platform. Do not present
  a resized full-page composite as a standard viewport capture.
- Generate `docs/visual-comparison/README.md` with one row per story and columns
  for example/state, **Windows** and **macOS**, embedding relative image links.
  Include dimensions, source commit and limitations. Update source specs and
  the manifest together. Archive older unmatched captures as historical evidence;
  do not label them canonical. Screenshots do not prove interaction, accessibility
  or frame-rate performance.
- Use the Computer Use skill and supported native `sky` API for Windows visual
  interaction. Stop on a locked desktop or failed recovery; never accept a black
  screenshot or substitute a browser render for native Windows/macOS evidence.

## Checks

Run `cargo fmt --all --check`, `cargo test --locked`, and the native release build
for changed Rust code. Validate the screenshot manifest and Markdown coverage
after regenerating evidence. macOS captures use the `platform-screenshots.yml`
workflow; a green build alone is not visual validation.
