
use super::*;
use rusqlite::Connection;

#[test]
fn insert_item_test() {
    // Create an isolated database that does not touch gitwise.db.
    let conn = Connection::open_in_memory().unwrap();

    conn.execute_batch(
        "
        CREATE TABLE accounts (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            display_name TEXT NOT NULL,
            profile_img_path TEXT NOT NULL,
            credentials TEXT NOT NULL
        );

        CREATE TABLE items (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            abstract_text TEXT NOT NULL,
            author_id TEXT NOT NULL,
            date TEXT NOT NULL,
            version TEXT NOT NULL,
            thumbnail_path TEXT NOT NULL,
            folder_path TEXT NOT NULL DEFAULT '',
            stars INTEGER NOT NULL DEFAULT 0,
            watching INTEGER NOT NULL DEFAULT 0,
            downloads INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (author_id) REFERENCES accounts(id)
        );

        INSERT INTO accounts VALUES (
            'test-user', 'testuser', 'test-password',
            'Test User', '', '[]'
        );
        ",
    )
    .unwrap();

    let item = Item {
        id: "test-item".into(),
        title: "Test Research".into(),
        abstract_text: "Testing database insertion.".into(),
        author_id: "test-user".into(),
        date: "2026-10-17".into(),
        version: "1.0".into(),
        thumbnail_path: "thumbnail.png".into(),
        folder_path: "uploads/test-item".into(),
        stars: 0,
        watching: 0,
        downloads: 0,
    };

    insert_item(&conn, &item).unwrap();

    let (saved_title, saved_folder): (String, String) = conn
        .query_row(
            "SELECT title, folder_path FROM items WHERE id = ?1",
            [&item.id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();

    assert_eq!(saved_title, "Test Research");
    assert_eq!(saved_folder, "uploads/test-item");
}

#[test]
fn login_account_test() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute_batch(
        "
        CREATE TABLE accounts (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            display_name TEXT NOT NULL,
            profile_img_path TEXT NOT NULL,
            credentials TEXT NOT NULL
        );

        INSERT INTO accounts VALUES (
            'user-001', 'testuser', 'testpass',
            'Test User', '', '[]'
        );
        ",
    )
    .unwrap();

    let result = login_account(&conn, "testuser", "testpass").unwrap();
    assert_eq!(result, Some("user-001".to_string()));

    let result = login_account(&conn, "wronguser", "testpass").unwrap();
    assert_eq!(result, None);
}

#[test]
fn get_items_by_author_test() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute_batch(
        "
        CREATE TABLE items (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            abstract_text TEXT NOT NULL,
            author_id TEXT NOT NULL,
            date TEXT NOT NULL,
            version TEXT NOT NULL,
            thumbnail_path TEXT NOT NULL,
            folder_path TEXT NOT NULL DEFAULT '',
            stars INTEGER NOT NULL DEFAULT 0,
            watching INTEGER NOT NULL DEFAULT 0,
            downloads INTEGER NOT NULL DEFAULT 0
        );

        INSERT INTO items VALUES (
            'item-001', 'Research A', 'First item',
            'author-001', '2026-10-16', '1.0', '', '',
            0, 0, 0
        );

        INSERT INTO items VALUES (
            'item-002', 'Research B', 'Second item',
            'author-002', '2026-10-17', '1.0', '', '',
            0, 0, 0
        );

        INSERT INTO items VALUES (
            'item-003', 'Research C', 'Third item',
            'author-001', '2026-10-17', '1.0', '', '',
            0, 0, 0
        );
        ",
    )
    .unwrap();

    let items = get_items_by_author(&conn, "author-001").unwrap();

    // Only the two items belonging to author-001 should be returned.
    assert_eq!(items.len(), 2);

    // Newer dates should appear first.
    assert_eq!(items[0].id, "item-003");
    assert_eq!(items[1].id, "item-001");

    // Every returned item should belong to the requested author.
    assert!(items.iter().all(|item| item.author_id == "author-001"));

    // Statistics should default to zero.
    assert_eq!(items[0].stars, 0);
    assert_eq!(items[0].watching, 0);
    assert_eq!(items[0].downloads, 0);
}

#[test]
fn seed_test_data_test() {
    let conn = Connection::open_in_memory().unwrap();

    create_tables(&conn).unwrap();
    seed_test_data(&conn).unwrap();

    let items = get_all_items(&conn).unwrap();

    assert!(items.iter().any(|item| item.id == "item-001"));
    assert!(items.iter().any(|item| item.id == "item-002"));

    println!("Seed data inserted successfully!");
}

#[test]
fn get_pending_proposals_for_owner_test() {
    let conn = Connection::open_in_memory().unwrap();

    // Create the tables required by the query.
    conn.execute_batch(
        "
        CREATE TABLE accounts ( 
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            display_name TEXT NOT NULL,
            profile_img_path TEXT NOT NULL,
            credentials TEXT NOT NULL
        );

        CREATE TABLE items (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            abstract_text TEXT NOT NULL,
            author_id TEXT NOT NULL,
            date TEXT NOT NULL,
            version TEXT NOT NULL,
            thumbnail_path TEXT NOT NULL,
            folder_path TEXT NOT NULL DEFAULT ''
        );

        CREATE TABLE proposals (
            id TEXT PRIMARY KEY,
            item_id TEXT NOT NULL,
            proposer_id TEXT NOT NULL,
            replacement_folder_path TEXT NOT NULL,
            note TEXT NOT NULL,
            status TEXT NOT NULL
                CHECK (status IN ('Pending', 'Accepted', 'Rejected')),
            date TEXT NOT NULL,
            FOREIGN KEY (item_id) REFERENCES items(id),
            FOREIGN KEY (proposer_id) REFERENCES accounts(id)
        );

        INSERT INTO accounts VALUES
            ('test-001', 'owner', 'pass', 'Owner', 'owner.jpg', '[]'),
            ('test-002', 'contributor', 'pass', 'Contributor', 'contributor.jpg', '[]');
            
        INSERT INTO items VALUES
            ('item-001', 'Owner Item', 'Description', 'test-001',
             '2026-10-16', '1.0', 'thumbnail.jpg', 'Library/item-001/'),
            ('item-002', 'Other Item', 'Description', 'test-002',
             '2026-10-17', '1.0', 'other.jpg', 'Library/item-002/');

        INSERT INTO proposals (
            id,
            item_id,
            proposer_id,
            replacement_folder_path,
            note,
            status,
            date
        ) VALUES
            (
                'proposal-001',
                'item-001',
                'test-002',
                'Submissions/proposal-001/',
                'Improve the example.',
                'Pending',
                '2026-10-17'
            ),
            (
                'proposal-002',
                'item-001',
                'test-002',
                'Submissions/proposal-002/',
                'Already accepted.',
                'Accepted',
                '2026-10-16'
            ),
            (
                'proposal-003',
                'item-002',
                'test-001',
                'Submissions/proposal-003/',
                'Belongs to another owner.',
                'Pending',
                '2026-10-18'
            );
        ",
    )
    .unwrap();

    let proposals = get_pending_proposals_for_owner(&conn, "test-001").unwrap();

    assert_eq!(proposals.len(), 1);
    assert_eq!(proposals[0].id, "proposal-001");
    assert_eq!(proposals[0].item_title, "Owner Item");
    assert_eq!(proposals[0].proposer_id, "test-002");
    assert_eq!(proposals[0].proposer_name, "Contributor");
    assert_eq!(proposals[0].note, "Improve the example.");

    // An account with no owned items should get no proposals.
    let proposals = get_pending_proposals_for_owner(&conn, "unknown-user").unwrap();

    assert!(proposals.is_empty());
}

#[test]
fn update_proposal_status_test() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute_batch(
        "
        CREATE TABLE accounts (
            id TEXT PRIMARY KEY
        );

        CREATE TABLE items (
            id TEXT PRIMARY KEY,
            author_id TEXT NOT NULL
        );

        CREATE TABLE proposals (
            id TEXT PRIMARY KEY,
            item_id TEXT NOT NULL,
            proposer_id TEXT NOT NULL,
            status TEXT NOT NULL
                CHECK (status IN ('Pending', 'Accepted', 'Rejected'))
        );

        INSERT INTO accounts VALUES ('owner-001');
        INSERT INTO accounts VALUES ('owner-002');

        INSERT INTO items VALUES ('item-001', 'owner-001');

        INSERT INTO proposals VALUES (
            'proposal-001', 'item-001', 'owner-002', 'Pending'
        );

        INSERT INTO proposals VALUES (
            'proposal-002', 'item-001', 'owner-002', 'Accepted'
        );
        ",
    )
    .unwrap();

    // The owner can accept a pending proposal.
    let updated = update_proposal_status(
        &conn,
        "proposal-001",
        "owner-001",
        "Accepted",
    )
    .unwrap();

    assert!(updated);

    let status: String = conn
        .query_row(
            "SELECT status FROM proposals WHERE id = ?1",
            ["proposal-001"],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(status, "Accepted");

    // A second decision must fail because it is no longer pending.
    assert!(
        !update_proposal_status(
            &conn,
            "proposal-001",
            "owner-001",
            "Rejected",
        )
        .unwrap()
    );

    // Another account cannot decide the owner's proposal.
    conn.execute(
        "INSERT INTO proposals VALUES (
            'proposal-003', 'item-001', 'owner-002', 'Pending'
        )",
        [],
    )
    .unwrap();

    assert!(
        !update_proposal_status(
            &conn,
            "proposal-003",
            "owner-002",
            "Accepted",
        )
        .unwrap()
    );

    // Invalid status values are rejected.
    assert!(
        !update_proposal_status(
            &conn,
            "proposal-003",
            "owner-001",
            "SomethingElse",
        )
        .unwrap()
    );

    // Rejecting a pending proposal also works.
    assert!(
        update_proposal_status(
            &conn,
            "proposal-003",
            "owner-001",
            "Rejected",
        )
        .unwrap()
    );
}

#[test]
fn get_profile_counts_test() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;

        // Create the tables required by get_profile_counts().
        conn.execute_batch(
        "
        CREATE TABLE accounts (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            display_name TEXT NOT NULL,
            profile_img_path TEXT NOT NULL,
            credentials TEXT NOT NULL
        );

        CREATE TABLE items (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            abstract_text TEXT NOT NULL,
            author_id TEXT NOT NULL,
            date TEXT NOT NULL,
            version TEXT NOT NULL,
            thumbnail_path TEXT NOT NULL,
            folder_path TEXT NOT NULL DEFAULT ''
        );

        CREATE TABLE proposals (
            id TEXT PRIMARY KEY,
            item_id TEXT NOT NULL,
            proposer_id TEXT NOT NULL,
            replacement_folder_path TEXT NOT NULL,
            note TEXT NOT NULL,
            status TEXT NOT NULL
                CHECK (status IN ('Pending', 'Accepted', 'Rejected')),
            date TEXT NOT NULL,
            FOREIGN KEY (item_id) REFERENCES items(id),
            FOREIGN KEY (proposer_id) REFERENCES accounts(id)
        );

        CREATE TABLE item_watches (
            item_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            PRIMARY KEY (item_id, user_id)
        );

        INSERT INTO accounts VALUES
            ('test-001', 'owner', 'pass', 'Owner', 'owner.jpg', '[]'),
            ('test-002', 'contributor', 'pass', 'Contributor', 'contributor.jpg', '[]');

        INSERT INTO items VALUES
            ('item-001', 'Owner Item', 'Description', 'test-001',
             '2026-10-16', '1.0', 'thumbnail.jpg', 'Library/item-001/'),
            ('item-002', 'Other Item', 'Description', 'test-002',
             '2026-10-17', '1.0', 'other.jpg', 'Library/item-002/');

        INSERT INTO proposals (
            id, item_id, proposer_id, replacement_folder_path,
            note, status, date
        ) VALUES
            ('proposal-001', 'item-001', 'test-002',
             'Submissions/proposal-001/', 'Improve the example.',
             'Pending', '2026-10-17'),
            ('proposal-002', 'item-001', 'test-002',
             'Submissions/proposal-002/', 'Already accepted.',
             'Accepted', '2026-10-16'),
            ('proposal-003', 'item-002', 'test-001',
             'Submissions/proposal-003/', 'Belongs to another owner.',
             'Pending', '2026-10-18');

        INSERT INTO item_watches VALUES
            ('item-001', 'test-002');
        ",
    )?;

    // test-001 owns item-001, which has one pending proposal.
    let counts = get_profile_counts(&conn, "test-001")?;

    assert_eq!(counts.pending_count, 1);
    assert_eq!(counts.watching_count, 0);
    assert_eq!(counts.outgoing_count, 1);

    // test-002 owns item-002, watches item-001,
    // and has one pending outgoing proposal.
    let counts = get_profile_counts(&conn, "test-002")?;

    assert_eq!(counts.pending_count, 1);
    assert_eq!(counts.watching_count, 1);
    assert_eq!(counts.outgoing_count, 1);

    // An unknown account should have zero counts.
    let counts = get_profile_counts(&conn, "unknown-user")?;

    assert_eq!(counts.pending_count, 0);
    assert_eq!(counts.watching_count, 0);
    assert_eq!(counts.outgoing_count, 0);

    Ok(())
}
