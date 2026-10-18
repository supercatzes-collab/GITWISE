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

// Initialize the database and create tables if they don't exist.
pub fn initialize_database() -> Result<Connection> {
    let conn = Connection::open("gitwise.db")?;

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
            target_file_path TEXT NOT NULL,
            proposer_id TEXT NOT NULL,
            replacement_path TEXT NOT NULL,
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
        ",

    )?;

    Ok(conn)
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
mod tests {
    use super::*;

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

        let result =
            login_account(&conn, "testuser", "testpass").unwrap();
        assert_eq!(result, Some("user-001".to_string()));

        let result =
            login_account(&conn, "wronguser", "testpass").unwrap();
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
    assert!(
        items
            .iter()
            .all(|item| item.author_id == "author-001")
    );

    // New statistics should default to zero.
    assert_eq!(items[0].stars, 0);
    assert_eq!(items[0].watching, 0);
    assert_eq!(items[0].downloads, 0);
    }
}
