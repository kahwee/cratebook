# Windows and macOS — same-size native comparison

**Standard: 960 × 640 logical-pixel client viewport at 1×; every comparison PNG is exactly 960 × 640 pixels.** Images below display at 480 × 320; click for original pixels. No app content is resized or padded. Native originals are preserved separately.

Source revision: `2930b2a76aadabdd078939ff62be826a0052d989` on both platforms. Editor · light · Standard corners · Preview · sidebar at top. The capture profile deliberately expands hidden components and seeds example data; it is not the ordinary launch state. See [AGENTS.md](../../AGENTS.md#native-screenshot-comparison-standard) for the capture standard and [size specifications](../visual-specifications.md) for logical dimensions.

Windows native frames measure 962 × 672; extracting `(1, 31, 961, 671)` preserves the entire 960 × 640 client area. macOS captures already measure 960 × 640, including its transparent title bar. Native fonts, traffic lights and toolbar insets intentionally differ. Windows may include a cursor halo from the host. These are appearance examples, not a claim of identical rasterization, interaction correctness, accessibility or frame-rate performance.

The worksheet has a separately labeled lower viewport so its charts are visible without stretching the first view. [Manifest and hashes](standard-960x640/manifest.json) record every original size and extraction rectangle. [Capture findings](standard-960x640/findings.md) summarize the inspected pairs. [Earlier unmatched captures](comparison.html) are historical evidence and do not meet this standard.

| Component example / pictured state | Windows — 960 × 640 | macOS — 960 × 640 |
| --- | --- | --- |
| **01 Kitchen sink**<br>Release checklist with 2 of 4 checks complete<br><code>card, checkbox, progress, toggle_group, toolbar</code> | <a href="standard-960x640/windows/01.png"><img src="standard-960x640/windows/01.png" width="480" alt="windows: Kitchen sink 01"></a> | <a href="standard-960x640/macos/01.png"><img src="standard-960x640/macos/01.png" width="480" alt="macos: Kitchen sink 01"></a> |
| **02 Sales worksheet**<br>Jan-Jun sales, B2 selected, populated charts; lower view below<br><code>card, button, scroll_area, toolbar</code> | <a href="standard-960x640/windows/02.png"><img src="standard-960x640/windows/02.png" width="480" alt="windows: Sales worksheet 02"></a> | <a href="standard-960x640/macos/02.png"><img src="standard-960x640/macos/02.png" width="480" alt="macos: Sales worksheet 02"></a> |
| **02 Sales worksheet — lower viewport**<br>Jan-Jun sales, B2 selected, populated charts; lower view below<br><code>card, button, scroll_area, toolbar</code> | <a href="standard-960x640/windows/02-lower.png"><img src="standard-960x640/windows/02-lower.png" width="480" alt="windows: Sales worksheet 02-lower"></a> | <a href="standard-960x640/macos/02-lower.png"><img src="standard-960x640/macos/02-lower.png" width="480" alt="macos: Sales worksheet 02-lower"></a> |
| **03 Contacts**<br>Ada Chen contact card and favorites<br><code>avatar, button, card, toggle, input</code> | <a href="standard-960x640/windows/03.png"><img src="standard-960x640/windows/03.png" width="480" alt="windows: Contacts 03"></a> | <a href="standard-960x640/macos/03.png"><img src="standard-960x640/macos/03.png" width="480" alt="macos: Contacts 03"></a> |
| **04 Task planner**<br>Inbox with scheduled action and list counts<br><code>checkbox, toggle_group, button</code> | <a href="standard-960x640/windows/04.png"><img src="standard-960x640/windows/04.png" width="480" alt="windows: Task planner 04"></a> | <a href="standard-960x640/macos/04.png"><img src="standard-960x640/macos/04.png" width="480" alt="macos: Task planner 04"></a> |
| **05 Project board**<br>Five cards in three populated lanes<br><code>card, button, toggle_group, toolbar</code> | <a href="standard-960x640/windows/05.png"><img src="standard-960x640/windows/05.png" width="480" alt="windows: Project board 05"></a> | <a href="standard-960x640/macos/05.png"><img src="standard-960x640/macos/05.png" width="480" alt="macos: Project board 05"></a> |
| **06 Calendar**<br>October 2026, October 12 selected with focus block<br><code>date_grid, card, button, toolbar</code> | <a href="standard-960x640/windows/06.png"><img src="standard-960x640/windows/06.png" width="480" alt="windows: Calendar 06"></a> | <a href="standard-960x640/macos/06.png"><img src="standard-960x640/macos/06.png" width="480" alt="macos: Calendar 06"></a> |
| **07 Card & context menu**<br>Open actions menu and destination submenu<br><code>context_menu, card</code> | <a href="standard-960x640/windows/07.png"><img src="standard-960x640/windows/07.png" width="480" alt="windows: Card &amp; context menu 07"></a> | <a href="standard-960x640/macos/07.png"><img src="standard-960x640/macos/07.png" width="480" alt="macos: Card &amp; context menu 07"></a> |
| **08 Settings**<br>Language, switch and terminal-position preferences<br><code>field, fieldset, select, switch, toggle, toggle_group</code> | <a href="standard-960x640/windows/08.png"><img src="standard-960x640/windows/08.png" width="480" alt="windows: Settings 08"></a> | <a href="standard-960x640/macos/08.png"><img src="standard-960x640/macos/08.png" width="480" alt="macos: Settings 08"></a> |
| **09 Document tabs**<br>Selected document with long and closable tabs<br><code>tabs</code> | <a href="standard-960x640/windows/09.png"><img src="standard-960x640/windows/09.png" width="480" alt="windows: Document tabs 09"></a> | <a href="standard-960x640/macos/09.png"><img src="standard-960x640/macos/09.png" width="480" alt="macos: Document tabs 09"></a> |
| **10 Data table**<br>Package/build columns with numeric sort controls<br><code>table</code> | <a href="standard-960x640/windows/10.png"><img src="standard-960x640/windows/10.png" width="480" alt="windows: Data table 10"></a> | <a href="standard-960x640/macos/10.png"><img src="standard-960x640/macos/10.png" width="480" alt="macos: Data table 10"></a> |
| **11 Button**<br>Run action, zero activations (plain GPUI example)<br><code>plain GPUI button; Jig button in Contacts and Board</code> | <a href="standard-960x640/windows/11.png"><img src="standard-960x640/windows/11.png" width="480" alt="windows: Button 11"></a> | <a href="standard-960x640/macos/11.png"><img src="standard-960x640/macos/11.png" width="480" alt="macos: Button 11"></a> |
| **12 Radio group**<br>Read selected, Write and Source options<br><code>radio_group</code> | <a href="standard-960x640/windows/12.png"><img src="standard-960x640/windows/12.png" width="480" alt="windows: Radio group 12"></a> | <a href="standard-960x640/macos/12.png"><img src="standard-960x640/macos/12.png" width="480" alt="macos: Radio group 12"></a> |
| **13 Collapsible**<br>Expanded page references<br><code>collapsible</code> | <a href="standard-960x640/windows/13.png"><img src="standard-960x640/windows/13.png" width="480" alt="windows: Collapsible 13"></a> | <a href="standard-960x640/macos/13.png"><img src="standard-960x640/macos/13.png" width="480" alt="macos: Collapsible 13"></a> |
| **14 Accordion**<br>Components section expanded<br><code>accordion</code> | <a href="standard-960x640/windows/14.png"><img src="standard-960x640/windows/14.png" width="480" alt="windows: Accordion 14"></a> | <a href="standard-960x640/macos/14.png"><img src="standard-960x640/macos/14.png" width="480" alt="macos: Accordion 14"></a> |
| **15 Checkbox**<br>Mixed parent and checked/unchecked children<br><code>checkbox, checkbox_group</code> | <a href="standard-960x640/windows/15.png"><img src="standard-960x640/windows/15.png" width="480" alt="windows: Checkbox 15"></a> | <a href="standard-960x640/macos/15.png"><img src="standard-960x640/macos/15.png" width="480" alt="macos: Checkbox 15"></a> |
| **16 Progress**<br>2 of 4 files processed<br><code>progress</code> | <a href="standard-960x640/windows/16.png"><img src="standard-960x640/windows/16.png" width="480" alt="windows: Progress 16"></a> | <a href="standard-960x640/macos/16.png"><img src="standard-960x640/macos/16.png" width="480" alt="macos: Progress 16"></a> |
| **17 Scroll area**<br>Bounded contributor list with avatar fallbacks<br><code>scroll_area, avatar</code> | <a href="standard-960x640/windows/17.png"><img src="standard-960x640/windows/17.png" width="480" alt="windows: Scroll area 17"></a> | <a href="standard-960x640/macos/17.png"><img src="standard-960x640/macos/17.png" width="480" alt="macos: Scroll area 17"></a> |
| **18 Meter**<br>4 GB of 10 GB used<br><code>meter</code> | <a href="standard-960x640/windows/18.png"><img src="standard-960x640/windows/18.png" width="480" alt="windows: Meter 18"></a> | <a href="standard-960x640/macos/18.png"><img src="standard-960x640/macos/18.png" width="480" alt="macos: Meter 18"></a> |
| **19 Toast**<br>Draft saved, revision 1, dismiss control visible<br><code>toast</code> | <a href="standard-960x640/windows/19.png"><img src="standard-960x640/windows/19.png" width="480" alt="windows: Toast 19"></a> | <a href="standard-960x640/macos/19.png"><img src="standard-960x640/macos/19.png" width="480" alt="macos: Toast 19"></a> |
| **20 Workspace navigation**<br>View menu open, sections and divider visible<br><code>menu, menubar, navigation_menu, separator, toolbar</code> | <a href="standard-960x640/windows/20.png"><img src="standard-960x640/windows/20.png" width="480" alt="windows: Workspace navigation 20"></a> | <a href="standard-960x640/macos/20.png"><img src="standard-960x640/macos/20.png" width="480" alt="macos: Workspace navigation 20"></a> |
| **21 Project form**<br>Desktop workspace and four-digit demo code 1234<br><code>form, input, otp_field</code> | <a href="standard-960x640/windows/21.png"><img src="standard-960x640/windows/21.png" width="480" alt="windows: Project form 21"></a> | <a href="standard-960x640/macos/21.png"><img src="standard-960x640/macos/21.png" width="480" alt="macos: Project form 21"></a> |
| **22 Zoom controls**<br>100 percent with stepper and slider<br><code>number_field, slider</code> | <a href="standard-960x640/windows/22.png"><img src="standard-960x640/windows/22.png" width="480" alt="windows: Zoom controls 22"></a> | <a href="standard-960x640/macos/22.png"><img src="standard-960x640/macos/22.png" width="480" alt="macos: Zoom controls 22"></a> |
| **23 Dialog**<br>Export settings open with PDF/Markdown/CSV options<br><code>dialog, toggle_group</code> | <a href="standard-960x640/windows/23.png"><img src="standard-960x640/windows/23.png" width="480" alt="windows: Dialog 23"></a> | <a href="standard-960x640/macos/23.png"><img src="standard-960x640/macos/23.png" width="480" alt="macos: Dialog 23"></a> |
| **24 Alert dialog**<br>Confirmation open with Remove draft and Cancel<br><code>alert_dialog</code> | <a href="standard-960x640/windows/24.png"><img src="standard-960x640/windows/24.png" width="480" alt="windows: Alert dialog 24"></a> | <a href="standard-960x640/macos/24.png"><img src="standard-960x640/macos/24.png" width="480" alt="macos: Alert dialog 24"></a> |
| **25 Drawer**<br>Inspector open with document metadata<br><code>drawer</code> | <a href="standard-960x640/windows/25.png"><img src="standard-960x640/windows/25.png" width="480" alt="windows: Drawer 25"></a> | <a href="standard-960x640/macos/25.png"><img src="standard-960x640/macos/25.png" width="480" alt="macos: Drawer 25"></a> |
| **26 Popover**<br>Details popup open<br><code>popover</code> | <a href="standard-960x640/windows/26.png"><img src="standard-960x640/windows/26.png" width="480" alt="windows: Popover 26"></a> | <a href="standard-960x640/macos/26.png"><img src="standard-960x640/macos/26.png" width="480" alt="macos: Popover 26"></a> |
| **27 Preview card**<br>Document preview open with research summary<br><code>preview_card</code> | <a href="standard-960x640/windows/27.png"><img src="standard-960x640/windows/27.png" width="480" alt="windows: Preview card 27"></a> | <a href="standard-960x640/macos/27.png"><img src="standard-960x640/macos/27.png" width="480" alt="macos: Preview card 27"></a> |
| **28 Tooltip**<br>Sync hint visible alongside trigger<br><code>tooltip</code> | <a href="standard-960x640/windows/28.png"><img src="standard-960x640/windows/28.png" width="480" alt="windows: Tooltip 28"></a> | <a href="standard-960x640/macos/28.png"><img src="standard-960x640/macos/28.png" width="480" alt="macos: Tooltip 28"></a> |
| **29 Combobox**<br>Workspace suggestions visible<br><code>combobox</code> | <a href="standard-960x640/windows/29.png"><img src="standard-960x640/windows/29.png" width="480" alt="windows: Combobox 29"></a> | <a href="standard-960x640/macos/29.png"><img src="standard-960x640/macos/29.png" width="480" alt="macos: Combobox 29"></a> |
| **30 Autocomplete**<br>Project-tag suggestions visible<br><code>autocomplete</code> | <a href="standard-960x640/windows/30.png"><img src="standard-960x640/windows/30.png" width="480" alt="windows: Autocomplete 30"></a> | <a href="standard-960x640/macos/30.png"><img src="standard-960x640/macos/30.png" width="480" alt="macos: Autocomplete 30"></a> |

## Component coverage index

Every public visual component has an example above; composite stories cover several components. `corners` and `state` are nonvisual configuration/state helpers used by these examples.

| Component | Example stories |
| --- | --- |
| `accordion` | 14 Accordion |
| `alert_dialog` | 24 Alert dialog |
| `autocomplete` | 30 Autocomplete |
| `avatar` | 03 Contacts, 17 Scroll area |
| `button` | 02 Sales worksheet, 03 Contacts, 04 Task planner, 05 Project board, 06 Calendar, 11 Button |
| `card` | 01 Kitchen sink, 02 Sales worksheet, 03 Contacts, 05 Project board, 06 Calendar, 07 Card & context menu |
| `checkbox` | 01 Kitchen sink, 04 Task planner, 15 Checkbox |
| `checkbox_group` | 15 Checkbox |
| `collapsible` | 13 Collapsible |
| `combobox` | 29 Combobox |
| `context_menu` | 07 Card & context menu |
| `date_grid` | 06 Calendar |
| `dialog` | 23 Dialog |
| `drawer` | 25 Drawer |
| `field` | 08 Settings |
| `fieldset` | 08 Settings |
| `form` | 21 Project form |
| `input` | 03 Contacts, 21 Project form |
| `menu` | 20 Workspace navigation |
| `menubar` | 20 Workspace navigation |
| `meter` | 18 Meter |
| `navigation_menu` | 20 Workspace navigation |
| `number_field` | 22 Zoom controls |
| `otp_field` | 21 Project form |
| `popover` | 26 Popover |
| `preview_card` | 27 Preview card |
| `progress` | 01 Kitchen sink, 16 Progress |
| `radio_group` | 12 Radio group |
| `scroll_area` | 02 Sales worksheet, 17 Scroll area |
| `select` | 08 Settings |
| `separator` | 20 Workspace navigation |
| `slider` | 22 Zoom controls |
| `switch` | 08 Settings |
| `table` | 10 Data table |
| `tabs` | 09 Document tabs |
| `toast` | 19 Toast |
| `toggle` | 03 Contacts, 08 Settings |
| `toggle_group` | 01 Kitchen sink, 04 Task planner, 05 Project board, 08 Settings, 23 Dialog |
| `toolbar` | 01 Kitchen sink, 02 Sales worksheet, 05 Project board, 06 Calendar, 20 Workspace navigation |
| `tooltip` | 28 Tooltip |
