# Windows interactive checks

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
