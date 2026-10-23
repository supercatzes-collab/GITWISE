use crate::filesystem::copy_folder_recursive;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use crate::filesystem::replace_item_folder;

static NEXT_TEST_ID: AtomicU64 = AtomicU64::new(0);

fn create_test_root() -> PathBuf {
    let id = NEXT_TEST_ID.fetch_add(1, Ordering::Relaxed);

    let root = std::env::temp_dir().join(format!(
        "gitwise_fs_test_{}_{}",
        std::process::id(),
        id,
    ));

    fs::create_dir_all(&root).unwrap();
    root
}

#[test]
fn copies_nested_folders_and_files() {
    let root = create_test_root();

    let source = root.join("source");
    let destination_parent = root.join("destination_parent");
    let destination = destination_parent.join("item-001");

    fs::create_dir_all(source.join("nested")).unwrap();
    fs::create_dir_all(&destination_parent).unwrap();

    fs::write(
        source.join("report.docx"),
        b"document contents",
    )
    .unwrap();

    fs::write(
        source.join("nested").join("notes.txt"),
        b"nested contents",
    )
    .unwrap();

    copy_folder_recursive(&source, &destination).unwrap();

    // Check that both the main file and nested file were copied.
    assert_eq!(
        fs::read(destination.join("report.docx")).unwrap(),
        b"document contents",
    );

    assert_eq!(
        fs::read(destination.join("nested").join("notes.txt")).unwrap(),
        b"nested contents",
    );

    // Copying must not modify the source.
    assert_eq!(
        fs::read(source.join("report.docx")).unwrap(),
        b"document contents",
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn refuses_to_overwrite_existing_destination() {
    let root = create_test_root();

    let source = root.join("source");
    let destination_parent = root.join("destination_parent");
    let destination = destination_parent.join("item-001");

    fs::create_dir_all(&source).unwrap();
    fs::create_dir_all(&destination).unwrap();

    fs::write(
        destination.join("keep.txt"),
        b"existing data",
    )
    .unwrap();

    let result = copy_folder_recursive(&source, &destination);

    assert_eq!(
        result.unwrap_err().kind(),
        io::ErrorKind::AlreadyExists,
    );

    // Existing destination data must remain intact.
    assert_eq!(
        fs::read(destination.join("keep.txt")).unwrap(),
        b"existing data",
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn refuses_destination_inside_source() {
    let root = create_test_root();
    let source = root.join("source");

    fs::create_dir_all(&source).unwrap();

    let destination = source.join("nested-copy");

    let result = copy_folder_recursive(&source, &destination);

    assert_eq!(
        result.unwrap_err().kind(),
        io::ErrorKind::InvalidInput,
    );

    assert!(!destination.exists());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn replaces_published_folder_and_preserves_source() {
    let root = create_test_root();

    let published = root.join("LIBRARY").join("item-001");
    let replacement = root.join("USER2").join("item-001");

    fs::create_dir_all(&published).unwrap();
    fs::create_dir_all(&replacement).unwrap();

    fs::write(
        published.join("old.txt"),
        b"original published version",
    )
    .unwrap();

    fs::write(
        replacement.join("report.docx"),
        b"proposed version",
    )
    .unwrap();

    let replacement_handle =
        replace_item_folder(&published, &replacement).unwrap();

    // The proposed version is installed.
    assert_eq!(
        fs::read(published.join("report.docx")).unwrap(),
        b"proposed version",
    );

    // Files omitted by the replacement are removed.
    assert!(!published.join("old.txt").exists());

    // The contributor's original folder remains intact.
    assert_eq!(
        fs::read(replacement.join("report.docx")).unwrap(),
        b"proposed version",
    );

    // Finalize only after the caller's database transaction commits.
    replacement_handle.finalize().unwrap();

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn rollback_restores_the_original_folder() {
    let root = create_test_root();

    let published = root.join("LIBRARY").join("item-001");
    let replacement = root.join("USER2").join("item-001");

    fs::create_dir_all(&published).unwrap();
    fs::create_dir_all(&replacement).unwrap();

    fs::write(
        published.join("old.txt"),
        b"original published version",
    )
    .unwrap();

    fs::write(
        replacement.join("new.txt"),
        b"proposed version",
    )
    .unwrap();

    let mut replacement_handle =
        replace_item_folder(&published, &replacement).unwrap();

    // Simulate a database failure after installing the new folder.
    replacement_handle.rollback().unwrap();

    assert_eq!(
        fs::read(published.join("old.txt")).unwrap(),
        b"original published version",
    );

    assert!(!published.join("new.txt").exists());

    // The contributor's source remains intact.
    assert_eq!(
        fs::read(replacement.join("new.txt")).unwrap(),
        b"proposed version",
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn missing_replacement_preserves_the_original() {
    let root = create_test_root();

    let published = root.join("LIBRARY").join("item-001");
    let missing_replacement = root.join("USER2").join("missing-item");

    fs::create_dir_all(&published).unwrap();

    fs::write(
        published.join("old.txt"),
        b"original published version",
    )
    .unwrap();

    let result =
        replace_item_folder(&published, &missing_replacement);

    assert!(result.is_err());

    assert_eq!(
        fs::read(published.join("old.txt")).unwrap(),
        b"original published version",
    );

    fs::remove_dir_all(root).unwrap();
}