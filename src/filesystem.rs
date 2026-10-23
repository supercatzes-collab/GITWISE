use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
/// Returns the environment directory containing the GITWISE project.
///
/// Example:
/// Gitiwise_env/GITWISE -> Gitiwise_env
pub fn environment_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("GITWISE project directory has no parent")
        .to_path_buf()
}

pub fn library_root() -> PathBuf {
    environment_root().join("LIBRARY")
}

pub fn user_workspace_root(user_id: &str) -> io::Result<PathBuf> {
    let folder_name = match user_id {
        "test-001" => "USER1",
        "test-002" => "USER2",
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No workspace configured for this account",
            ));
        }
    };

    Ok(environment_root().join(folder_name))
}

pub fn user_item_folder_path(
    user_id: &str,
    item_id: &str,
) -> io::Result<PathBuf> {
    validate_folder_name(item_id)?;

    Ok(user_workspace_root(user_id)?.join(item_id))
}

/// Returns the permanent library path for an item.
///
/// Example: library_folder/item-001
pub fn item_folder_path(item_id: &str) -> io::Result<PathBuf> {
    validate_folder_name(item_id)?;

    Ok(library_root().join(item_id))
}

/// Creates the library directory and the specified user's workspace
/// if they do not already exist.
pub fn ensure_storage_directories(user_id: &str) -> io::Result<()> {
    fs::create_dir_all(library_root())?;
    fs::create_dir_all(user_workspace_root(user_id)?)?;

    Ok(())
}

/// Prevents account IDs or item IDs from being used as paths
/// containing directory traversal or other path characters.
fn validate_folder_name(name: &str) -> io::Result<()> {
    if name.is_empty()
        || !name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid account or item ID for a folder name",
        ));
    }

    Ok(())
}

/// Validates that a path points to a real directory and returns
/// its canonical (absolute) path.
pub fn validate_folder(path: &Path) -> io::Result<PathBuf> {
    let metadata = fs::symlink_metadata(path)?;

    if metadata.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Symbolic links cannot be used as folder sources",
        ));
    }

    if !metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The selected path is not a directory",
        ));
    }

    fs::canonicalize(path)
}

/// Copies an entire directory recursively.
///
/// The source remains unchanged. The destination must not already
/// exist, and its parent directory must already exist.
pub fn copy_folder_recursive(
    source: &Path,
    destination: &Path,
) -> io::Result<()> {
    let source = validate_folder(source)?;

    let destination_name = destination.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "The destination must name a new directory",
        )
    })?;

    let destination_parent = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    let destination_parent = fs::canonicalize(destination_parent)?;

    if !fs::metadata(&destination_parent)?.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The destination parent is not a directory",
        ));
    }

    let destination = destination_parent.join(destination_name);

    // Prevent copying a folder into itself or one of its descendants.
    if destination.starts_with(&source) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The destination cannot be inside the source folder",
        ));
    }

    // Never silently overwrite an existing destination.
    match fs::symlink_metadata(&destination) {
        Ok(_) => {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "The destination folder already exists",
            ));
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }

    // Create the destination only after validation succeeds.
    fs::create_dir(&destination)?;

    // Remove any incomplete destination if copying fails.
    if let Err(copy_error) =
        copy_directory_contents(&source, &destination)
    {
        match fs::remove_dir_all(&destination) {
            Ok(()) => return Err(copy_error),
            Err(cleanup_error) => {
                return Err(io::Error::new(
                    copy_error.kind(),
                    format!(
                        "Copy failed: {copy_error}. \
                         Cleanup also failed: {cleanup_error}"
                    ),
                ));
            }
        }
    }

    Ok(())
}

/// Copies the contents of one directory into an existing directory.
fn copy_directory_contents(
    source: &Path,
    destination: &Path,
) -> io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        let source_path = entry.path();
        let destination_path =
            destination.join(entry.file_name());

        if file_type.is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Symbolic links are not supported: {}",
                    source_path.display()
                ),
            ));
        } else if file_type.is_dir() {
            fs::create_dir(&destination_path)?;

            copy_directory_contents(
                &source_path,
                &destination_path,
            )?;
        } else if file_type.is_file() {
            fs::copy(
                &source_path,
                &destination_path,
            )?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Unsupported filesystem entry: {}",
                    source_path.display()
                ),
            ));
        }
    }

    Ok(())
}

static NEXT_REPLACEMENT_ID: AtomicU64 = AtomicU64::new(0);

/// Generates a unique temporary directory path beside an existing folder.
fn unique_sibling_path(
    folder: &Path,
    purpose: &str,
) -> io::Result<PathBuf> {
    let parent = folder.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Folder has no parent directory",
        )
    })?;

    let name = folder.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Folder has no name",
        )
    })?;

    for _ in 0..1000 {
        let id = NEXT_REPLACEMENT_ID.fetch_add(
            1,
            Ordering::Relaxed,
        );

        let candidate = parent.join(format!(
            ".{}.{}-{}-{}",
            name.to_string_lossy(),
            purpose,
            std::process::id(),
            id,
        ));

        match fs::symlink_metadata(&candidate) {
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Ok(candidate);
            }
            Ok(_) => continue,
            Err(error) => return Err(error),
        }
    }

    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "Could not allocate a unique temporary folder name",
    ))
}

/// Replaces a published item folder with a replacement folder.
///
/// The replacement is first copied into a staging directory.
/// The original is then backed up before the staged copy is installed.
/// The returned handle retains that backup until `finalize()` is called.
pub fn replace_item_folder(
    published_folder: &Path,
    replacement_folder: &Path,
) -> io::Result<FolderReplacement> {
    // Both folders must already exist and be valid directories.
    let destination = validate_folder(published_folder)?;
    let source = validate_folder(replacement_folder)?;

    // Prevent replacing a folder with itself or using nested folders
    // in a way that could destroy the source during replacement.
    if source == destination
        || source.starts_with(&destination)
        || destination.starts_with(&source)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source and published folders must be separate directories",
        ));
    }

    let staging = unique_sibling_path(&destination, "staging")?;
    let backup = unique_sibling_path(&destination, "backup")?;

    // Prepare the replacement without touching the original.
    if let Err(copy_error) = copy_folder_recursive(&source, &staging) {
        let cleanup_error = if staging.exists() {
            fs::remove_dir_all(&staging).err()
        } else {
            None
        };

        return Err(match cleanup_error {
            Some(cleanup_error) => io::Error::new(
                copy_error.kind(),
                format!(
                    "Could not stage replacement: {copy_error}. \
                     Staging cleanup also failed: {cleanup_error}"
                ),
            ),
            None => copy_error,
        });
    }

    // Move the original aside before installing the replacement.
    if let Err(error) = fs::rename(&destination, &backup) {
        let cleanup_error = fs::remove_dir_all(&staging).err();

        return Err(match cleanup_error {
            Some(cleanup_error) => io::Error::new(
                error.kind(),
                format!(
                    "Could not back up the original folder: {error}. \
                     Staging cleanup also failed: {cleanup_error}"
                ),
            ),
            None => error,
        });
    }

    // If installation fails, attempt to restore the original immediately.
    if let Err(install_error) = fs::rename(&staging, &destination) {
        let restore_result = fs::rename(&backup, &destination);
        let cleanup_result = fs::remove_dir_all(&staging);

        let message = format!(
            "Could not install the replacement: {install_error}. \
             Restore result: {restore_result:?}. \
             Staging cleanup result: {cleanup_result:?}"
        );

        return Err(io::Error::new(
            install_error.kind(),
            message,
        ));
    }

    Ok(FolderReplacement {
        destination,
        backup,
        active: true,
    })
}

/// Tracks a replacement until the database update is committed.
pub struct FolderReplacement {
    destination: PathBuf,
    backup: PathBuf,
    active: bool,
}

impl FolderReplacement {
    /// Restores the original folder from its backup.
    pub fn rollback(&mut self) -> io::Result<()> {
        if !self.active {
            return Ok(());
        }

        if self.destination.exists() {
            fs::remove_dir_all(&self.destination)?;
        }

        fs::rename(&self.backup, &self.destination)?;
        self.active = false;

        Ok(())
    }

    /// Finalizes a successful replacement by deleting the backup.
    ///
    /// The replacement remains installed even if backup cleanup fails.
    pub fn finalize(mut self) -> io::Result<()> {
        self.active = false;
        fs::remove_dir_all(&self.backup)
    }
}

// If the handle is dropped without finalizing, try to restore the original.
impl Drop for FolderReplacement {
    fn drop(&mut self) {
        if self.active {
            let _ = self.rollback();
        }
    }
}

/// Returns an available destination path without overwriting
/// an existing file or folder.
///
/// Example:
/// item-001
/// item-001 (1)
/// item-001 (2)
pub fn available_destination_path(
    desired: &Path,
) -> io::Result<PathBuf> {
    let parent = desired
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    let name = desired.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Destination must have a folder name",
        )
    })?;

    let name = name.to_string_lossy();

    // Use the original name if it is available.
    match fs::symlink_metadata(desired) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(desired.to_path_buf());
        }
        Ok(_) => {}
        Err(error) => return Err(error),
    }

    // Otherwise, find the first available numbered name.
    for index in 1..=10_000 {
        let candidate =
            parent.join(format!("{name} ({index})"));

        match fs::symlink_metadata(&candidate) {
            Err(error)
                if error.kind() == io::ErrorKind::NotFound =>
            {
                return Ok(candidate);
            }
            Ok(_) => continue,
            Err(error) => return Err(error),
        }
    }

    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "Could not find an available destination folder name",
    ))
}