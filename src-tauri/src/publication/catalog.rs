use std::{fs, io::{self, Cursor, Read}, path::PathBuf};

use rusqlite::Connection;
use zip::ZipArchive;

use crate::utils::unpack_zip;

use super::Manifest;

pub struct Catalog {
    pub_path: PathBuf,
    catalog_db: Connection,
}

pub struct PublicationDisplay {
    pub id: i32,
    
    /// File path name for Catalog
    pub name: String,

    pub hash: String,
    pub timestamp: String,
    pub language_idx: i32, // Can be ignored, since we don't have all language indices
    pub year: i32,
    
    pub title: String,
    pub short_title: String,
    
    /// Used for sorting, since first came last
    pub issue_number: i32,
    pub issue_id: i32,
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
                id INTEGER PRIMARY KEY,
                
                name TEXT NOT NULL,
                hash TEXT NOT NULL,
                timestamp DATETIME NOT NULL,
                language_idx INTEGER NOT NULL,
                year INTEGER NOT NULL,

                title TEXT NOT NULL,
                short_title TEXT NOT NULL,

                issue_number INTEGER DEFAULT 0,
                issue_id INTEGER DEFAULT 0,
                issue_cover_title TEXT
                
                icon_path TEXT
                categories TEXT NOT NULL
                attributes TEXT NOT NULL
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
        id: i32,
        name: &str,
        hash: &str,
        timestamp: &str,
        language_idx: i32,
        year: i32,
        title: &str,
        short_title: &str,
        issue_number: i32,
        issue_id: i32,
        issue_cover_title: &str,
        icon_path: &str,
        categories: Vec<&str>,
        attributes: Vec<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.catalog_db.execute(
            "INSERT INTO publications (
                id, name, hash, timestamp, language_idx, year, title, short_title, issue_number, issue_id, issue_cover_title, icon_path, categories, attributes
            ) VALUES (
                ?,?,?,?,?,?,?,?,?,?,?,?,?,?
            )",
            (
                id, name, hash, timestamp, language_idx, year, title, short_title, issue_number, issue_id, issue_cover_title, icon_path, categories.join(","), attributes.join(","),
            ),
        )?;

        Ok(())
    }
    
    pub fn install_jwpub_file<T: Into<PathBuf>>(&self, file_path: T) -> Result<(), Box<dyn std::error::Error>> {
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
}

pub fn get_metadata_from_archive(pub_archive: &mut ZipArchive<io::BufReader<fs::File>>) -> Result<Manifest, serde_json::Error> {
    let mut manifest_file = pub_archive.by_name("manifest.json").unwrap();
    let mut manifest_data = String::new();
    manifest_file.read_to_string(&mut manifest_data).unwrap();
    serde_json::from_str(manifest_data.as_str())
}