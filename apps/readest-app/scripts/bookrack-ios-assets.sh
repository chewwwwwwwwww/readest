#!/usr/bin/env bash
set -euo pipefail
APP_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$APP_DIR"
assets=src-tauri/gen/apple/Assets.xcassets
[[ -f "$assets/AppIcon.appiconset/Contents.json" ]] || { echo 'Initialize the iOS project first; see docs/bookrack/ios.md.' >&2; exit 1; }
python3 scripts/bookrack-ios-plist.py
work="$(mktemp -d "${TMPDIR:-/tmp}/bookrack-icons.XXXXXX")"
trap 'rm -rf "$work"' EXIT
pnpm exec tauri icon ../../docs/bookrack/icon.svg --output "$work" --ios-color '#f6f5f0'
python3 - "$work" "$assets" <<'PY'
import json, pathlib, shutil, sys
source, assets = map(pathlib.Path, sys.argv[1:])
icons = assets / 'AppIcon.appiconset'
manifest = json.loads((icons / 'Contents.json').read_text())
for entry in manifest['images']:
    name = entry.get('filename')
    if name:
        if pathlib.Path(name).name != name:
            sys.exit('Unexpected icon filename in generated asset catalog')
        if not (source / 'ios' / name).is_file():
            sys.exit(f'Tauri did not generate required icon: {name}')
for entry in manifest['images']:
    if name := entry.get('filename'):
        shutil.copyfile(source / 'ios' / name, icons / name)
launch = assets / 'BookrackLaunch.imageset'
launch.mkdir(exist_ok=True)
shutil.copyfile(source / '128x128@2x.png', launch / 'bookrack.png')
(launch / 'Contents.json').write_text(json.dumps({'images': [{'idiom': 'universal', 'filename': 'bookrack.png'}], 'info': {'version': 1, 'author': 'xcode'}}, indent=2)+'\n')
PY
cp ../../docs/bookrack/LaunchScreen.storyboard src-tauri/gen/apple/LaunchScreen.storyboard
printf '%s\n' 'Bookrack iOS icon and launch assets installed into the generated Xcode project.'
