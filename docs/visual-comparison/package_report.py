from pathlib import Path
import json, zipfile

root = Path(__file__).resolve().parent
manifest = json.loads((root / 'manifest.json').read_text())
assert len(manifest) == 30
assert all(row[p] and row[p]['nonblank'] for row in manifest for p in ('windows', 'macos'))
assert all(row['windows']['pixels'] == [1122, 749] for row in manifest)
output = root.parent / 'cratebook-windows-macos-screenshots.zip'
with zipfile.ZipFile(output, 'w', zipfile.ZIP_DEFLATED) as archive:
    for path in root.rglob('*'):
        if path.is_file() and path.suffix in ('.png', '.html', '.json', '.txt', '.py'):
            archive.write(path, 'visual-comparison/' + path.relative_to(root).as_posix())
print(output)
