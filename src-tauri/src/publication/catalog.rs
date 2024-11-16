use std::{fs, io::{self, Cursor, Read}, path::PathBuf};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use crate::utils::unpack_zip;

use super::Manifest;

pub struct Catalog {
    pub_path: PathBuf,
    catalog_db: Connection,
}

#[derive(Serialize, Deserialize)]
pub struct PublicationDisplay {
    pub id: i32,
    
    /// File path name for Catalog
    pub name: String,
    pub symbol: String,

    pub hash: String,
    pub timestamp: String,
    pub language_idx: i32, // Can be ignored, since we don't have all language indices
    pub year: i32,
    
    pub title: String,
    pub short_title: String,
    
    /// Used for sorting, since first came last
    pub issue_number: i32,
    pub issue_id: i32,
    pub issue_title: String,
    pub issue_cover_title: String,
    
    pub icon_path: String,
    pub categories: Vec<String>,
    pub attributes: Vec<String>,
}

impl Catalog {
    pub fn init<T: Into<PathBuf>>(location: T) -> Result<Self, Box<dyn std::error::Error>> {
        let location = location.into();
        if !location.exists() {
            fs_extra::dir::create_all(&location, false)?;
        }

        let db = Connection::open(location.join("catalog.db"))?;
        db.execute(
        "CREATE TABLE IF NOT EXISTS publications (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                
                name TEXT NOT NULL,
                symbol TEXT NOT NULL,
                symbol TEXT NOT NULL,
                hash TEXT NOT NULL,
                timestamp DATETIME NOT NULL,
                language_idx INTEGER NOT NULL,
                year INTEGER NOT NULL,

                title TEXT NOT NULL,
                short_title TEXT NOT NULL,

                issue_number INTEGER DEFAULT 0,
                issue_id INTEGER DEFAULT 0,
                issue_title TEXT,
                issue_cover_title TEXT,
                
                icon_path TEXT,
                categories JSON NOT NULL DEFAULT('[]'),
                attributes JSON NOT NULL DEFAULT('[]'),
            )", 
            ()
        )?;

        Ok(Self { 
            pub_path: location, 
            catalog_db: db 
        })
    }

    pub fn insert_publication_to_catalog(
        &self,
        name: &str,
        symbol: &str,
        hash: &str,
        timestamp: &str,
        language_idx: i32,
        year: i32,
        title: &str,
        short_title: &str,
        issue_number: i32,
        issue_id: i32,
        issue_title: &str,
        issue_cover_title: &str,
        icon_path: &str,
        categories: &Vec<String>,
        attributes: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.catalog_db.execute(
            "INSERT INTO publications (
                name, symbol, hash, timestamp, language_idx, year, title, short_title, issue_number, issue_id, issue_title, issue_cover_title, icon_path, categories, attributes
                name, symbol, hash, timestamp, language_idx, year, title, short_title, issue_number, issue_id, issue_title, issue_cover_title, icon_path, categories, attributes
            ) VALUES (
                ?,?,?,?,?,?,?,?,?,?,?,?,?,?,?
                ?,?,?,?,?,?,?,?,?,?,?,?,?,?,?
            )",
            (
                name, symbol, hash, timestamp, language_idx, year, title, short_title, issue_number, issue_id, issue_title, issue_cover_title, icon_path, serde_json::to_value(categories)?, serde_json::to_value(attributes)?,
            ),
        )?;

        Ok(())
    }
    
    pub fn install_jwpub_file<T: Into<PathBuf>>(&self, file_path: T) -> Result<(), Box<dyn std::error::Error>> {
        // Check manifest for valid .JWPUB;
        let file_path: PathBuf = file_path.into();
        let package = fs::File::open(file_path)?;
        let reader = io::BufReader::new(package);
        let mut package = ZipArchive::new(reader)?;

        let manifest = get_metadata_from_archive(&mut package)?;
        let pub_pathname = manifest.name.replace(".jwpub", "");
        
        let location = self.pub_path.join(pub_pathname);
        if !location.exists() {
            fs_extra::dir::create_all(&location, false)?;
        }

        let mut content_file = package.by_name("contents")?;
        let mut content_data = Vec::<u8>::new();
        content_file.read_to_end(&mut content_data)?;
        let content_package = ZipArchive::new(Cursor::new(content_data))?;

        unpack_zip(content_package, &location);
        Ok(())
    }

    pub fn get_list_from_category(&self, category: &str) -> Result<Vec<PublicationDisplay>, Box<dyn std::error::Error>> {
        let mut stmt = self.catalog_db.prepare("SELECT * FROM publications WHERE categories LIKE ?")?;
        let mut rows = stmt.query([format!("%{}%", category)])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(PublicationDisplay {
                id: row.get(0)?,
                name: row.get(1)?,
                symbol: row.get(2)?,
                hash: row.get(3)?,
                timestamp: row.get(4)?,
                language_idx: row.get(5)?,
                year: row.get(6)?,
                title: row.get(7)?,
                short_title: row.get(8)?,
                issue_number: row.get(9)?,
                issue_id: row.get(10)?,
                issue_title: row.get(11)?,
                issue_cover_title: row.get(12)?,
                icon_path: row.get(13)?,
                categories: serde_json::from_value(row.get(14)?)?,
                attributes: serde_json::from_value(row.get(15)?)?,
            })
        }

        Ok(result)
    }
}

pub fn get_metadata_from_archive(pub_archive: &mut ZipArchive<io::BufReader<fs::File>>) -> Result<Manifest, serde_json::Error> {
    let mut manifest_file = pub_archive.by_name("manifest.json").unwrap();
    let mut manifest_data = String::new();
    manifest_file.read_to_string(&mut manifest_data).unwrap();
    serde_json::from_str(manifest_data.as_str())
}