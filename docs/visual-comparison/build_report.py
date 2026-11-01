from pathlib import Path
from PIL import Image, ImageDraw, ImageFont, ImageStat
import json, html, zipfile

root = Path(__file__).resolve().parent
spec = json.loads((root / 'specs.json').read_text())
manifest = []
for i, title in enumerate(spec['stories'], 1):
    row = {'number': i, 'title': title}
    for platform in ('windows', 'macos'):
        path = root / platform / f'{i:02}.png'
        if path.exists():
            with Image.open(path) as im:
                row[platform] = {'file': path.relative_to(root).as_posix(), 'pixels': list(im.size),
                                 'nonblank': sum(ImageStat.Stat(im.convert('RGB')).stddev) > 3}
        else:
            row[platform] = None
    manifest.append(row)
(root / 'manifest.json').write_text(json.dumps(manifest, indent=2))

specrows = ''.join(f'<tr><th>{html.escape(k.replace("_", " "))}</th><td>{html.escape(json.dumps(v, ensure_ascii=False))}</td></tr>'
                   for k, v in spec.items() if k != 'stories')
sections = []
for row in manifest:
    figures = []
    for platform, label in [('windows', 'Windows'), ('macos', 'macOS')]:
        item = row[platform]
        if item:
            w, h = item['pixels']
            body = f'<a href="{item["file"]}" target="_blank"><img loading="lazy" src="{item["file"]}" alt="{label}: {html.escape(row["title"])}"></a>'
            caption = f'{label} · actual PNG {w} × {h} pixels'
            if not item['nonblank']: caption += ' · REJECTED: blank capture'
        else:
            body = '<p class="pending">Capture pending. No substitute image.</p>'
            caption = label + ' · capture unavailable'
        figures.append(f'<figure>{body}<figcaption>{caption}</figcaption></figure>')
    sections.append(f'<section id="story-{row["number"]}" data-story="{row["number"]}"><h2>{row["number"]:02} · {html.escape(row["title"])}</h2><div class="pair">{"".join(figures)}</div></section>')
options = ''.join(f'<option value="{r["number"]}">{r["number"]:02} {html.escape(r["title"])}</option>' for r in manifest)
counts = {p: sum(bool(r[p] and r[p]['nonblank']) for r in manifest) for p in ('windows', 'macos')}
report = f'''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width">
<title>Cratebook · Windows / macOS visual specifications</title>
<style>body{{font:15px/1.5 system-ui,sans-serif;margin:0;background:#f4f4f5;color:#242429}}header,main{{max-width:1500px;margin:auto;padding:24px}}h1{{font-size:28px}}h2{{font-size:19px}}.pair{{display:grid;grid-template-columns:1fr 1fr;gap:16px;align-items:start}}figure{{margin:0;background:white;padding:8px;border:1px solid #d1d1d6;border-radius:8px;min-width:0}}img{{width:100%;height:auto;display:block}}figcaption{{padding:8px 2px;font-size:13px;color:#50505a}}section{{margin:32px 0}}table{{border-collapse:collapse;width:100%;background:white}}th,td{{padding:9px 12px;border:1px solid #ddd;text-align:left;vertical-align:top}}th{{width:38%}}td{{overflow-wrap:anywhere}}.controls{{position:sticky;top:0;background:#f4f4f5;padding:12px 0;border-bottom:1px solid #ccc;z-index:2}}select,button{{font:inherit;padding:7px}}.pending{{padding:40px 16px;color:#7a3f16}}.native .pair{{display:flex;overflow:auto}}.native figure{{flex:none}}.native img{{width:auto;max-width:none}}@media(max-width:750px){{.pair{{grid-template-columns:1fr}}}}[hidden]{{display:none!important}}</style>
<header><h1>Cratebook: Windows and macOS</h1><p>30 initial story states · Editor / light · Standard corners · requested 1120 × 720 logical bounds</p>
<p>Verified nonblank files: Windows {counts['windows']}/30 · macOS {counts['macos']}/30. Build commit: <code>{spec['commit']}</code>.</p>
<p>Images below fit their columns for comparison. Captions show actual PNG dimensions. Use original-pixel mode or open an image to inspect unscaled pixels. A logical pixel is not necessarily one PNG pixel; native frame height, display scale and fonts differ. These are first-viewport captures, not screenshots of every interaction state.</p>
<details><summary>Size and typography specifications (from source)</summary><p>{html.escape(spec['measurement_note'])}</p><table>{specrows}</table></details>
<p id="findings"><a href="findings.txt">Read the capture findings and limitations</a>. All 30 initial views were captured on each platform. Windows originals are 1122 × 749 pixels; macOS originals are 1024 × 674–677 pixels because the runner constrained its windows. This is a visual comparison at the recorded sizes, not a pixel-matched regression baseline.</p></header>
<main><div class="controls"><label>Story <select id="filter"><option value="all">All 30 stories</option>{options}</select></label> <button id="pixels">Original-pixel mode</button></div>{''.join(sections)}</main>
<script>document.querySelector('#filter').onchange=e=>{{document.querySelectorAll('section').forEach(s=>s.hidden=e.target.value!=='all'&&s.dataset.story!==e.target.value)}};document.querySelector('#pixels').onclick=()=>document.body.classList.toggle('native');</script></html>'''
(root / 'comparison.html').write_text(report, encoding='utf-8')

# Review sheets preserve the native images, resized only for this overview.
font = ImageFont.load_default(size=16)
for start in range(0, 30, 6):
    sheet = Image.new('RGB', (1020, 6 * 400), '#eeeeef')
    draw = ImageDraw.Draw(sheet)
    for offset, row in enumerate(manifest[start:start+6]):
        y = offset * 400
        draw.text((12, y + 5), f'{row["number"]:02} {row["title"]}', fill='black', font=font)
        for column, platform in enumerate(('windows', 'macos')):
            draw.text((12 + column * 510, y + 30), platform, fill='black', font=font)
            if row[platform]:
                with Image.open(root / row[platform]['file']) as im:
                    im = im.convert('RGB'); im.thumbnail((495, 340))
                    sheet.paste(im, (10 + column * 510, y + 55))
    sheet.save(root / f'review-{start//6+1}.png')
print(json.dumps({'capture_counts': counts, 'manifest': str(root / 'manifest.json')}))
