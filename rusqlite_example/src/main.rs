use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // Connect to or create a SQLite database file on the local machine
    let conn = Connection::open("local_data.db")?;

    // Create a table in the database
    conn.execute(
        "CREATE TABLE IF NOT EXISTS data (id INTEGER PRIMARY KEY, value TEXT)",
        params![],
    )?;

    // Insert data into the table
    conn.execute("INSERT INTO data (value) VALUES (?)", params!["This is some local data"])?;

    // Save the changes to the database
    conn.commit()?;

    Ok(())
}
