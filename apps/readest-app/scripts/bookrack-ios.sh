#!/usr/bin/env bash
# Run from any directory. Credentials/signing remain operator-owned.
set -euo pipefail
APP_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$APP_DIR"
mode="${1:-frontend}"
case "$mode" in frontend|simulator|device) ;; *) echo 'Usage: scripts/bookrack-ios.sh [frontend|simulator|device]' >&2; exit 2;; esac
export NEXT_PUBLIC_APP_PLATFORM=tauri
export NEXT_PUBLIC_SELF_HOSTED=true
export SELF_HOSTED=true
if [[ "${BOOKRACK_LOCAL_ONLY:-}" == 1 ]]; then
  # Explicit local-reader build: route cloud calls to an unavailable loopback
  # port instead of inheriting upstream hosted-service defaults.
  export NEXT_PUBLIC_SUPABASE_URL=http://127.0.0.1:9
  export NEXT_PUBLIC_SUPABASE_ANON_KEY=bookrack-local-only
  export NEXT_PUBLIC_API_BASE_URL=http://127.0.0.1:9
  export NEXT_PUBLIC_NODE_BASE_URL=http://127.0.0.1:9
else
  : "${NEXT_PUBLIC_SUPABASE_URL:?Set your own sync endpoint or BOOKRACK_LOCAL_ONLY=1}"
  : "${NEXT_PUBLIC_SUPABASE_ANON_KEY:?Set your own public anon key or BOOKRACK_LOCAL_ONLY=1}"
  : "${NEXT_PUBLIC_API_BASE_URL:?Set your own API endpoint or BOOKRACK_LOCAL_ONLY=1}"
  export NEXT_PUBLIC_NODE_BASE_URL="${NEXT_PUBLIC_NODE_BASE_URL:-$NEXT_PUBLIC_API_BASE_URL}"
fi
export SUPABASE_URL="$NEXT_PUBLIC_SUPABASE_URL"
export SUPABASE_PUBLIC_URL="$NEXT_PUBLIC_SUPABASE_URL"
export SUPABASE_ANON_KEY="$NEXT_PUBLIC_SUPABASE_ANON_KEY"
export API_BASE_URL="$NEXT_PUBLIC_API_BASE_URL"
# Exported values win over dotenv defaults; no source-map upload hook is run.
if [[ "$mode" == frontend ]]; then
  pnpm exec next build
  node --input-type=module -e "import { stripMaps } from './scripts/upload-sourcemaps.mjs'; console.log('Bookrack: removed', stripMaps('out/_next/static'), 'source maps without uploading.');"
  exit 0
fi
if [[ "$(uname -s)" != Darwin ]]; then echo 'iOS requires macOS and Xcode.' >&2; exit 1; fi
config='{"build":{"beforeBuildCommand":"bash scripts/bookrack-ios.sh frontend"},"bundle":{"iOS":{"developmentTeam":null}}}'
if [[ "$mode" == simulator ]]; then
  bash scripts/bookrack-ios-assets.sh
  exec pnpm tauri ios build --ci --no-sign --target aarch64-sim --config "$config" -- --locked
fi
: "${BOOKRACK_IOS_CONFIG:?Set BOOKRACK_IOS_CONFIG to your local signing config; see docs/bookrack/ios.md}"
python3 - "$BOOKRACK_IOS_CONFIG" <<'PY'
import json, pathlib, sys
config = json.loads(pathlib.Path(sys.argv[1]).read_text())
team = config.get('bundle', {}).get('iOS', {}).get('developmentTeam')
identifier = config.get('identifier', '')
if not team or team == 'J5W48D69VR' or not identifier or identifier == 'com.bilingify.readest':
    sys.exit('Set your own development team and bundle identifier before device builds.')
if config.get('build'):
    sys.exit('Signing config must not override the Bookrack frontend build hook.')
# The customized Xcode project has independent signing settings and extensions.
project = pathlib.Path('src-tauri/gen/apple/Readest.xcodeproj/project.pbxproj').read_text()
if 'J5W48D69VR' in project or 'com.bilingify.readest' in project:
    sys.exit('Reconcile all Xcode targets, entitlements and App Groups first; see ios.md.')
PY
bash scripts/bookrack-ios-assets.sh
exec pnpm tauri ios build --ci --target aarch64 --config "$config" --config "$BOOKRACK_IOS_CONFIG" -- --locked
