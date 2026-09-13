# Native Bookrack on iOS

The native bundle contains this fork's web UI as a static export. Building it
with `NEXT_PUBLIC_SELF_HOSTED=true` enables upstream's self-hosted access gate,
including explicit TTS audio downloads, without changing reading or TTS engines.
The App Store build does not receive this flag from your server.

## Toolchain and export

Use the pinned pnpm version and complete the source baseline setup first. On
macOS, install full Xcode, accept its license, select its developer directory,
and install the iOS SDK/simulator runtime. Install Rust through rustup with
`aarch64-apple-ios` and `aarch64-apple-ios-sim` targets; a Homebrew host-only Rust
installation cannot cross-compile iOS. CocoaPods is needed by Tauri's iOS plugins.
Follow [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for setup.

From `apps/readest-app`:

```bash
BOOKRACK_LOCAL_ONLY=1 bash scripts/bookrack-ios.sh frontend
BOOKRACK_LOCAL_ONLY=1 bash scripts/bookrack-ios.sh simulator
```

The wrapper forces both self-hosted variables and the Tauri platform, then runs
Next's static export into `out/`. It overrides the upstream source-map upload
hook and invokes only its map-stripping helper, without uploading. The simulator
command targets Apple silicon and passes `--no-sign`; nulling the Tauri team
setting alone does not remove signing settings from the customized Xcode targets.
No device installation is performed.
This wrapper targets Apple silicon; the preserved Xcode project hardcodes arm64.
Never run a web and Tauri build simultaneously in one
checkout: both use `.next/` and can corrupt each other's build output.

`BOOKRACK_LOCAL_ONLY=1` explicitly builds a local reader. It overrides cloud
Supabase/API defaults with an unavailable loopback endpoint and a noncredential
placeholder key, so missing credentials never connect this build to upstream's
hosted account service. Cloud login/sync controls cannot function in this mode.
Without local-only mode, the script refuses incomplete sync configuration before
starting a build.

The build flag enables access; it does not configure sync. For sync, supply your
own browser-safe `NEXT_PUBLIC_SUPABASE_URL`, `NEXT_PUBLIC_SUPABASE_ANON_KEY` and
`NEXT_PUBLIC_API_BASE_URL` at build time and rebuild. Never embed a service-role
key, database password or S3 secret in the app. Server URL settings do not exist
in the upstream native settings UI. Use local-only mode when you do not need sync. Rebuild with your own settings
to enable sync later.

## Signing is more than one environment variable

This upstream revision tracks a customized Xcode project under
`src-tauri/gen/apple`, including ShareExtension and ReadestWidget targets.
**Do not blindly run `tauri ios init` over it:** that can overwrite the customized
project. Preserve its extension, Rust build and library-linking configuration.
A fresh clone lacks ignored `Sources/`, `Assets.xcassets`, `Externals/`, and
`LaunchScreen.storyboard` files. Before initialization, save the tracked
`gen/apple` files outside the checkout; run `pnpm tauri ios init --ci` with your
local config, then restore those saved customized files. Regenerate the Xcode
project from the preserved `project.yml` using `env -u FORCE_COLOR xcodegen
generate` from `gen/apple` (install XcodeGen first). Inspect the diff: Tauri
initialization must not silently remove widget/share targets or entitlements.
The preserved project references an ignored main `Readest_iOS/Info.plist`
as an opaque input, so initialization may not create it. The Bookrack assets
helper seeds it only when missing, using standard Tauri/Xcode application keys,
background audio, and the tracked `Info.plist` plus configured `Info-ios.plist`
metadata. Existing operator plists are left untouched by the helper. Tauri then
performs its normal source-plist merge during each build. Never copy a
developer's private provisioning files to fill missing generated files.
The simulator may retain upstream target names; those are not shipping branding.

Copy `docs/bookrack/ios-signing.example.json` to an ignored `*.local.json` file
and fill in your own registered bundle identifier and Apple Developer team ID.
Set `BOOKRACK_IOS_CONFIG` to its absolute path. Do not reuse upstream's team
`J5W48D69VR` or bundle ID `com.bilingify.readest`.

Before the first device build, reconcile these together in Xcode and source:

- Main app, widget and share-extension bundle IDs and development teams in
  `gen/apple/project.yml` and `Readest.xcodeproj/project.pbxproj`.
- Their entitlement files, App Group and iCloud container IDs. The native bridge
  and widget/share Swift code also use the App Group string; update all matching
  references together so extensions can still share data.
- Associated domains, Apple sign-in and OAuth redirects must use registrations
  you control. CarPlay requires Apple's entitlement approval; remove unsupported
  capabilities from your provisioning setup or obtain approval before export.
- `Info-ios.plist` and generated Info.plist metadata, display name, URL schemes,
  document types and background audio must remain consistent with the app.

The device wrapper refuses the untouched upstream Xcode signing identifiers.
This is an explicit operator configuration gate, not an assertion that changing
the JSON alone produces an installable app.

The wrapper runs `scripts/bookrack-ios-assets.sh` before native compilation.
It generates the full iOS icon family from the design lock's
`docs/bookrack/icon.svg`, verifies filenames against the generated Xcode asset
catalog, and installs a static paper-and-moss launch screen from
`docs/bookrack/LaunchScreen.storyboard`. The centered book mark follows
`docs/bookrack/splash.svg`. Generated assets remain in the ignored Xcode output;
the reproducible source assets and installer are committed. Confirm both in
Simulator before calling visual integration done.

```bash
BOOKRACK_LOCAL_ONLY=1 BOOKRACK_IOS_CONFIG=/absolute/path/ios-signing.local.json \
  bash scripts/bookrack-ios.sh device
```

Use Xcode's signing UI for your Apple account and profiles. For sideloading,
connect the device, enable Developer Mode and install the signed build through
Xcode. For TestFlight, create your App Store Connect app record, archive/export
with the matching bundle ID and upload through Xcode Organizer; provisioning,
Apple processing and any beta review remain operator steps. No upload, signing
credential import or install is automated here. See the
[Tauri iOS CLI](https://v2.tauri.app/reference/cli/#ios) for export options.

## Validation and human gates

Record the actual export/native build commands and outcomes in the PR. A passing
web build or configuration check is not evidence of a compiled iOS app. Until a
native build succeeds and design assets are verified in it, bookrack issue #6
remains open.

Human steps: finish Apple identity/capability setup, sign, install on iPhone/iPad,
import one DRM-free book, download its audio, then fully close the app and verify
playback in airplane mode. Check interruption/resume and retention after restart.

Fallback: serve the self-hosted web client via trusted HTTPS and use Safari's
Share → Add to Home Screen. It provides the same self-hosted unlock without Apple
signing; browser storage may be evicted. Follow bookrack's `docs/BRINGUP.md` and
test offline playback on the actual device before relying on it.
