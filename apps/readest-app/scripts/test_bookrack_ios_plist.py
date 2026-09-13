import importlib.util
from pathlib import Path
import plistlib
import shutil
import tempfile
import unittest

spec = importlib.util.spec_from_file_location(
    "bookrack_ios_plist", Path(__file__).with_name("bookrack-ios-plist.py")
)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class SeedPlistTests(unittest.TestCase):
    def test_missing_seed_preserves_metadata_and_existing_operator_file(self):
        source = Path(__file__).resolve().parent.parent / "src-tauri"
        with tempfile.TemporaryDirectory() as temporary:
            app = Path(temporary)
            tauri = app / "src-tauri"
            destination = tauri / "gen/apple/Readest_iOS/Info.plist"
            destination.parent.mkdir(parents=True)
            for name in ("tauri.conf.json", "Info.plist", "Info-ios.plist"):
                shutil.copyfile(source / name, tauri / name)
            shutil.copyfile(source.parent / "package.json", app / "package.json")
            self.assertTrue(module.seed(tauri))
            metadata = plistlib.loads(destination.read_bytes())
            common = plistlib.loads((tauri / "Info.plist").read_bytes())
            ios = plistlib.loads((tauri / "Info-ios.plist").read_bytes())
            for key, value in {**common, **ios}.items():
                self.assertEqual(metadata[key], value, key)
            self.assertEqual(metadata["CFBundleIdentifier"], "$(PRODUCT_BUNDLE_IDENTIFIER)")
            self.assertEqual(metadata["UILaunchStoryboardName"], "LaunchScreen")
            self.assertIn("audio", metadata["UIBackgroundModes"])
            self.assertTrue(metadata["LSSupportsOpeningDocumentsInPlace"])
            metadata["CFBundleDisplayName"] = "Operator reader"
            original = plistlib.dumps(metadata, fmt=plistlib.FMT_BINARY)
            destination.write_bytes(original)
            self.assertFalse(module.seed(tauri))
            self.assertEqual(destination.read_bytes(), original)
            destination.unlink()
            destination.symlink_to(tauri / "operator.plist")
            self.assertFalse(module.seed(tauri))
            self.assertFalse((tauri / "operator.plist").exists())


if __name__ == "__main__":
    unittest.main()
