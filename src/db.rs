use std::collections::HashMap;

use crate::{
    errors::DatabaseError,
    message::{self, Message},
};
use sqlite::{Connection, State};

pub struct DatabaseHandler {
    pub conn: Connection,
}

impl DatabaseHandler {
    pub fn new() -> Result<Self, DatabaseError> {
        let conn = Connection::open("./database.db").map_err(|_| DatabaseError::ConnectionError)?;
        Self::initialize_tables(&conn)?;

        // Clear the messages table on every start
        conn.execute("DELETE FROM messages;")
            .map_err(|_| DatabaseError::TableError)?;

        Ok(DatabaseHandler { conn })
    }

    pub fn initialize_tables(conn: &Connection) -> Result<(), DatabaseError> {
        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                hour TEXT NOT NULL,
                owner TEXT NOT NULL,
                text TEXT NOT NULL
            );
            ",
        )
        .map_err(|_| DatabaseError::TableError)?;

        Ok(())
    }

    pub fn insert_messages(&self, messages: &[Message]) -> Result<(), DatabaseError> {
        self.conn
            .execute("BEGIN TRANSACTION;")
            .map_err(|_| DatabaseError::TableError)?;

        for message in messages {
            self.conn
                .execute(format!(
            "INSERT INTO messages (date, hour, owner, text) VALUES ('{}', '{}', '{}', '{}');",
            message.date.replace("'", "''"),
            message.hour.replace("'", "''"),
            message.owner.replace("'", "''"),
            message.text.replace("'", "''")
        ))
                .map_err(|_| DatabaseError::TableError)?;
        }

        self.conn
            .execute("COMMIT;")
            .map_err(|_| DatabaseError::TableError)?;

        Ok(())
    }

    pub fn get_messages_count(&self) -> Result<HashMap<String, i64>, DatabaseError> {
        let mut count_hashmap: HashMap<String, i64> = HashMap::new();
        let query = "SELECT owner, COUNT(*) as message_count FROM messages GROUP BY owner order by message_count DESC;";
        let mut statement = self
            .conn
            .prepare(query)
            .map_err(|_| DatabaseError::TableError)?;

        while let Ok(State::Row) = statement.next() {
            let owner: String = statement.read("owner").unwrap();
            let count: i64 = statement.read("message_count").unwrap();
            count_hashmap.insert(owner, count);
        }
        Ok(count_hashmap)
    }

}
