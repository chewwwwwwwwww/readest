use std::path::{Path, PathBuf};

pub fn prepare_directory(legacy: &Path, durable: &Path) -> Result<PathBuf, String> {
    fn is_directory(path: &Path) -> Result<bool, String> {
        match std::fs::symlink_metadata(path) {
            Ok(metadata) if metadata.is_dir() => Ok(true),
            Ok(_) => Err(format!(
                "TTS storage is not a directory: {}",
                path.display()
            )),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(format!("Cannot inspect TTS storage: {error}")),
        }
    }
    let old_exists = is_directory(legacy)?;
    let new_exists = is_directory(durable)?;
    // Windows resolves AppCache and AppLocalData to the same directory.
    if legacy == durable && new_exists {
        return Ok(durable.to_path_buf());
    }
    if old_exists && new_exists {
        return Err(format!(
            "Both legacy and durable TTS storage exist; preserve and recover both before retrying: {} and {}",
            legacy.display(), durable.display()
        ));
    }
    if !new_exists {
        let parent = durable.parent().ok_or("TTS storage has no parent")?;
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        if old_exists {
            // Same app container filesystem: rename the entire closed database,
            // sidecars and audio together. No partial copy or overwrite fallback.
            std::fs::rename(legacy, durable)
                .map_err(|error| format!("Cannot migrate TTS storage: {error}"))?;
        } else {
            std::fs::create_dir(durable).map_err(|error| error.to_string())?;
        }
    }
    Ok(durable.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            static SEQUENCE: AtomicU64 = AtomicU64::new(0);
            let path = std::env::temp_dir().join(format!(
                "bookrack-tts-storage-{}-{}",
                std::process::id(),
                SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
        fn paths(&self) -> (PathBuf, PathBuf) {
            (
                self.0.join("Caches/tts-cache"),
                self.0.join("Application Support/tts-cache"),
            )
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn shared_cache_and_data_directory_can_reopen_on_windows() {
        let fixture = Fixture::new();
        let (_, shared) = fixture.paths();
        prepare_directory(&shared, &shared).unwrap();
        fs::write(shared.join("index.sqlite"), "pinned audio index").unwrap();
        prepare_directory(&shared, &shared).unwrap();
        assert_eq!(
            fs::read_to_string(shared.join("index.sqlite")).unwrap(),
            "pinned audio index"
        );
    }

    #[test]
    fn creates_durable_root_for_new_install() {
        let fixture = Fixture::new();
        let (legacy, durable) = fixture.paths();
        assert_eq!(prepare_directory(&legacy, &durable).unwrap(), durable);
        assert!(durable.is_dir());
        assert!(!legacy.exists());
    }

    #[test]
    fn migrates_whole_tree_with_sqlite_sidecars_packs_and_pins_and_retries() {
        let fixture = Fixture::new();
        let (legacy, durable) = fixture.paths();
        fs::create_dir_all(legacy.join("packs/book/chapter")).unwrap();
        for (file, bytes) in [
            ("index.sqlite", "database with pins"),
            ("index.sqlite-wal", "pending transaction"),
            ("index.sqlite-shm", "shared memory"),
            ("packs/book/chapter/audio.mp3", "audio"),
        ] {
            fs::write(legacy.join(file), bytes).unwrap();
        }
        prepare_directory(&legacy, &durable).unwrap();
        assert!(!legacy.exists());
        assert_eq!(
            fs::read_to_string(durable.join("index.sqlite-wal")).unwrap(),
            "pending transaction"
        );
        assert_eq!(
            fs::read_to_string(durable.join("packs/book/chapter/audio.mp3")).unwrap(),
            "audio"
        );
        // A crash after rename but before backup exclusion/database opening can retry.
        prepare_directory(&legacy, &durable).unwrap();
        assert_eq!(
            fs::read_to_string(durable.join("index.sqlite")).unwrap(),
            "database with pins"
        );
    }

    #[test]
    fn conflicting_roots_are_preserved_and_rejected() {
        let fixture = Fixture::new();
        let (legacy, durable) = fixture.paths();
        fs::create_dir_all(&legacy).unwrap();
        fs::create_dir_all(&durable).unwrap();
        fs::write(legacy.join("index.sqlite"), "legacy pins").unwrap();
        fs::write(durable.join("index.sqlite"), "durable pins").unwrap();
        assert!(prepare_directory(&legacy, &durable).is_err());
        assert_eq!(
            fs::read_to_string(legacy.join("index.sqlite")).unwrap(),
            "legacy pins"
        );
        assert_eq!(
            fs::read_to_string(durable.join("index.sqlite")).unwrap(),
            "durable pins"
        );
    }

    #[cfg(unix)]
    #[test]
    fn refuses_symlink_roots_without_touching_target() {
        let fixture = Fixture::new();
        let (legacy, durable) = fixture.paths();
        fs::create_dir_all(legacy.parent().unwrap()).unwrap();
        std::os::unix::fs::symlink(fixture.0.join("missing"), &legacy).unwrap();
        assert!(prepare_directory(&legacy, &durable).is_err());
        assert!(fs::symlink_metadata(&legacy)
            .unwrap()
            .file_type()
            .is_symlink());
        assert!(!durable.exists());
    }
}
