#!/usr/bin/env python3
"""Seed the ignored main plist required by Readest's preserved Xcode project."""

import json
from pathlib import Path
import plistlib


def seed(tauri_dir: Path) -> bool:
    destination = tauri_dir / "gen/apple/Readest_iOS/Info.plist"
    # Never replace operator metadata (including an existing symlink).
    if destination.exists() or destination.is_symlink():
        return False
    config = json.loads((tauri_dir / "tauri.conf.json").read_text())
    version = config["version"]
    version_file = tauri_dir / version
    if version_file.is_file():
        version = json.loads(version_file.read_text())["version"]
    version = version.split("-", 1)[0].replace("+", ".")
    ios = config.get("bundle", {}).get("iOS", {})
    # Standard XcodeGen application keys and Tauri's mobile/ios/project.yml
    # defaults. Tauri refreshes the short version when it builds the project.
    metadata = {
        "CFBundleDevelopmentRegion": "en",
        "CFBundleExecutable": "$(EXECUTABLE_NAME)",
        "CFBundleIdentifier": "$(PRODUCT_BUNDLE_IDENTIFIER)",
        "CFBundleInfoDictionaryVersion": "6.0",
        "CFBundleName": "$(PRODUCT_NAME)",
        "CFBundlePackageType": "APPL",
        "CFBundleShortVersionString": ".".join((version.split(".") + ["0", "0"])[:3]),
        "CFBundleVersion": ios.get("bundleVersion", version),
        "LSRequiresIPhoneOS": True,
        # NativeBridge configures AVAudioSession playback; retain it off-screen.
        "UIBackgroundModes": ["audio"],
        "UILaunchStoryboardName": "LaunchScreen",
        "UIRequiredDeviceCapabilities": ["arm64", "metal"],
        "UISupportedInterfaceOrientations": [
            "UIInterfaceOrientationPortrait",
            "UIInterfaceOrientationLandscapeLeft",
            "UIInterfaceOrientationLandscapeRight",
        ],
        "UISupportedInterfaceOrientations~ipad": [
            "UIInterfaceOrientationPortrait",
            "UIInterfaceOrientationPortraitUpsideDown",
            "UIInterfaceOrientationLandscapeLeft",
            "UIInterfaceOrientationLandscapeRight",
        ],
    }
    # Match Tauri's top-level, later-source-wins merge. Keep document types,
    # localization, URL schemes and custom scene configuration intact.
    sources = [tauri_dir / "Info.plist", tauri_dir / "Info.ios.plist"]
    if ios.get("infoPlist"):
        sources.append(tauri_dir / ios["infoPlist"])
    for source in sources:
        if source.is_file():
            with source.open("rb") as handle:
                metadata.update(plistlib.load(handle))
    payload = plistlib.dumps(metadata)
    # Parent is created by tauri ios init. Exclusive creation also prevents
    # replacing a plist created between the existence check and this write.
    try:
        with destination.open("xb") as handle:
            handle.write(payload)
    except FileExistsError:
        return False
    return True


if __name__ == "__main__":
    tauri_dir = Path(__file__).resolve().parent.parent / "src-tauri"
    if seed(tauri_dir):
        print("Seeded missing main iOS Info.plist from tracked metadata.")
