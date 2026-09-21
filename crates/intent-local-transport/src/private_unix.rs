use nix::unistd::geteuid;
use std::io;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};

fn private_parent(path: &Path) -> io::Result<PathBuf> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let metadata = std::fs::symlink_metadata(parent)?;
    if !metadata.file_type().is_dir()
        || metadata.uid() != geteuid().as_raw()
        || metadata.permissions().mode() & 0o077 != 0
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Unix authentication endpoint parent must be an owned private directory",
        ));
    }
    Ok(parent.to_path_buf())
}

pub fn create_private_unix_listener(path: &Path) -> io::Result<UnixListener> {
    let _private_parent = private_parent(path)?;
    let listener = UnixListener::bind(path)?;
    if let Err(error) = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)) {
        let _ = std::fs::remove_file(path);
        return Err(error);
    }
    Ok(listener)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::DirBuilder;
    use std::os::unix::fs::DirBuilderExt;

    struct TestDirectory(PathBuf);

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(self.0.join("peer"));
            let _ = std::fs::remove_dir(&self.0);
        }
    }

    fn directory(mode: u32) -> io::Result<TestDirectory> {
        let mut nonce = [0_u8; 16];
        getrandom::fill(&mut nonce).map_err(io::Error::other)?;
        let path = std::env::temp_dir().join(format!(
            "intent-private-listener-{}-{:032x}",
            std::process::id(),
            u128::from_ne_bytes(nonce)
        ));
        DirBuilder::new().mode(0o700).create(&path)?;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode))?;
        Ok(TestDirectory(path))
    }

    #[test]
    fn listener_requires_private_owned_parent_and_sets_socket_mode() -> io::Result<()> {
        let private = directory(0o700)?;
        let listener = create_private_unix_listener(&private.0.join("peer"))?;
        assert_eq!(
            std::fs::symlink_metadata(private.0.join("peer"))?
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
        drop(listener);

        let public = directory(0o755)?;
        let result = create_private_unix_listener(&public.0.join("peer"));
        assert!(matches!(
            result,
            Err(ref error) if error.kind() == io::ErrorKind::PermissionDenied
        ));
        assert!(!public.0.join("peer").exists());
        Ok(())
    }
}
