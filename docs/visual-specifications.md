# Native gallery visual specifications

Use the same source commit, initial story state, theme and requested logical window bounds when comparing Windows with macOS. The reference capture set uses all 30 stories, Editor/light, Standard corners, Preview, the visible sidebar and the split title bar. Native builds accept `--story=1` through `--story=30` to select a fresh story at launch. The existing `--theme=editor` and `--dark` flags select appearance.

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
