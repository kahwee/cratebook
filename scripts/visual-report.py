"""Validate native captures and generate the canonical Markdown comparison.

Requires Pillow. Usage: python scripts/visual-report.py CAPTURE_DIRECTORY
Input: PLATFORM/native/NN.png, PLATFORM/commit.txt (same revision on both).
Original screenshots are never overwritten. Client images are never resized.
"""
from pathlib import Path
import hashlib
import html
import json
import re
import shutil
import sys
from PIL import Image, ImageDraw, ImageFont, ImageStat

repo = Path(__file__).resolve().parents[1]
root = Path(sys.argv[1]).resolve()
destination = repo / 'docs/visual-comparison/standard-960x640'

# Story number, visible example state, and public components demonstrated.
examples = [
 ('Kitchen sink', 'Release checklist with 2 of 4 checks complete', 'card, checkbox, progress, toggle_group, toolbar'),
 ('Sales worksheet', 'Jan-Jun sales, B2 selected, populated charts; lower view below', 'card, button, scroll_area, toolbar'),
 ('Contacts', 'Ada Chen contact card and favorites', 'avatar, button, card, toggle, input'),
 ('Task planner', 'Inbox with scheduled action and list counts', 'checkbox, toggle_group, button'),
 ('Project board', 'Five cards in three populated lanes', 'card, button, toggle_group, toolbar'),
 ('Calendar', 'October 2026, October 12 selected with focus block', 'date_grid, card, button, toolbar'),
 ('Card & context menu', 'Open actions menu and destination submenu', 'context_menu, card'),
 ('Settings', 'Language, switch and terminal-position preferences', 'field, fieldset, select, switch, toggle, toggle_group'),
 ('Document tabs', 'Selected document with long and closable tabs', 'tabs'),
 ('Data table', 'Package/build columns with numeric sort controls', 'table'),
 ('Button', 'Run action, zero activations (plain GPUI example)', 'plain GPUI button; Jig button in Contacts and Board'),
 ('Radio group', 'Read selected, Write and Source options', 'radio_group'),
 ('Collapsible', 'Expanded page references', 'collapsible'),
 ('Accordion', 'Components section expanded', 'accordion'),
 ('Checkbox', 'Mixed parent and checked/unchecked children', 'checkbox, checkbox_group'),
 ('Progress', '2 of 4 files processed', 'progress'),
 ('Scroll area', 'Bounded contributor list with avatar fallbacks', 'scroll_area, avatar'),
 ('Meter', '4 GB of 10 GB used', 'meter'),
 ('Toast', 'Draft saved, revision 1, dismiss control visible', 'toast'),
 ('Workspace navigation', 'View menu open, sections and divider visible', 'menu, menubar, navigation_menu, separator, toolbar'),
 ('Project form', 'Desktop workspace and four-digit demo code 1234', 'form, input, otp_field'),
 ('Zoom controls', '100 percent with stepper and slider', 'number_field, slider'),
 ('Dialog', 'Export settings open with PDF/Markdown/CSV options', 'dialog, toggle_group'),
 ('Alert dialog', 'Confirmation open with Remove draft and Cancel', 'alert_dialog'),
 ('Drawer', 'Inspector open with document metadata', 'drawer'),
 ('Popover', 'Details popup open', 'popover'),
 ('Preview card', 'Document preview open with research summary', 'preview_card'),
 ('Tooltip', 'Sync hint visible alongside trigger', 'tooltip'),
 ('Combobox', 'Workspace suggestions visible', 'combobox'),
 ('Autocomplete', 'Project-tag suggestions visible', 'autocomplete'),
]
revisions = [(root / platform / 'commit.txt').read_text().strip() for platform in ('windows', 'macos')]
assert revisions[0] == revisions[1], f'Mixed capture revisions: {revisions}'
revision = revisions[0]
assert len(revision) == 40
manifest = {'source_commit': revision, 'client_logical_px': [960,640],
            'comparison_png_px': [960,640], 'scale': 1,
            'profile': 'Editor/light/Standard/expanded examples/sidebar top/Preview',
            'normalization': 'Native frame removal only. No resizing, padding or app-content cropping.',
            'captures': []}
destination.mkdir(parents=True, exist_ok=True)
rows = []
for number, (title, state, components) in enumerate(examples, 1):
    variants = [f'{number:02}'] + (['02-lower'] if number == 2 else [])
    for key in variants:
        entry = {'id': key, 'story': title, 'example': state, 'components': components}
        cells = []
        for platform in ('windows', 'macos'):
            original = root / platform / 'native' / f'{key}.png'
            with Image.open(original) as source:
                source.load()
                native_size = source.size
                if platform == 'windows':
                    assert native_size == (962,672), f'{original}: unexpected native size {native_size}'
                    crop = (1,31,961,671)
                    client = source.crop(crop)
                else:
                    assert native_size == (960,640), f'{original}: unexpected native size {native_size}'
                    crop = (0,0,960,640)
                    client = source.copy()
                assert client.size == (960,640)
                assert sum(ImageStat.Stat(client.convert('RGB')).stddev) > 3, f'Blank: {original}'
                out = destination / platform / f'{key}.png'
                out.parent.mkdir(parents=True, exist_ok=True)
                client.save(out)
            native_out = destination / platform / 'native' / original.name
            native_out.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(original, native_out)
            entry[platform] = {'native_size_px': list(native_size), 'client_crop_ltrb': list(crop),
                'png_size_px': [960,640], 'file': f'{platform}/{key}.png',
                'native_sha256': hashlib.sha256(original.read_bytes()).hexdigest(),
                'sha256': hashlib.sha256(out.read_bytes()).hexdigest()}
            relative = f'standard-960x640/{platform}/{key}.png'
            cells.append(f'<a href="{relative}"><img src="{relative}" width="480" height="320" alt="{platform}: {html.escape(title)} {key}"></a>')
        manifest['captures'].append(entry)
        label = f'{number:02} {title}' + (' — lower viewport' if key.endswith('lower') else '')
        rows.append(f'| **{label}**<br>{state}<br><code>{components}</code> | {cells[0]} | {cells[1]} |')

assert len(manifest['captures']) == 31
(destination / 'manifest.json').write_text(json.dumps(manifest, indent=2), encoding='utf-8')
readme = f'''# Windows and macOS — same-size native comparison

**Standard: 960 × 640 logical-pixel client viewport at 1×; every comparison PNG is exactly 960 × 640 pixels.** Images below display at 480 × 320; click for original pixels. No app content is resized or padded. Native originals are preserved separately.

Source revision: `{revision}` on both platforms. Editor · light · Standard corners · Preview · sidebar at top. The capture profile deliberately expands hidden components and seeds example data; it is not the ordinary launch state. See [AGENTS.md](../../AGENTS.md#native-screenshot-comparison-standard) for the capture standard and [size specifications](../visual-specifications.md) for logical dimensions.

Windows native frames measure 962 × 672; extracting `(1, 31, 961, 671)` preserves the entire 960 × 640 client area. macOS captures already measure 960 × 640, including its transparent title bar. Native fonts, traffic lights and toolbar insets intentionally differ. Windows may include a cursor halo from the host. These are appearance examples, not a claim of identical rasterization, interaction correctness, accessibility or frame-rate performance.

The worksheet has a separately labeled lower viewport so its charts are visible without stretching the first view. [Manifest and hashes](standard-960x640/manifest.json) record every original size and extraction rectangle. [Earlier unmatched captures](comparison.html) are historical evidence and do not meet this standard.

| Component example / pictured state | Windows — 960 × 640 | macOS — 960 × 640 |
| --- | --- | --- |
''' + '\n'.join(rows) + '\n'
(destination.parent / 'README.md').write_text(readme, encoding='utf-8')
visual_modules = '''accordion alert_dialog autocomplete avatar button card checkbox checkbox_group collapsible combobox context_menu date_grid dialog drawer field fieldset form input menu menubar meter navigation_menu number_field otp_field popover preview_card progress radio_group scroll_area select separator slider switch table tabs toast toggle toggle_group toolbar tooltip'''.split()
index = '\n## Component coverage index\n\nEvery public visual component has an example above; composite stories cover several components. `corners` and `state` are nonvisual configuration/state helpers used by these examples.\n\n| Component | Example stories |\n| --- | --- |\n'
for module in visual_modules:
    matches = [f'{n:02} {title}' for n, (title, _, components) in enumerate(examples, 1)
               if module in re.findall(r'\b[a-z_]+\b', components)]
    assert matches, f'No example documented for {module}'
    index += f'| `{module}` | {", ".join(matches)} |\n'
with (destination.parent / 'README.md').open('a', encoding='utf-8') as stream:
    stream.write(index)
# Scaled sheets are review aids, never canonical evidence.
font = ImageFont.load_default(size=16)
for start in range(0,31,6):
    sheet = Image.new('RGB',(1000,6*370),'#eeeeef')
    draw = ImageDraw.Draw(sheet)
    for offset, entry in enumerate(manifest['captures'][start:start+6]):
        y = offset*370
        draw.text((10,y+5),f'{entry["id"]} {entry["story"]} — Windows / macOS',font=font,fill='black')
        for col, platform in enumerate(('windows','macos')):
            with Image.open(destination / entry[platform]['file']) as im:
                im.thumbnail((480,320))
                sheet.paste(im,(10+col*500,y+35))
    sheet.save(destination / f'review-{start//6+1}.png')
print(f'Validated 62 native captures and 62 exact 960x640 comparison PNGs at {revision}.')
