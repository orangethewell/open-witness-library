use std::path::PathBuf;

use rusqlite::Connection;

use super::tables::*;

pub struct Publication {
    db: Connection
}

impl Publication {
    pub fn from_path(pub_path: PathBuf) -> Self {
        let db = Connection::open(pub_path.join("")).unwrap();
        Self {
            db
        }
    }
}

