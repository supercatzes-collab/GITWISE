//database.rs
use crate::components::shared_data_components::Item;
use rusqlite::{params, Connection, OptionalExtension, Result};

#[derive(Clone, Debug)]
pub struct AccountProfile {
    pub id: String,
    pub display_name: String,
    pub profile_img_path: String,
    pub credentials: String,
}

#[derive(Clone, Debug, Default)]
pub struct ProfileCounts {
    pub pending_count: u32,
    pub watching_count: u32,
    pub outgoing_count: u32,
}


#[derive(Clone, Debug)]
pub struct PendingProposal {
    pub id: String,
    pub item_id: String,
    pub item_title: String,
    pub proposer_id: String,
    pub proposer_name: String,
    pub proposer_img_path: String,
    pub item_thumbnail_path: String,
    pub original_folder_path: String,
    pub replacement_folder_path: String,
    pub note: String,
    pub date: String,
}


// Initialize the database and create tables if they don't exist.
pub fn initialize_database() -> Result<Connection> {
    let conn = Connection::open("gitwise.db")?;
    create_tables(&conn)?;
    Ok(conn)
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
             CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                display_name TEXT NOT NULL,
                profile_img_path TEXT NOT NULL,
                credentials TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS items (
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

            CREATE TABLE IF NOT EXISTS proposals (
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

            CREATE TABLE IF NOT EXISTS item_contributors (
                item_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                contribution_type TEXT NOT NULL
                    CHECK (contribution_type IN ('version', 'modification')),
                contributed_at TEXT NOT NULL,

                PRIMARY KEY (item_id, user_id),

                FOREIGN KEY (item_id) REFERENCES items(id),
                FOREIGN KEY (user_id) REFERENCES accounts(id)
            );
            CREATE TABLE IF NOT EXISTS item_stars (
                item_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                PRIMARY KEY (item_id, user_id),
                FOREIGN KEY (item_id) REFERENCES items(id),
                FOREIGN KEY (user_id) REFERENCES accounts(id)
            );

            CREATE TABLE IF NOT EXISTS item_watches (
                item_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                PRIMARY KEY (item_id, user_id),
                FOREIGN KEY (item_id) REFERENCES items(id),
                FOREIGN KEY (user_id) REFERENCES accounts(id)
            );
        "
    )?;

    Ok(())
}

// Ensure the demo account exists.
pub fn ensure_default_account(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO accounts (
            id, username, password, display_name,
            profile_img_path, credentials
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            "test-001",
            "testuser",
            "testpass",
            "Alan Turing",
            "assets/img_assets/profile.jpg",
            r#"["Software Engineer", "Mathematician", "Cryptanalyst"]"#
        ],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO accounts (
            id, username, password, display_name,
            profile_img_path, credentials
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            "test-002",
            "contributor",
            "testpass",
            "Test Contributor",
            "assets/img_assets/profile_1.jpg",
            r#"["Developer", "Contributor"]"#
        ],
    )?;

    Ok(())
}

/// Insert predictable development data for testing database queries.
/// Safe to run repeatedly: existing records with these IDs are not replaced.
pub fn seed_test_data(conn: &Connection) -> Result<()> {
    // Ensure test-001 and test-002 exist.
    ensure_default_account(conn)?;

    // Use a transaction so a failure does not leave a partially seeded dataset.
    let tx = conn.unchecked_transaction()?;

    // Test items.
    tx.execute(
        "INSERT OR IGNORE INTO items (
            id, title, abstract_text, author_id, date, version,
            thumbnail_path, folder_path
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            "item-001",
            "Rust Concurrency Toolkit",
            "A sample project for testing proposals and collaboration.",
            "test-001",
            "2026-10-16",
            "1.0",
            "assets/img_assets/profile.jpg",
            "Library/item-001/"
        ],
    )?;

    tx.execute(
        "INSERT OR IGNORE INTO items (
            id, title, abstract_text, author_id, date, version,
            thumbnail_path, folder_path
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            "item-002",
            "Data Structures Playground",
            "A second sample project for testing author filtering.",
            "test-002",
            "2026-10-17",
            "1.0",
            "assets/img_assets/profile_1.jpg",
            "Library/item-002/"
        ],
    )?;

    // Proposals for item-001.
    let proposals = [
        (
            "proposal-001",
            "Submissions/proposal-001/",
            "Improve the concurrency example.",
            "Pending",
            "2026-10-17",
        ),
        (
            "proposal-002",
            "Submissions/proposal-002/",
            "Add documentation and examples.",
            "Accepted",
            "2026-10-16",
        ),
        (
            "proposal-003",
            "Submissions/proposal-003/",
            "Replace the sample implementation.",
            "Rejected",
            "2026-10-15",
        ),
    ];

   
    for (id, replacement_folder_path, note, status, date) in proposals {
        tx.execute(
            "INSERT OR IGNORE INTO proposals (
                id, item_id, proposer_id, replacement_folder_path,
                note, status, date
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                id,
                "item-001",
                "test-002",
                replacement_folder_path,
                note,
                status,
                date,
            ],
        )?;
    }


    // One star and one watch from the contributor.
    tx.execute(
        "INSERT OR IGNORE INTO item_stars (item_id, user_id)
         VALUES (?1, ?2)",
        params!["item-001", "test-002"],
    )?;

    tx.execute(
        "INSERT OR IGNORE INTO item_watches (item_id, user_id)
         VALUES (?1, ?2)",
        params!["item-001", "test-002"],
    )?;

    // Record the contributor's relationship to item-001.
    tx.execute(
        "INSERT OR IGNORE INTO item_contributors (
            item_id, user_id, contribution_type, contributed_at
        ) VALUES (?1, ?2, ?3, ?4)",
        params![
            "item-001",
            "test-002",
            "modification",
            "2026-10-16"
        ],
    )?;

    // Recalculate counts instead of incrementing them.
    tx.execute(
        "UPDATE items SET
            stars = (
                SELECT COUNT(*) FROM item_stars
                WHERE item_id = items.id
            ),
            watching = (
                SELECT COUNT(*) FROM item_watches
                WHERE item_id = items.id
            )",
        [],
    )?;

    tx.commit()?;
    Ok(())
}

#[test]
fn seed_test_data_test() {
    let conn = Connection::open_in_memory().unwrap();

    create_tables(&conn).unwrap();
    seed_test_data(&conn).unwrap();

    let item_count: u32 = conn
        .query_row(
            "SELECT COUNT(*) FROM items
             WHERE id IN ('item-001', 'item-002')",
            [],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(item_count, 2);

    let proposal_count: u32 = conn
        .query_row(
            "SELECT COUNT(*) FROM proposals
             WHERE id IN (
                 'proposal-001',
                 'proposal-002',
                 'proposal-003'
             )",
            [],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(proposal_count, 3);
}


// Find an account matching the supplied username and password.
pub fn login_account(
    conn: &Connection,
    username: &str,
    password: &str,
) -> Result<Option<String>> {
    let account_id = conn
        .query_row(
            "SELECT id FROM accounts
             WHERE username = ?1 AND password = ?2",
            params![username, password],
            |row| row.get::<_, String>(0),
        )
        .optional()?;

    Ok(account_id)
}

pub fn get_account_display_name(
    conn: &Connection,
    account_id: &str,
) -> Result<Option<String>> {
    conn.query_row(
        "SELECT display_name FROM accounts WHERE id = ?1",
        [account_id],
        |row| row.get(0),
    )
    .optional()
}

// Insert a research item into the database.
pub fn insert_item(conn: &Connection, item: &Item) -> Result<()> {
    conn.execute(
        "
        INSERT INTO items (
            id, title, abstract_text, author_id,
            date, version, thumbnail_path, folder_path
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        ",
        params![
            &item.id,
            &item.title,
            &item.abstract_text,
            &item.author_id,
            &item.date,
            &item.version,
            &item.thumbnail_path,
            &item.folder_path,
        ],
    )?;

    Ok(())
}

pub fn get_account_profile(
    conn: &Connection,
    account_id: &str,
) -> Result<Option<AccountProfile>> {
    let profile = conn
        .query_row(
            "SELECT id, display_name, profile_img_path, credentials
             FROM accounts
             WHERE id = ?1",
            [account_id],
            |row| {
                Ok(AccountProfile {
                    id: row.get(0)?,
                    display_name: row.get(1)?,
                    profile_img_path: row.get(2)?,
                    credentials: row.get(3)?,
                })
            },
        )
        .optional()?;

    Ok(profile)
}

pub fn get_profile_counts(
    conn: &Connection,
    account_id: &str,
) -> Result<ProfileCounts> {
    conn.query_row(
        "
        SELECT
            (
                SELECT COUNT(*)
                FROM proposals p
                JOIN items i ON i.id = p.item_id
                WHERE i.author_id = ?1
                  AND p.status = 'Pending'
            ),
            (
                SELECT COUNT(*)
                FROM item_watches
                WHERE user_id = ?1
            ),
            (
                SELECT COUNT(*)
                FROM proposals
                WHERE proposer_id = ?1
                  AND status = 'Pending'
            )
        ",
        [account_id],
        |row| {
            Ok(ProfileCounts {
                pending_count: row.get(0)?,
                watching_count: row.get(1)?,
                outgoing_count: row.get(2)?,
            })
        },
    )
}

// Retrieve pending proposals for items owned by a specific account.
pub fn get_pending_proposals_for_owner(
    conn: &Connection,
    owner_id: &str,
) -> Result<Vec<PendingProposal>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            p.id,
            p.item_id,
            i.title,
            p.proposer_id,
            a.display_name,
            a.profile_img_path,
            i.thumbnail_path,
            i.folder_path,
            p.replacement_folder_path,
            p.note,
            p.date
        FROM proposals p
        JOIN items i ON i.id = p.item_id
        JOIN accounts a ON a.id = p.proposer_id
        WHERE i.author_id = ?1
          AND p.status = 'Pending'
        ORDER BY p.date DESC
        ",
    )?;

    let proposals = stmt
        .query_map([owner_id], |row| {
            Ok(PendingProposal {
                id: row.get(0)?,
                item_id: row.get(1)?,
                item_title: row.get(2)?,
                proposer_id: row.get(3)?,
                proposer_name: row.get(4)?,
                proposer_img_path: row.get(5)?,
                item_thumbnail_path: row.get(6)?,
                original_folder_path: row.get(7)?,
                replacement_folder_path: row.get(8)?,
                note: row.get(9)?,
                date: row.get(10)?,
            })
        })?
        .collect::<Result<Vec<PendingProposal>>>()?;

    Ok(proposals)
    
}


pub fn get_pending_proposal_by_id(
    conn: &Connection,
    proposal_id: &str,
    owner_id: &str,
) -> Result<Option<PendingProposal>> {
    conn.query_row(
        "
        SELECT
            p.id,
            p.item_id,
            i.title,
            p.proposer_id,
            a.display_name,
            a.profile_img_path,
            i.thumbnail_path,
            i.folder_path,
            p.replacement_folder_path,
            p.note,
            p.date
        FROM proposals p
        JOIN items i ON i.id = p.item_id
        JOIN accounts a ON a.id = p.proposer_id
        WHERE p.id = ?1
          AND i.author_id = ?2
          AND p.status = 'Pending'
        ",
        params![proposal_id, owner_id],
        |row| {
            Ok(PendingProposal {
                id: row.get(0)?,
                item_id: row.get(1)?,
                item_title: row.get(2)?,
                proposer_id: row.get(3)?,
                proposer_name: row.get(4)?,
                proposer_img_path: row.get(5)?,
                item_thumbnail_path: row.get(6)?,
                original_folder_path: row.get(7)?,
                replacement_folder_path: row.get(8)?,
                note: row.get(9)?,
                date: row.get(10)?,
            })
        },
    )
    .optional()
}



// Accept or reject a pending proposal, provided the caller owns the item.
// Returns true if the proposal was updated, false otherwise.
pub fn update_proposal_status(
    conn: &Connection,
    proposal_id: &str,
    owner_id: &str,
    new_status: &str,
) -> Result<bool> {
    // Only these two status transitions are allowed through this function.
    if new_status != "Accepted" && new_status != "Rejected" {
        return Ok(false);
    }

    let rows_updated = conn.execute(
        "
        UPDATE proposals
        SET status = ?1
        WHERE id = ?2
          AND status = 'Pending'
          AND EXISTS (
              SELECT 1
              FROM items
              WHERE items.id = proposals.item_id
                AND items.author_id = ?3
          )
        ",
        params![new_status, proposal_id, owner_id],
    )?;

    Ok(rows_updated == 1)
}

// Retrieve all items belonging to a specific author.
pub fn get_items_by_author(
    conn: &Connection,
    author_id: &str,
) -> Result<Vec<Item>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, abstract_text, author_id,
        date, version, thumbnail_path, folder_path,
        stars, watching, downloads
        FROM items
        WHERE author_id = ?1
        ORDER BY date DESC",
    )?;

    let items = stmt
        .query_map([author_id], |row| {
            Ok(Item {
                id: row.get(0)?,
                title: row.get(1)?,
                abstract_text: row.get(2)?,
                author_id: row.get(3)?,
                date: row.get(4)?,
                version: row.get(5)?,
                thumbnail_path: row.get(6)?,
                folder_path: row.get(7)?,
                stars: row.get(8)?,
                watching: row.get(9)?,
                downloads: row.get(10)?,
            })
        })?
        .collect::<Result<Vec<Item>>>()?;

    Ok(items)
}

// Retrieve all items from the database.
pub fn get_all_items(conn: &Connection) -> Result<Vec<Item>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, abstract_text, author_id, date, version,
        thumbnail_path, folder_path, stars, watching, downloads
        FROM items
        ORDER BY date DESC",
    )?;

    let items = stmt
        .query_map([], |row| {
            Ok(Item {
                id: row.get(0)?,
                title: row.get(1)?,
                abstract_text: row.get(2)?,
                author_id: row.get(3)?,
                date: row.get(4)?,
                version: row.get(5)?,
                thumbnail_path: row.get(6)?,
                folder_path: row.get(7)?,
                stars: row.get(8)?,
                watching: row.get(9)?,
                downloads: row.get(10)?,
            })
        })?
        .collect::<Result<Vec<Item>>>()?;

    Ok(items)
}

// Retrieve one item using its ID.
// Returns None if no item with that ID exists.
pub fn get_item_by_id(
    conn: &Connection,
    item_id: &str,
) -> Result<Option<Item>> {
    conn.query_row(
       "SELECT id, title, abstract_text, author_id,
        date, version, thumbnail_path, folder_path,
        stars, watching, downloads
        FROM items
        WHERE id = ?1",
        [item_id],
        |row| {
            Ok(Item {
                id: row.get(0)?,
                title: row.get(1)?,
                abstract_text: row.get(2)?,
                author_id: row.get(3)?,
                date: row.get(4)?,
                version: row.get(5)?,
                thumbnail_path: row.get(6)?,
                folder_path: row.get(7)?,
                stars: row.get(8)?,
                watching: row.get(9)?,
                downloads: row.get(10)?,
            })
        },
    )
    .optional()
}

pub fn increment_item_downloads(
    conn: &Connection,
    item_id: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE items
         SET downloads = downloads + 1
         WHERE id = ?1",
        [item_id],
    )?;

    Ok(())
}

pub fn toggle_item_star(
        conn: &Connection,
        item_id: &str,
        user_id: &str,
    ) -> Result<()> {
        let already_starred: bool = conn.query_row(
            "SELECT EXISTS(
                SELECT 1 FROM item_stars
                WHERE item_id = ?1 AND user_id = ?2
            )",
            rusqlite::params![item_id, user_id],
            |row| row.get(0),
        )?;

        if already_starred {
            conn.execute(
                "DELETE FROM item_stars
                 WHERE item_id = ?1 AND user_id = ?2",
                rusqlite::params![item_id, user_id],
            )?;
        } else {
            conn.execute(
                "INSERT INTO item_stars (item_id, user_id)
                 VALUES (?1, ?2)",
                rusqlite::params![item_id, user_id],
            )?;
        }

        conn.execute(
            "UPDATE items
             SET stars = (
                 SELECT COUNT(*) FROM item_stars
                 WHERE item_id = ?1
             )
             WHERE id = ?1",
            [item_id],
        )?;

        Ok(())
    }

    pub fn toggle_item_watch(
        conn: &Connection,
        item_id: &str,
        user_id: &str,
    ) -> Result<()> {
        let already_watching: bool = conn.query_row(
            "SELECT EXISTS(
                SELECT 1 FROM item_watches
                WHERE item_id = ?1 AND user_id = ?2
            )",
            rusqlite::params![item_id, user_id],
            |row| row.get(0),
        )?;

        if already_watching {
            conn.execute(
                "DELETE FROM item_watches
                 WHERE item_id = ?1 AND user_id = ?2",
                rusqlite::params![item_id, user_id],
            )?;
        } else {
            conn.execute(
                "INSERT INTO item_watches (item_id, user_id)
                 VALUES (?1, ?2)",
                rusqlite::params![item_id, user_id],
            )?;
        }

        conn.execute(
            "UPDATE items
             SET watching = (
                 SELECT COUNT(*) FROM item_watches
                 WHERE item_id = ?1
             )
             WHERE id = ?1",
            [item_id],
        )?;

        Ok(())
    }

#[cfg(test)]
#[path = "database_tests/mod.rs"]
mod tests;