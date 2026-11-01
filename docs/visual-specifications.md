# Native gallery visual specifications

The canonical [Markdown comparison](visual-comparison/README.md) uses **960 × 640 client viewports at 1× on both platforms**. Every comparison image must be exactly 960 × 640 pixels, with no scaling or padding. See [AGENTS.md](../AGENTS.md#native-screenshot-comparison-standard) for the mandatory capture procedure. The normal app default remains 1120 × 720 logical pixels; the explicit capture profile uses 960 × 640 at fixed integer origin (32, 32).

The earlier [unmatched comparison](visual-comparison/comparison.html) is historical evidence only. Its 1122 × 749 Windows and 1024 × 674–677 macOS images do not meet the canonical standard.

Use the same source commit, example state, theme and actual logical client bounds when comparing Windows with macOS. The canonical set uses all 30 stories, Editor/light, Standard corners, Preview, visible sidebar at its top and split title bar. `--capture` selects deterministic expanded examples; native builds accept `--story=1` through `--story=30`. `--capture-lower --story=2` adds the worksheet's lower viewport without changing dimensions. The `cratebook-capture` binary selects this same profile without launch arguments. Ordinary app launches keep their normal initial states.

The `Capture macOS gallery comparison` workflow builds and captures the selected commit on `macos-26`. It records the commit and `sw_vers` output with the images. A browser screenshot is not a substitute for a native macOS capture.

## Source-defined dimensions

All sizes below are GPUI logical pixels. PNG pixel dimensions and display scale must be recorded separately. Native title bars and window borders can make the outer image dimensions differ even with the same requested bounds.

| Element | Desktop | Compact |
| --- | --- | --- |
| Requested default window bounds | 1120 × 720 | Resize the same window |
| Minimum window bounds | 360 × 480 | 360 × 480 |
| Compact breakpoint | width ≥ 680 | width < 680 |
| Right sidebar | 218 wide | Separate Stories panel |
| App toolbar | 44 high | 44 high |
| Editor status bar | 24 high | Hidden |
| Content maximum width | 800 including horizontal padding | Available width |
| Content horizontal padding | 28 per side | 16 per side |
| Content top / bottom padding | 24 / 28 | 18 / 16 |
| Gap between content sections | 10 | 10 |
| Shared action minimum height | 28 | 44 |
| Editor sidebar row | 27 high | 48 high |
| Other-theme sidebar row | 32 high | 48 high |
| Sidebar toggle button | 32 × 32 | Hidden |
| Bottom navigation controls | Hidden | 44 high, plus panel padding |

## Typography and corners

| Token | Size |
| --- | --- |
| Body | 13 desktop / 16 compact |
| Story heading, desktop and compact Editor | 24 |
| Story description | 14 |
| Eyebrow | 12 |
| Source code | 13 |
| Compact Stories heading | 26 |
| Compact cover heading in Base, Mint and Tahoe | 32 |
| Standard control / card / dialog / toast radius | 6 / 10 / 14 / 10 |
| Round control / card / dialog / toast radius | 12 / 18 / 22 / 16 |
| Square radii | 0 |

## Intentional platform differences

| Property | Windows | macOS |
| --- | --- | --- |
| UI font | Segoe UI explicitly requested | GPUI platform default; family not explicitly requested |
| Source font | Consolas | Menlo |
| Native window chrome | Separate Windows title bar | Transparent macOS title bar and traffic lights |
| Toolbar left inset outside fullscreen | 12 | 84 for traffic-light clearance |
| Traffic-light position | Not applicable | (15, 17) |
| Title-bar dragging and double-click handler | Native Windows title bar | App title region invokes native move / double-click behavior |

Editor/light uses canvas `#fafafa`, sidebar `#f0f0f1`, border `#d6d6d8`, text `#262629`, muted text `#5e5e64` and selected surface `#dcdce0`. Matching token values do not establish identical rasterization or text wrapping.

## Capture acceptance

Record original PNG width and height for every image. Preserve original images, then fit them side by side only for the overview. Label overview scaling. Inspect each image for a blank/locked screen, wrong window, wrong story, clipped framing or loading state before accepting it. A blank Windows capture does not establish a rendering defect in the app. First-viewport screenshots do not prove that all offscreen content or interaction states work.

Measure image differences only after distinguishing app content, platform chrome and display scale. Do not infer accessibility compliance, IME behavior or frame-rate performance from screenshot parity.
