# Same-size capture findings

Source: `2930b2a76aadabdd078939ff62be826a0052d989` on both platforms.
Native macOS run: [41266134730](https://github.com/kahwee/cratebook/actions/runs/41266134730).

All 31 pairs were inspected on six overview sheets; Windows views were also
inspected during native capture. All **62 comparison PNGs measure 960 × 640**,
representing the same 960 × 640 logical client viewport at 1×. The original
Windows tool payloads are JPEG and are preserved without re-encoding under
`windows/native/`; macOS originals are PNG under `macos/native/`. The comparison
PNGs remove only the verified Windows native frame, without scaling app content.
The manifest records original and comparison hashes, framing and provenance.

- All 30 expected story views show populated examples on both platforms. Hidden
  components are shown open, including context menu/submenu, accordion, disclosure,
  toast, menus, dialog, alert, drawer, popover, preview, tooltip and suggestions.
- The worksheet's separate lower viewport shows its complete charts on both
  platforms. The first viewport is intentionally retained to show actual scrolling
  needs at the standard size.
- Content, fixture values, control order and overall layout agree. Platform fonts
  produce visible differences in weight, glyphs, line wrapping and intrinsic
  control sizing. These are observable differences, not proof of a defect.
- macOS traffic lights and its wider left toolbar inset are retained. Windows
  native title-bar buttons appear only in preserved native originals. Both
  canonical images contain the app toolbar and status strip at the same viewport
  dimensions. macOS native rounded corner pixels remain visible.
- Both sidebars start at the top, including stories whose selected row is below
  the viewport. Windows captures retain the host's pointer halo; it is capture
  decoration, not gallery styling. Do not use these images for raw pixel equality.
- No blank, locked, wrong-story or occluded comparison image was accepted.
  Screenshot inspection does not establish keyboard/IME/accessibility correctness
  or smooth frame pacing. The component coverage index describes visible examples,
  not exhaustive testing of every API or interaction state.

Later upstream changes, including document panel corner clipping, may exist on
`main`. This comparison deliberately preserves a single shared source revision
rather than mixing newer views into one platform's set.
