# Windows interactive checks

## Live Windows checks after 0.1.4 update

Native Computer Use through `node_repl` and `@oai/sky` now captures and operates the gallery on an unlocked desktop. The earlier browser-only runtime and locked-desktop limitations are no longer blocking this pass.

Verified in the native gallery:

- Checkbox input updates the release checklist progress from 2/4 to 3/4.
- Document tabs switch using the Right arrow. Light/dark appearance, the monospace source panel and hiding/restoring the sidebar work.
- Editing the sales worksheet from 32 to 325 and choosing Apply updates the cell, regional total (267 to 560), sparkline and monthly chart.
- Calendar Next changes October 2026 to November 2026 and updates the selected-day agenda.
- Right-clicking the project card opens its menu. Move to opens the submenu; Archive updates the card and dismisses both menus.
- Closing the test app with the native close button removes its window and releases the executable for rebuilding.
- Clipboard entry originally did nothing in fixture editors. The fix handles the platform paste shortcut in shared entries, project forms and search stories. In the final build, pasting `Café 東京` displays the full name, `a12b3456` produces the four demo digits `1234`, and Create project displays the successful result.

Further checks found and fixed two issues: Enter accepted an autocomplete value but its synthetic keyboard click reopened suggestions; tall previews were flex-shrunk and clipped at minimum window width. After the fixes, Enter leaves suggestions closed, the minimum-width worksheet scrolls vertically to both charts, the sheet scrolls horizontally to its final column, and Rust source scrolls to its last lines. Stories, Canvas and Rust source remain reachable in compact mode. The fixes were rebuilt after integrating GPUI Jig 0.1.6 and the current Cratebook 0.1.5 changes.

Additional native checks passed:

| Story or control | Observed result |
| --- | --- |
| Contacts | Pasted Mira filters to Mira Shah and updates the detail card. |
| Task planner | Completing an Inbox task decreases Inbox to 0 and increases Logbook to 2. |
| Project board | Moving a planned item to Done updates both lane counts and completion to 2/5. |
| Settings | Selecting Español updates the language label and closes its list. |
| Data table | Builds sorts numerically as 3, 8, 12, 24. |
| Button | Click increments the count; Space on the focused button increments it again. |
| Radio group | Right arrow changes Read to Write. |
| Collapsible | Page references expands and reveals its reference list. |
| Accordion | Keyboard section opens and displays its content. |
| Checkbox | Select all checks every notification option. |
| Progress | Processing a file advances to 1/4 and updates the track. |
| Scroll area | Wheel scrolling reveals the final contributors inside the bounded list. |
| Meter | Add 1 GB updates usage from 4 to 5 GB and updates the track. |
| Toast | Save shows a revision notification; Dismiss removes it. |
| Workspace navigation | Documents changes the selected section and displayed content. |
| Zoom controls | Increase changes 100% to 110% and updates the slider track. |
| Dialog | Opening export settings, selecting Markdown and saving updates the report preset and closes the panel. |
| Alert dialog | Removal confirmation opens and Cancel closes it; destructive confirmation was not exercised. |
| Drawer | Inspector opens and its close control dismisses it. |
| Popover | Details opens; Got it dismisses it. |
| Preview card | Activating the file trigger displays its preview. |
| Tooltip | Activating Sync workspace shows the synced state and help text. |
| Combobox | Pasting Doc filters to Documentation; choosing it displays the selection and closes the list. |
| Autocomplete | Pasting a custom value and pressing Return accepts it without reopening the list after the fix. |
| Themes and corners | Editor, Tahoe, Mint and Base render in native checks; dark/light and corner changes retain story state. Not every story was tested in every combination. |

The entries remain append-only demo editors. Cursor positioning, selection and full IME editing are not established by these checks. This pass does not establish frame timing, exhaustive keyboard/accessibility behavior, every confirmation/cancellation path, or worksheet drag-reordering. Continue the checklist below for those cases.

Use the extracted release executable on an unlocked Windows desktop. Record the Windows version, display scale, theme, appearance and reproduction steps. Build and unit-test results do not replace these checks.

- Open `cratebook.exe` from Explorer. Confirm the gallery appears without a console window.
- Resize between the minimum size, 1120 by 720, and maximized. Browse every story, including the bottom of the component list. Confirm the compact Stories, Canvas and Rust source controls remain reachable.
- Hide and show the sidebar and toggle Split title bar. Drag, maximize, restore and minimize using the native Windows title bar.
- Switch among Editor, Base, Mint and Tahoe in light and dark mode. Check text contrast, focus indicators, clipping and scroll behavior.
- In Rust source, confirm fixed-width text, scroll to the end, and copy the source into an editor.
- Operate buttons, checkboxes, radio groups, tabs and the date picker using pointer and keyboard. Verify that state survives switching stories and themes.
- Right-click the project card, use Move to and Archive, and verify the card updates. Test dismissal and keyboard navigation.
- Open each dialog, popover, drawer and toast example. Check confirmation, cancellation, focus and dismissal behavior. These examples demonstrate composition; some overlay behavior remains application-owned.
- Close the last window. Confirm no `cratebook.exe` process remains.

## Automated coverage

`cargo test --locked --release` checks the table model. GPUI Jig separately tests state navigation, ranges, accordion state, calendar behavior and documentation examples on Windows and macOS. CI also builds the portable Windows package.

The packaging script targets x86-64 MSVC. ARM64 and signing are not part of this distribution.

## Windows preview 0.1.2 validation, 2026-10-31

- Native Windows MSVC release build and the existing Cratebook table test passed.
- GPUI Jig 0.1.4 passed 5 state/calendar tests and 10 documentation examples on Windows. The preview includes the updated corner presets and Contacts/Tasks stories.
- The PowerShell package script produced a ZIP with the executable, documentation and license notices. ZIP integrity, SHA-256, the x86-64 PE header and GUI subsystem were checked.
- The executable extracted into a separate directory launched with `--theme=tahoe --dark`. Windows reported a responding `Cratebook` window, and the launch log was empty.
- Interactive checks above remain pending. The available Computer Use runtime exposed no native app-control API, so rendering, resizing, keyboard input, clipboard and close behavior were not visually verified. Treat this release as a Windows preview.

## Windows preview 0.1.3 validation, 2026-10-31

- The portable package now tests and builds the explicit x86-64 MSVC target with a static C runtime. The existing table test passed with those settings.
- PE import inspection confirmed that the packaged executable has no `VCRUNTIME`, `MSVCP` or dynamic Universal CRT imports. It still uses native Windows system DLLs, including Direct3D, DirectWrite and ICU. This is dependency inspection, not a test on a clean Windows installation.
- ZIP integrity, SHA-256, required documentation and license files, architecture and GUI subsystem passed verification. The extracted executable launched with `--theme=tahoe --dark`; Windows reported a responding window and no launch log output.
- The preview incorporates GPUI Jig 0.1.5, document-tab keyboard navigation, embedded Phosphor icons and the sales worksheet story. GPUI Jig's 15 tests and documentation examples passed locally and its Windows CI job passed.
- Interactive checks remain pending because native app-control APIs are unavailable in this session, even with Computer Use enabled.
